//! Module for creating and modifying inventories of any type.

use crate::item::Item;

pub type SlotIndex = usize;

// Constants representing various standard inventory slot indices

pub const SLOT_CRAFTING_OUTPUT: SlotIndex = 0;
pub const SLOT_CRAFTING_INPUT_X0_Y0: SlotIndex = 1;
pub const SLOT_CRAFTING_INPUT_X1_Y0: SlotIndex = 2;
pub const SLOT_CRAFTING_INPUT_X0_Y1: SlotIndex = 3;
pub const SLOT_CRAFTING_INPUT_X1_Y1: SlotIndex = 4;

pub const SLOT_ARMOR_HEAD: SlotIndex = 5;
pub const SLOT_ARMOR_CHEST: SlotIndex = 6;
pub const SLOT_ARMOR_LEGS: SlotIndex = 7;
pub const SLOT_ARMOR_FEET: SlotIndex = 8;

pub const SLOT_OFFHAND: SlotIndex = 45;

pub const SLOT_INVENTORY_OFFSET: SlotIndex = 9;
pub const SLOT_HOTBAR_OFFSET: SlotIndex = 36;

/// The various types of inventories ("windows").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InventoryType {
    Player,
    Container,
    Chest,
    CraftingTable,
    Furnace,
    Dispenser,
    EnchantingTable,
    BrewingStand,
    Villager,
    Beacon,
    Anvil,
    Hopper,
    Dropper,
    ShulkerBox,
    Horse,
}

/// An inventory, consisting of a vector
/// of `Slot`s and a type.
#[derive(Debug, Clone)]
pub struct Inventory {
    /// The item vector.
    ///
    /// The vector always contains an entry
    /// for each slot in the inventory, indexed
    /// by the slot IDs. When an entry is set to
    /// `None`, there is no item in the slot.
    items: Vec<Option<ItemStack>>,
    /// The type of this inventory.
    pub ty: InventoryType,
}

impl Inventory {
    /// Creates a new inventory of the given
    /// type and number of slots.
    pub fn new(ty: InventoryType, num_slots: u32) -> Self {
        Self {
            items: vec![None; num_slots as usize],
            ty,
        }
    }

    /// Retrieves a reference to the item at the given slot index.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn item_at(&self, index: SlotIndex) -> Option<&ItemStack> {
        self.items[index].as_ref()
    }

    pub fn item_at_mut(&mut self, index: SlotIndex) -> Option<&mut ItemStack> {
        self.items[index].as_mut()
    }

    /// Sets the item at the given slot index.
    pub fn set_item_at(&mut self, index: SlotIndex, item: ItemStack) {
        self.items[index] = Some(item);
    }

    /// Clears the item at the given slot index, returning
    /// the old item.
    pub fn clear_item_at(&mut self, index: SlotIndex) -> Option<ItemStack> {
        self.items[index].take()
    }

    /// Returns the number of slots in this inventory.
    pub fn slot_count(&self) -> u16 {
        self.items.len() as u16
    }
}

/// Represents an item stack.
///
/// An item stack includes a type, an amount, and a bunch of properties (enchantments, etc.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemStack {
    /// The type of this item.
    pub ty: Item,
    /// The number of items in this stack.
    pub amount: u8,
    // TODO enchantments, more
}

impl ItemStack {
    pub fn new(ty: Item, amount: u8) -> Self {
        Self { ty, amount }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory() {
        let mut inv = Inventory::new(InventoryType::Chest, 36);
        assert_eq!(inv.slot_count(), 36);

        inv.set_item_at(0, ItemStack::new(Item::Air, 0));

        let item = inv.item_at(0).unwrap();
        assert_eq!(item.ty, Item::Air);
        assert_eq!(item.amount, 0);

        let item = inv.item_at_mut(0).unwrap();
        item.ty = Item::Sponge;
        assert_eq!(inv.item_at(0).unwrap().ty, Item::Sponge);

        inv.clear_item_at(0);
        assert!(inv.item_at(0).is_none());
    }
}
