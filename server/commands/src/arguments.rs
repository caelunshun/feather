use crate::CommandCtx;
use feather_core::position;
use feather_core::util::{Gamemode, Position};
use feather_definitions::Item;
use feather_server_types::{Game, Name, NetworkId, Player};
use fecs::{component, Entity, IntoQuery, Read, World};
use lieutenant::{ArgumentKind, Input};
use rand::Rng;
use smallvec::SmallVec;
use std::convert::Infallible;
use std::num::ParseFloatError;
use std::str::FromStr;
use thiserror::Error;

/*
#[derive(Debug, Error)]
pub enum SelectorParseError {
    #[error("no player with name {0}")]
    PlayerNotFound(String),
}

/// Argument kind which supports entity selectors.
pub struct EntitySelector {
    /// Entities selected by the parameter.
    pub entities: SmallVec<[Entity; 1]>,
}

impl ArgumentKind<CommandCtx> for EntitySelector {
    type ParseError = SelectorParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");

        true
    }

    fn parse<'a>(ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let head = input.advance_until(" ");

        // See https://minecraft.gamepedia.com/Commands#Target_selectors
        let entities = find_selected_entities(ctx, head)?;

        Ok(EntitySelector { entities })
    }
}

impl EntitySelector {
    /// Parses the returned entities for use in reporting success messages
    /// Either the name of the entity for one entity, or how many were affected for many entities.
    pub fn entities_to_string(&self, ctx: &CommandCtx, add_player: bool) -> String {
        if self.entities.is_empty() {
            "no entities".to_string()
        } else if self.entities.len() == 1 {
            if let Some(name) = ctx.world.try_get::<Name>(*self.entities.first().unwrap()) {
                if add_player {
                    if ctx
                        .world
                        .try_get::<Player>(*self.entities.first().unwrap())
                        .is_some()
                    {
                        format!("player {}", name.0)
                    } else {
                        format!("entity {}", name.0)
                    }
                } else {
                    name.0.to_string()
                }
            } else {
                "Server".to_string()
            }
        } else {
            // TODO: confirm this is correct behaviour for success messages involving many players
            let mut players = true;
            for entity in &self.entities {
                players &= ctx.world.try_get::<Player>(*entity).is_some();
            }
            if players {
                format!("{} players", self.entities.len())
            } else {
                format!("{} entities", self.entities.len())
            }
        }
    }
}

fn find_selected_entities(
    ctx: &CommandCtx,
    input: &str,
) -> Result<SmallVec<[Entity; 1]>, SelectorParseError> {
    use smallvec::smallvec;
    Ok(match input {
        "@p" => {
            // Nearest player
            let pos = ctx
                .world
                .try_get::<Position>(ctx.sender)
                .map(|r| *r)
                .unwrap_or(position!(0.0, 0.0, 0.0));

            nearest_player_to(&ctx.world, pos).into_iter().collect()
        }
        "@r" => {
            // Random player
            random_player(&ctx.game, &ctx.world).into_iter().collect()
        }
        "@a" => {
            // Every player
            <Read<Player>>::query()
                .iter_entities(ctx.world.inner())
                .map(|(e, _)| e)
                .collect()
        }
        "@e" => {
            // Every entity
            <Read<NetworkId>>::query()
                .iter_entities(ctx.world.inner())
                .map(|(e, _)| e)
                .collect()
        }
        "@s" => {
            // Command sender, if it was a player
            if ctx.world.has::<Player>(ctx.sender) {
                smallvec![ctx.sender]
            } else {
                SmallVec::new()
            }
        }
        player_name => smallvec![find_player_by_name(&ctx.world, player_name)
            .ok_or_else(|| SelectorParseError::PlayerNotFound(player_name.to_owned()))?],
    })
}

// TODO: eliminate linear searches.
// These search functions are incredibly naive.
fn find_player_by_name(world: &World, name: &str) -> Option<Entity> {
    <Read<Name>>::query()
        .iter_entities(world.inner())
        .find(|(_, n)| n.0 == name)
        .map(|(entity, _name)| entity)
}

fn nearest_player_to(world: &World, pos: Position) -> Option<Entity> {
    <Read<Position>>::query()
        .filter(component::<Player>())
        .iter_entities(world.inner())
        .min_by_key(|(_, p)| pos.distance_squared_to(**p).floor() as u64)
        .map(|(entity, _)| entity)
}

fn random_player(game: &Game, world: &World) -> Option<Entity> {
    let query = <Read<Player>>::query();

    let count = query.iter(world.inner()).count();

    let index = game.rng().gen_range(0, count);

    query
        .iter_entities(world.inner())
        .nth(index)
        .map(|(e, _)| e)
}

#[derive(Debug, Error)]
pub enum CoordinatesParseError {
    #[error("missing coordinate")]
    MissingCoordinate,
    #[error("failed to parse float: {0}")]
    ParseFloat(#[from] ParseFloatError),
}

/// Parses a position (<x> <y> <z>, but also with support for relative
/// positions as per https://minecraft.gamepedia.com/Commands#Tilde_and_caret_notation).
#[derive(Copy, Clone, Debug)]
pub struct Coordinates {
    pub x: Coordinate,
    pub y: Coordinate,
    pub z: Coordinate,
}

impl Coordinates {
    /// Converts these coordinates into a `Position`.
    ///
    /// The input `relative_to` is the position to interpret
    /// as the origin of relative coordinates. For example,
    /// this is the position of the target entity for the `/tp`
    /// command.
    pub fn into_position(self, relative_to: Position) -> Position {
        let direction = relative_to.direction();
        position!(
            Self::coordinate_into_absolute(self.x, relative_to.x, direction.x),
            Self::coordinate_into_absolute(self.y, relative_to.y, direction.y),
            Self::coordinate_into_absolute(self.z, relative_to.z, direction.z),
            relative_to.pitch,
            relative_to.yaw,
        )
    }

    fn coordinate_into_absolute(coord: Coordinate, relative_to: f64, facing_magnitude: f64) -> f64 {
        match coord {
            Coordinate::Absolute(coord) => coord,
            Coordinate::Relative(rel) => relative_to + rel,
            Coordinate::RelativeLook(rel) => relative_to + rel * facing_magnitude,
        }
    }
}

impl From<Position> for Coordinates {
    fn from(pos: Position) -> Self {
        Coordinates {
            x: Coordinate::Absolute(pos.x),
            y: Coordinate::Absolute(pos.y),
            z: Coordinate::Absolute(pos.z),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Coordinate {
    /// Coordinates relative to some position. The origin
    /// is interpreted differently by different commands.
    ///
    /// For example, `/tp` interprets this as the coordinates
    /// relative to the initial position of the target entity.
    /// On the other hand, another command may use
    /// this as the coordinates relative to the sender's
    /// position.
    Relative(f64),
    /// Relative coordinates in the direction the player is looking.
    /// This is similar to `Relative`, but the axes are rotated
    /// to align with the entity's view direction.
    RelativeLook(f64),
    /// Absolute coordinates, in world space.
    Absolute(f64),
}

impl FromStr for Coordinate {
    type Err = CoordinatesParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(first) = s.chars().next() {
            Ok(match first {
                '~' => {
                    let offset = if s.len() > 1 {
                        f64::from_str(&s[1..])?
                    } else {
                        0.0
                    };
                    Coordinate::Relative(offset)
                }
                '^' => {
                    let offset = if s.len() > 1 {
                        f64::from_str(&s[1..])?
                    } else {
                        0.0
                    };
                    Coordinate::RelativeLook(offset)
                }
                _ => Coordinate::Absolute(f64::from_str(s)?),
            })
        } else {
            Err(CoordinatesParseError::MissingCoordinate)
        }
    }
}

impl ArgumentKind<CommandCtx> for Coordinates {
    type ParseError = CoordinatesParseError;

    fn satisfies<'a>(ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        Self::parse(ctx, input).is_ok()
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let x = input.advance_until(" ");
        let y = input.advance_until(" ");
        let z = input.advance_until(" ");

        let x = Coordinate::from_str(x)?;
        let y = Coordinate::from_str(y)?;
        let z = Coordinate::from_str(z)?;

        Ok(Coordinates { x, y, z })
    }
}

#[derive(Debug, Error)]
pub enum GamemodeParseError {
    #[error("invalid gamemode string {0}")]
    InvalidGamemode(String),
}

/// A parsed gamemode string ("survival", "creative", ...)
#[derive(Copy, Clone, Debug)]
pub struct ParsedGamemode(pub Gamemode);

impl ArgumentKind<CommandCtx> for ParsedGamemode {
    type ParseError = GamemodeParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        !input.advance_until(" ").is_empty()
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let s = input.advance_until(" ");

        let gamemode = match s {
            "survival" => Gamemode::Survival,
            "creative" => Gamemode::Creative,
            "spectator" => Gamemode::Spectator,
            "adventure" => Gamemode::Adventure,
            s => return Err(GamemodeParseError::InvalidGamemode(s.to_owned())),
        };

        Ok(ParsedGamemode(gamemode))
    }
}

#[derive(Debug, Error)]
pub enum TextParseError {}

/// A multi-word argument (parses until the end of the command)
#[derive(Clone, Debug)]
pub struct TextArgument(pub String);

impl ArgumentKind<CommandCtx> for TextArgument {
    type ParseError = Infallible;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        !input.advance_to_end().is_empty()
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_to_end();

        Ok(TextArgument(text.to_owned()))
    }
}

impl AsRef<str> for TextArgument {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Error)]
pub enum ItemParseError {
    #[error("Unknown item {0}")]
    ItemDoesNotExist(String),
}

#[derive(Clone, Debug)]
pub struct ItemArgument(pub Item);

impl ArgumentKind<CommandCtx> for ItemArgument {
    type ParseError = ItemParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        !input.advance_until(" ").is_empty()
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        let item = Item::from_identifier(text);
        match item {
            Some(s) => Ok(ItemArgument(s)),
            None => Err(ItemParseError::ItemDoesNotExist(text.to_owned())),
        }
    }
}

#[derive(Debug, Error)]
pub enum I32ParseError {
    #[error("Invalid integer {0}")]
    Invalid(String),
}

#[derive(Clone, Debug)]
pub struct I32Argument(pub i32);

impl ArgumentKind<CommandCtx> for I32Argument {
    type ParseError = I32ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        !input.advance_until(" ").is_empty()
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        let number = text.parse::<i32>();
        match number {
            Ok(s) => Ok(I32Argument(s)),
            Err(_) => Err(I32ParseError::Invalid(text.to_owned())),
        }
    }
}

#[derive(Debug, Error)]
pub enum PositiveI32ParseError {
    #[error("Invalid integer {0}")]
    Invalid(String),
    #[error("Integer must not be less than 0, found {0}")]
    Negative(i32),
}

#[derive(Clone, Debug)]
pub struct PositiveI32Argument(pub i32);

impl ArgumentKind<CommandCtx> for PositiveI32Argument {
    type ParseError = PositiveI32ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        !input.advance_until(" ").is_empty()
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        let number = text.parse::<i32>();
        match number {
            Ok(integer) => {
                if integer >= 0 {
                    Ok(PositiveI32Argument(integer))
                } else {
                    Err(PositiveI32ParseError::Negative(integer))
                }
            }
            Err(_) => Err(PositiveI32ParseError::Invalid(text.to_owned())),
        }
    }
}
*/


#[derive(Debug, Error)]
pub enum BoolArgumentParseError {}

#[derive(Clone, Debug)]
pub struct BoolArgument(pub String);

impl ArgumentKind<CommandCtx> for BoolArgument {
    type ParseError = BoolArgumentParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(BoolArgument(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum DoubleArgumentParseError {}

#[derive(Clone, Debug)]
pub struct DoubleArgument(pub String);

impl ArgumentKind<CommandCtx> for DoubleArgument {
    type ParseError = DoubleArgumentParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(DoubleArgument(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum FloatArgumentParseError {}

#[derive(Clone, Debug)]
pub struct FloatArgument(pub String);

impl ArgumentKind<CommandCtx> for FloatArgument {
    type ParseError = FloatArgumentParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(FloatArgument(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum FloatArgumentBetween0And1ParseError {}

#[derive(Clone, Debug)]
pub struct FloatArgumentBetween0And1(pub String);

impl ArgumentKind<CommandCtx> for FloatArgumentBetween0And1 {
    type ParseError = FloatArgumentBetween0And1ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(FloatArgumentBetween0And1(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum FloatArgumentBetween0And2ParseError {}

#[derive(Clone, Debug)]
pub struct FloatArgumentBetween0And2(pub String);

impl ArgumentKind<CommandCtx> for FloatArgumentBetween0And2 {
    type ParseError = FloatArgumentBetween0And2ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(FloatArgumentBetween0And2(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum FloatArgumentPositiveParseError {}

#[derive(Clone, Debug)]
pub struct FloatArgumentPositive(pub String);

impl ArgumentKind<CommandCtx> for FloatArgumentPositive {
    type ParseError = FloatArgumentPositiveParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(FloatArgumentPositive(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum FloatArgumentGreaterThen1ParseError {}

#[derive(Clone, Debug)]
pub struct FloatArgumentGreaterThen1(pub String);

impl ArgumentKind<CommandCtx> for FloatArgumentGreaterThen1 {
    type ParseError = FloatArgumentGreaterThen1ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(FloatArgumentGreaterThen1(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntegerArgumentParseError {}

#[derive(Clone, Debug)]
pub struct IntegerArgument(pub String);

impl ArgumentKind<CommandCtx> for IntegerArgument {
    type ParseError = IntegerArgumentParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntegerArgument(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntegerArgumentBetween0And1000000ParseError {}

#[derive(Clone, Debug)]
pub struct IntegerArgumentBetween0And1000000(pub String);

impl ArgumentKind<CommandCtx> for IntegerArgumentBetween0And1000000 {
    type ParseError = IntegerArgumentBetween0And1000000ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntegerArgumentBetween0And1000000(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntegerArgumentBetween0And255ParseError {}

#[derive(Clone, Debug)]
pub struct IntegerArgumentBetween0And255(pub String);

impl ArgumentKind<CommandCtx> for IntegerArgumentBetween0And255 {
    type ParseError = IntegerArgumentBetween0And255ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntegerArgumentBetween0And255(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntegerArgumentBetween0And65535ParseError {}

#[derive(Clone, Debug)]
pub struct IntegerArgumentBetween0And65535(pub String);

impl ArgumentKind<CommandCtx> for IntegerArgumentBetween0And65535 {
    type ParseError = IntegerArgumentBetween0And65535ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntegerArgumentBetween0And65535(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntegerArgumentPositiveParseError {}

#[derive(Clone, Debug)]
pub struct IntegerArgumentPositive(pub String);

impl ArgumentKind<CommandCtx> for IntegerArgumentPositive {
    type ParseError = IntegerArgumentPositiveParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntegerArgumentPositive(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntegerArgumentBetween1And1000000ParseError {}

#[derive(Clone, Debug)]
pub struct IntegerArgumentBetween1And1000000(pub String);

impl ArgumentKind<CommandCtx> for IntegerArgumentBetween1And1000000 {
    type ParseError = IntegerArgumentBetween1And1000000ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntegerArgumentBetween1And1000000(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntegerArgumentBetween1And64ParseError {}

#[derive(Clone, Debug)]
pub struct IntegerArgumentBetween1And64(pub String);

impl ArgumentKind<CommandCtx> for IntegerArgumentBetween1And64 {
    type ParseError = IntegerArgumentBetween1And64ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntegerArgumentBetween1And64(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntegerArgumentGreaterThen1ParseError {}

#[derive(Clone, Debug)]
pub struct IntegerArgumentGreaterThen1(pub String);

impl ArgumentKind<CommandCtx> for IntegerArgumentGreaterThen1 {
    type ParseError = IntegerArgumentGreaterThen1ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntegerArgumentGreaterThen1(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum StringArgumentGreedyParseError {}

#[derive(Clone, Debug)]
pub struct StringArgumentGreedy(pub String);

impl ArgumentKind<CommandCtx> for StringArgumentGreedy {
    type ParseError = StringArgumentGreedyParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(StringArgumentGreedy(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum StringArgumentPhraseParseError {}

#[derive(Clone, Debug)]
pub struct StringArgumentPhrase(pub String);

impl ArgumentKind<CommandCtx> for StringArgumentPhrase {
    type ParseError = StringArgumentPhraseParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(StringArgumentPhrase(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum StringArgumentWordParseError {}

#[derive(Clone, Debug)]
pub struct StringArgumentWord(pub String);

impl ArgumentKind<CommandCtx> for StringArgumentWord {
    type ParseError = StringArgumentWordParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(StringArgumentWord(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum BlockPosParseError {}

#[derive(Clone, Debug)]
pub struct BlockPos(pub String);

impl ArgumentKind<CommandCtx> for BlockPos {
    type ParseError = BlockPosParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(BlockPos(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum BlockPredicateParseError {}

#[derive(Clone, Debug)]
pub struct BlockPredicate(pub String);

impl ArgumentKind<CommandCtx> for BlockPredicate {
    type ParseError = BlockPredicateParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(BlockPredicate(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum BlockStateParseError {}

#[derive(Clone, Debug)]
pub struct BlockState(pub String);

impl ArgumentKind<CommandCtx> for BlockState {
    type ParseError = BlockStateParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(BlockState(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ColorParseError {}

#[derive(Clone, Debug)]
pub struct Color(pub String);

impl ArgumentKind<CommandCtx> for Color {
    type ParseError = ColorParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Color(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ColumnPosParseError {}

#[derive(Clone, Debug)]
pub struct ColumnPos(pub String);

impl ArgumentKind<CommandCtx> for ColumnPos {
    type ParseError = ColumnPosParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(ColumnPos(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ComponentParseError {}

#[derive(Clone, Debug)]
pub struct Component(pub String);

impl ArgumentKind<CommandCtx> for Component {
    type ParseError = ComponentParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Component(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum DimensionParseError {}

#[derive(Clone, Debug)]
pub struct Dimension(pub String);

impl ArgumentKind<CommandCtx> for Dimension {
    type ParseError = DimensionParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Dimension(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum EntityAnchorParseError {}

#[derive(Clone, Debug)]
pub struct EntityAnchor(pub String);

impl ArgumentKind<CommandCtx> for EntityAnchor {
    type ParseError = EntityAnchorParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(EntityAnchor(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum EntitySummonParseError {}

#[derive(Clone, Debug)]
pub struct EntitySummon(pub String);

impl ArgumentKind<CommandCtx> for EntitySummon {
    type ParseError = EntitySummonParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(EntitySummon(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum MultipleEntitiesParseError {}

#[derive(Clone, Debug)]
pub struct MultipleEntities(pub String);

impl ArgumentKind<CommandCtx> for MultipleEntities {
    type ParseError = MultipleEntitiesParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(MultipleEntities(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum MultiplePlayersParseError {}

#[derive(Clone, Debug)]
pub struct MultiplePlayers(pub String);

impl ArgumentKind<CommandCtx> for MultiplePlayers {
    type ParseError = MultiplePlayersParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(MultiplePlayers(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum SingleEntitiesParseError {}

#[derive(Clone, Debug)]
pub struct SingleEntities(pub String);

impl ArgumentKind<CommandCtx> for SingleEntities {
    type ParseError = SingleEntitiesParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(SingleEntities(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum SinglePlayerParseError {}

#[derive(Clone, Debug)]
pub struct SinglePlayer(pub String);

impl ArgumentKind<CommandCtx> for SinglePlayer {
    type ParseError = SinglePlayerParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(SinglePlayer(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum MinecraftFunctionParseError {}

#[derive(Clone, Debug)]
pub struct MinecraftFunction(pub String);

impl ArgumentKind<CommandCtx> for MinecraftFunction {
    type ParseError = MinecraftFunctionParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(MinecraftFunction(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum GameProfileParseError {}

#[derive(Clone, Debug)]
pub struct GameProfile(pub String);

impl ArgumentKind<CommandCtx> for GameProfile {
    type ParseError = GameProfileParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(GameProfile(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum IntRageParseError {}

#[derive(Clone, Debug)]
pub struct IntRage(pub String);

impl ArgumentKind<CommandCtx> for IntRage {
    type ParseError = IntRageParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(IntRage(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum EnchantmentParseError {}

#[derive(Clone, Debug)]
pub struct Enchantment(pub String);

impl ArgumentKind<CommandCtx> for Enchantment {
    type ParseError = EnchantmentParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Enchantment(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum PredicateParseError {}

#[derive(Clone, Debug)]
pub struct Predicate(pub String);

impl ArgumentKind<CommandCtx> for Predicate {
    type ParseError = PredicateParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Predicate(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ItemSlotParseError {}

#[derive(Clone, Debug)]
pub struct ItemSlot(pub String);

impl ArgumentKind<CommandCtx> for ItemSlot {
    type ParseError = ItemSlotParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(ItemSlot(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ItemStackParseError {}

#[derive(Clone, Debug)]
pub struct ItemStack(pub String);

impl ArgumentKind<CommandCtx> for ItemStack {
    type ParseError = ItemStackParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(ItemStack(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum MessageParseError {}

#[derive(Clone, Debug)]
pub struct Message(pub String);

impl ArgumentKind<CommandCtx> for Message {
    type ParseError = MessageParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Message(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum MobEffectParseError {}

#[derive(Clone, Debug)]
pub struct MobEffect(pub String);

impl ArgumentKind<CommandCtx> for MobEffect {
    type ParseError = MobEffectParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(MobEffect(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum NbtCommandTagParseError {}

#[derive(Clone, Debug)]
pub struct NbtCommandTag(pub String);

impl ArgumentKind<CommandCtx> for NbtCommandTag {
    type ParseError = NbtCommandTagParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(NbtCommandTag(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum NbtPathParseError {}

#[derive(Clone, Debug)]
pub struct NbtPath(pub String);

impl ArgumentKind<CommandCtx> for NbtPath {
    type ParseError = NbtPathParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(NbtPath(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum NbtTagParseError {}

#[derive(Clone, Debug)]
pub struct NbtTag(pub String);

impl ArgumentKind<CommandCtx> for NbtTag {
    type ParseError = NbtTagParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(NbtTag(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ObjectiveParseError {}

#[derive(Clone, Debug)]
pub struct Objective(pub String);

impl ArgumentKind<CommandCtx> for Objective {
    type ParseError = ObjectiveParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Objective(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ObjectiveCriteriaParseError {}

#[derive(Clone, Debug)]
pub struct ObjectiveCriteria(pub String);

impl ArgumentKind<CommandCtx> for ObjectiveCriteria {
    type ParseError = ObjectiveCriteriaParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(ObjectiveCriteria(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum OperationParseError {}

#[derive(Clone, Debug)]
pub struct Operation(pub String);

impl ArgumentKind<CommandCtx> for Operation {
    type ParseError = OperationParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Operation(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ParticleParseError {}

#[derive(Clone, Debug)]
pub struct Particle(pub String);

impl ArgumentKind<CommandCtx> for Particle {
    type ParseError = ParticleParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Particle(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ResourceLocationParseError {}

#[derive(Clone, Debug)]
pub struct ResourceLocation(pub String);

impl ArgumentKind<CommandCtx> for ResourceLocation {
    type ParseError = ResourceLocationParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(ResourceLocation(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum RotationParseError {}

#[derive(Clone, Debug)]
pub struct Rotation(pub String);

impl ArgumentKind<CommandCtx> for Rotation {
    type ParseError = RotationParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Rotation(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum MultipleScoreHoldersParseError {}

#[derive(Clone, Debug)]
pub struct MultipleScoreHolders(pub String);

impl ArgumentKind<CommandCtx> for MultipleScoreHolders {
    type ParseError = MultipleScoreHoldersParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(MultipleScoreHolders(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum SingleScoreHolderParseError {}

#[derive(Clone, Debug)]
pub struct SingleScoreHolder(pub String);

impl ArgumentKind<CommandCtx> for SingleScoreHolder {
    type ParseError = SingleScoreHolderParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(SingleScoreHolder(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum ScoreboardSlotParseError {}

#[derive(Clone, Debug)]
pub struct ScoreboardSlot(pub String);

impl ArgumentKind<CommandCtx> for ScoreboardSlot {
    type ParseError = ScoreboardSlotParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(ScoreboardSlot(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum SwizzleParseError {}

#[derive(Clone, Debug)]
pub struct Swizzle(pub String);

impl ArgumentKind<CommandCtx> for Swizzle {
    type ParseError = SwizzleParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Swizzle(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum TeamParseError {}

#[derive(Clone, Debug)]
pub struct Team(pub String);

impl ArgumentKind<CommandCtx> for Team {
    type ParseError = TeamParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Team(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum TimeParseError {}

#[derive(Clone, Debug)]
pub struct Time(pub String);

impl ArgumentKind<CommandCtx> for Time {
    type ParseError = TimeParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Time(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum UuidParseError {}

#[derive(Clone, Debug)]
pub struct Uuid(pub String);

impl ArgumentKind<CommandCtx> for Uuid {
    type ParseError = UuidParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Uuid(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum Vec2ParseError {}

#[derive(Clone, Debug)]
pub struct Vec2(pub String);

impl ArgumentKind<CommandCtx> for Vec2 {
    type ParseError = Vec2ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Vec2(text.to_owned()))
    }
}


#[derive(Debug, Error)]
pub enum Vec3ParseError {}

#[derive(Clone, Debug)]
pub struct Vec3(pub String);

impl ArgumentKind<CommandCtx> for Vec3 {
    type ParseError = Vec3ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");
        //TODO 
        true
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        //TODO
        Ok(Vec3(text.to_owned()))
    }
}

