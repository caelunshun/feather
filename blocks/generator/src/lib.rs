use crate::load::ident;
use heck::CamelCase;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use serde::ser::{SerializeSeq, SerializeStruct};
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;
use std::ops::RangeInclusive;
use std::str::FromStr;
use syn::export::ToTokens;

mod load;

#[derive(Debug)]
struct Blocks {
    property_types: BTreeMap<String, Property>,
    blocks: Vec<Block>,
}

#[derive(Debug)]
pub struct Block {
    /// Lowercase name of this block, minecraft: prefix removed.
    name: Ident,
    /// `name.to_camel_case()`
    name_camel_case: Ident,
    /// This block's properties.
    properties: Vec<String>,
    /// Default state and its property values.
    default_state: Vec<(String, String)>,
    /// Block states mapped to vanilla state IDs.
    ids: Vec<(Vec<(String, String)>, u16)>,
    /// Strides and offset coefficients for each property of this block.
    index_parameters: BTreeMap<String, (u16, u16)>,
}

#[derive(Debug)]
struct Property {
    /// Name of this property, with Rust keywords removed. (e.g. "type" => "kind")
    name: Ident,
    /// CamelCase name of this property if it were a struct or enum.
    ///
    /// Often prefixed with the name of the block to which this property belongs.
    name_camel_case: Ident,
    /// The kind of this property.
    kind: PropertyKind,
    /// Possible values of this property.
    possible_values: Vec<String>,
}

impl Property {
    /// Returns the tokens to create an instance of this property from a `u16`.
    fn tokens_for_from_u16(&self, input: TokenStream) -> TokenStream {
        match &self.kind {
            PropertyKind::Integer { .. } => quote! {{ #input as i32 }},
            PropertyKind::Boolean { .. } => quote! { if #input == 0 { false } else { true } },
            PropertyKind::Enum { name, .. } => {
                quote! { #name::try_from(#input).expect("invalid block state") }
            }
        }
    }

    fn tokens_for_to_u16(&self, input: TokenStream) -> TokenStream {
        match &self.kind {
            PropertyKind::Integer { range } => {
                let min = *range.start() as u16;
                quote! {
                    #input as u16 - #min
                }
            }
            _ => quote! { #input as u16 },
        }
    }

    /// Returns an expression for a value of this property.
    fn expr_for_value(&self, value: &str) -> TokenStream {
        match &self.kind {
            PropertyKind::Integer { .. } => {
                let value = i32::from_str(value).unwrap();
                quote! { #value }
            }
            PropertyKind::Boolean => {
                let value = bool::from_str(value).unwrap();
                quote! { #value }
            }
            PropertyKind::Enum { name, .. } => {
                let variant = ident(value.to_camel_case());
                quote! { #name::#variant }
            }
        }
    }
}

impl ToTokens for Property {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let x = match &self.kind {
            PropertyKind::Integer { .. } => quote! { i32 },
            PropertyKind::Boolean => quote! { bool },
            PropertyKind::Enum { name, .. } => quote! { #name },
        };

        tokens.extend(x);
    }
}

impl Property {
    /// Returns the tokens necessary to define this property's type,
    /// i.e. if it is an enum.
    pub fn tokens_for_definition(&self) -> Option<TokenStream> {
        match &self.kind {
            PropertyKind::Enum { name, variants } => Some({
                let definition = quote! {
                    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
                    #[repr(u16)]
                    pub enum #name {
                        #(
                            #variants,
                        )*
                    }
                };

                let variant_indices: Vec<_> = (0..variants.len() as u16).collect();
                let try_from_error_msg = format!("invalid value {{}} for {}", name);

                let imp = quote! {
                    impl TryFrom<u16> for #name {
                        type Error = anyhow::Error;

                        fn try_from(value: u16) -> anyhow::Result<Self> {
                            match value {
                                #(
                                    #variant_indices => Ok(#name::#variants),
                                )*
                                x => Err(anyhow::anyhow!(#try_from_error_msg, x)),
                            }
                        }
                    }
                };

                quote! {
                    #definition
                    #imp
                }
            }),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum PropertyKind {
    Integer { range: RangeInclusive<i32> },
    Boolean,
    Enum { name: Ident, variants: Vec<Ident> },
}

#[derive(Debug, Default)]
pub struct Output {
    pub kind: String,
    pub block_fns: String,
    pub block_table: String,
    pub block_table_serialized: Vec<u8>,
    pub vanilla_ids_serialized: Vec<u8>,
}

/// Generates code for the block report.
pub fn generate() -> anyhow::Result<Output> {
    let blocks = load::load()?;

    let mut output = Output::default();

    output.kind.push_str(&generate_kind(&blocks).to_string());
    let table_src = generate_table(&blocks);
    output.block_table.push_str(&table_src.to_string());
    let block_fns_src = generate_block_fns(&blocks);
    output.block_fns.push_str(&block_fns_src.to_string());

    output.block_table_serialized = serialize_block_table(&blocks);
    output.vanilla_ids_serialized = serialized_vanilla_ids(&blocks);

    Ok(output)
}

/// Generates the `BlockKind` enum.
fn generate_kind(blocks: &Blocks) -> TokenStream {
    let mut variants = vec![];

    for block in &blocks.blocks {
        let name = &block.name_camel_case;
        variants.push(quote! { #name });
    }

    quote! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ToPrimitive, FromPrimitive)]
        #[repr(u16)]
        pub enum BlockKind {
            #(#variants,)*
        }
    }
}

/// Generates the `BlockTable` struct and its implementation.
fn generate_table(blocks: &Blocks) -> TokenStream {
    let mut fields = vec![];
    let mut fns = vec![];
    let mut types = vec![];

    for property in blocks.property_types.values() {
        let name = &property.name;

        types.push(property.tokens_for_definition());

        fields.push(quote! {
            #name: Vec<(u16, u16)>
        });

        let from_u16 = property.tokens_for_from_u16(quote! { x });

        let doc = format!(
            "Retrieves the `{}` value for the given block kind with the given state value.
        Returns the value of the property, or `None` if it does not exist.",
            name
        );
        fns.push(quote! {
            #[doc = #doc]
            pub fn #name(&self, kind: BlockKind, state: u16) -> Option<#property> {
                let (offset_coefficient, stride) = self.#name[kind as u16 as usize];

                if offset_coefficient == 0 {
                    return None;
                }

                let x = crate::n_dimensional_index(state, offset_coefficient, stride);
                Some(#from_u16)
            }
        });

        let set = ident(format!("set_{}", name));
        let doc = format!("Updates the state value for the given block kind such that its `{}` value is updated. Returns the new state,
        or `None` if the block does not have this property.", name);
        let to_u16 = property.tokens_for_to_u16(quote! { value });
        fns.push(quote! {
            #[doc = #doc]
            pub fn #set(&self, kind: BlockKind, state: u16, value: #property) -> Option<u16> {
                let (offset_coefficient, stride) = self.#name[kind as u16 as usize];

                if offset_coefficient == 0 {
                    return None;
                }

                let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
                let new = ({ #to_u16 } as i32 - old) * stride as i32 + state as i32;
                Some(new as u16)
            }
        });
    }

    quote! {
        use crate::BlockKind;
        use std::convert::TryFrom;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        pub struct BlockTable {
            #(#fields,)*
        }

        impl BlockTable {
            #(#fns)*
        }

        #(#types)*
    }
}

/// Generated functions for `BlockId`.
fn generate_block_fns(blocks: &Blocks) -> TokenStream {
    let mut fns = vec![];

    for block in &blocks.blocks {
        let name = &block.name;
        let name_camel_case = &block.name_camel_case;

        let default_state = &block.default_state;

        let mut state_intializers = vec![];
        for (name, value) in default_state {
            let value_expr = blocks.property_types[name].expr_for_value(value);

            let name_fn = ident(format!("set_{}", name));
            state_intializers.push(quote! {
                block.#name_fn(#value_expr);
            });
        }

        let mut doc = format!(
            "Returns an instance of `{}` with default state values.",
            block.name
        );

        if !default_state.is_empty() {
            doc.push_str("\nThe default state values are as follows:\n");
        }

        for (name, value) in default_state {
            doc.push_str(&format!("* `{}`: {}\n", name, value));
        }

        fns.push(quote! {
            #[doc = #doc]
            pub fn #name() -> Self {
                let mut block = Self {
                    kind: BlockKind::#name_camel_case,
                    state: 0,
                };
                #(#state_intializers)*
                block
            }
        })
    }

    for property in blocks.property_types.values() {
        let property_name = &property.name;
        let set = ident(format!("set_{}", property_name));
        let with = ident(format!("with_{}", property_name));

        let f = quote! {
            pub fn #property_name(self) -> Option<#property> {
                BLOCK_TABLE.#property_name(self.kind, self.state)
            }

            pub fn #set(&mut self, #property_name: #property) -> bool {
                match BLOCK_TABLE.#set(self.kind, self.state, #property_name) {
                    Some(new_state) => {
                        self.state = new_state;
                        true
                    }
                    None => false,
                }
            }

            pub fn #with(mut self, #property_name: #property) -> Self {
                self.#set(#property_name);
                self
            }
        };
        fns.push(f);
    }

    let res = quote! {
        use crate::*;

        impl BlockId {
            #(#fns)*
        }
    };
    res
}

/// Returns the serialized `BlockTable`.
fn serialize_block_table(blocks: &Blocks) -> Vec<u8> {
    let table = BlockTableSerialize::new(&blocks.blocks, &blocks.property_types);

    bincode::serialize(&table).expect("bincode failed to serialize block table")
}

/// Serializable form of the generated `BlockTable`.
#[derive(Debug)]
struct BlockTableSerialize {
    fields: BTreeMap<String, Vec<(u16, u16)>>,
}

// custom serialize impl needed because of https://github.com/servo/bincode/issues/245
impl Serialize for BlockTableSerialize {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("BlockTable", self.fields.len())?;

        for (name, value) in &self.fields {
            // Leak memory! This is a build script; it doesn't matter.
            let name = Box::leak(name.clone().into_boxed_str());
            state.serialize_field(name, value)?;
        }

        state.end()
    }
}

impl BlockTableSerialize {
    pub fn new(blocks: &[Block], property_types: &BTreeMap<String, Property>) -> Self {
        let mut fields: BTreeMap<String, Vec<(u16, u16)>> = BTreeMap::new();

        for block in blocks {
            for property_name in property_types.keys() {
                let index_parameters = match block.index_parameters.get(property_name) {
                    Some(params) => *params,
                    None => (0, 0),
                };

                fields
                    .entry(property_name.clone())
                    .or_default()
                    .push(index_parameters);
            }
        }

        assert!(fields.values().map(Vec::len).all(|len| len == blocks.len()));

        Self { fields }
    }
}

/// Returns the serialized state ID map.
fn serialized_vanilla_ids(blocks: &Blocks) -> Vec<u8> {
    let table = VanillaStateIdSerialize::new(blocks);

    bincode::serialize(&table).expect("bincode failed to serialize vanilla ID table")
}

/// Serializable state ID table.
#[derive(Debug)]
struct VanillaStateIdSerialize {
    ids: Vec<Vec<u16>>, // indexed by [kind as u16 as usize][state as usize]
}

impl Serialize for VanillaStateIdSerialize {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_seq(Some(self.ids.len()))?;

        for id in &self.ids {
            state.serialize_element(id)?;
        }

        state.end()
    }
}

impl VanillaStateIdSerialize {
    pub fn new(blocks: &Blocks) -> Self {
        let mut ids: Vec<Vec<u16>> = std::iter::repeat_with(Vec::new)
            .take(blocks.blocks.len())
            .collect();

        for (i, block) in blocks.blocks.iter().enumerate() {
            for (state, id) in &block.ids {
                let mut internal_id = 0;

                for (property_name, property_value) in state {
                    let (offset_coefficient, stride) = block.index_parameters[property_name];

                    let index = blocks.property_types[property_name]
                        .possible_values
                        .iter()
                        .position(|val| val == property_value)
                        .unwrap();

                    let multiplier = internal_id / offset_coefficient;
                    let mut new = property_value_as_u16(
                        property_value,
                        index,
                        &blocks.property_types[property_name].kind,
                    ) * stride;
                    new += multiplier * offset_coefficient;
                    internal_id = new;
                }

                let internal_id = internal_id as usize;
                // pad with zeroes
                if internal_id >= ids[i].len() {
                    let to_extend = internal_id - ids[i].len() + 1;
                    ids[i].extend(std::iter::repeat(0).take(to_extend));
                }
                assert_eq!(ids[i][internal_id], 0, "failed for {}", block.name);
                ids[i][internal_id] = *id;
            }
        }

        Self { ids }
    }
}

fn property_value_as_u16(value: &str, index: usize, kind: &PropertyKind) -> u16 {
    let start = match kind {
        PropertyKind::Integer { range } => *range.start() as u16,
        _ => 0,
    };

    if let Ok(x) = i32::from_str(value) {
        x as u16 - start
    } else if let Ok(x) = bool::from_str(value) {
        x as u16
    } else {
        index as u16
    }
}
