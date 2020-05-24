use crate::{BlockId, BlockKind};
impl BlockId {
    #[doc = "Determines whether or not a block has the `age_0_15` property."]
    pub fn has_age_0_15(self) -> bool {
        match self.kind() {
            BlockKind::Fire | BlockKind::Cactus | BlockKind::SugarCane => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `age_0_2` property."]
    pub fn has_age_0_2(self) -> bool {
        match self.kind() {
            BlockKind::Cocoa => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `age_0_25` property."]
    pub fn has_age_0_25(self) -> bool {
        match self.kind() {
            BlockKind::Kelp => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `age_0_3` property."]
    pub fn has_age_0_3(self) -> bool {
        match self.kind() {
            BlockKind::NetherWart | BlockKind::Beetroots | BlockKind::FrostedIce => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `age_0_5` property."]
    pub fn has_age_0_5(self) -> bool {
        match self.kind() {
            BlockKind::ChorusFlower => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `age_0_7` property."]
    pub fn has_age_0_7(self) -> bool {
        match self.kind() {
            BlockKind::Wheat
            | BlockKind::PumpkinStem
            | BlockKind::MelonStem
            | BlockKind::Carrots
            | BlockKind::Potatoes => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `attached` property."]
    pub fn has_attached(self) -> bool {
        match self.kind() {
            BlockKind::TripwireHook | BlockKind::Tripwire => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `axis_xyz` property."]
    pub fn has_axis_xyz(self) -> bool {
        match self.kind() {
            BlockKind::OakLog
            | BlockKind::SpruceLog
            | BlockKind::BirchLog
            | BlockKind::JungleLog
            | BlockKind::AcaciaLog
            | BlockKind::DarkOakLog
            | BlockKind::StrippedSpruceLog
            | BlockKind::StrippedBirchLog
            | BlockKind::StrippedJungleLog
            | BlockKind::StrippedAcaciaLog
            | BlockKind::StrippedDarkOakLog
            | BlockKind::StrippedOakLog
            | BlockKind::OakWood
            | BlockKind::SpruceWood
            | BlockKind::BirchWood
            | BlockKind::JungleWood
            | BlockKind::AcaciaWood
            | BlockKind::DarkOakWood
            | BlockKind::StrippedOakWood
            | BlockKind::StrippedSpruceWood
            | BlockKind::StrippedBirchWood
            | BlockKind::StrippedJungleWood
            | BlockKind::StrippedAcaciaWood
            | BlockKind::StrippedDarkOakWood
            | BlockKind::QuartzPillar
            | BlockKind::HayBlock
            | BlockKind::PurpurPillar
            | BlockKind::BoneBlock => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `axis_xz` property."]
    pub fn has_axis_xz(self) -> bool {
        match self.kind() {
            BlockKind::NetherPortal => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `bites` property."]
    pub fn has_bites(self) -> bool {
        match self.kind() {
            BlockKind::Cake => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `cauldron_level` property."]
    pub fn has_cauldron_level(self) -> bool {
        match self.kind() {
            BlockKind::Cauldron => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `chest_kind` property."]
    pub fn has_chest_kind(self) -> bool {
        match self.kind() {
            BlockKind::Chest | BlockKind::TrappedChest => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `comparator_mode` property."]
    pub fn has_comparator_mode(self) -> bool {
        match self.kind() {
            BlockKind::Comparator => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `conditional` property."]
    pub fn has_conditional(self) -> bool {
        match self.kind() {
            BlockKind::CommandBlock
            | BlockKind::RepeatingCommandBlock
            | BlockKind::ChainCommandBlock => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `delay` property."]
    pub fn has_delay(self) -> bool {
        match self.kind() {
            BlockKind::Repeater => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `disarmed` property."]
    pub fn has_disarmed(self) -> bool {
        match self.kind() {
            BlockKind::Tripwire => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `distance` property."]
    pub fn has_distance(self) -> bool {
        match self.kind() {
            BlockKind::OakLeaves
            | BlockKind::SpruceLeaves
            | BlockKind::BirchLeaves
            | BlockKind::JungleLeaves
            | BlockKind::AcaciaLeaves
            | BlockKind::DarkOakLeaves => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `down` property."]
    pub fn has_down(self) -> bool {
        match self.kind() {
            BlockKind::BrownMushroomBlock
            | BlockKind::RedMushroomBlock
            | BlockKind::MushroomStem
            | BlockKind::ChorusPlant => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `drag` property."]
    pub fn has_drag(self) -> bool {
        match self.kind() {
            BlockKind::BubbleColumn => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `east_connected` property."]
    pub fn has_east_connected(self) -> bool {
        match self.kind() {
            BlockKind::Fire
            | BlockKind::OakFence
            | BlockKind::BrownMushroomBlock
            | BlockKind::RedMushroomBlock
            | BlockKind::MushroomStem
            | BlockKind::IronBars
            | BlockKind::GlassPane
            | BlockKind::Vine
            | BlockKind::NetherBrickFence
            | BlockKind::Tripwire
            | BlockKind::CobblestoneWall
            | BlockKind::MossyCobblestoneWall
            | BlockKind::WhiteStainedGlassPane
            | BlockKind::OrangeStainedGlassPane
            | BlockKind::MagentaStainedGlassPane
            | BlockKind::LightBlueStainedGlassPane
            | BlockKind::YellowStainedGlassPane
            | BlockKind::LimeStainedGlassPane
            | BlockKind::PinkStainedGlassPane
            | BlockKind::GrayStainedGlassPane
            | BlockKind::LightGrayStainedGlassPane
            | BlockKind::CyanStainedGlassPane
            | BlockKind::PurpleStainedGlassPane
            | BlockKind::BlueStainedGlassPane
            | BlockKind::BrownStainedGlassPane
            | BlockKind::GreenStainedGlassPane
            | BlockKind::RedStainedGlassPane
            | BlockKind::BlackStainedGlassPane
            | BlockKind::SpruceFence
            | BlockKind::BirchFence
            | BlockKind::JungleFence
            | BlockKind::AcaciaFence
            | BlockKind::DarkOakFence
            | BlockKind::ChorusPlant => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `east_wire` property."]
    pub fn has_east_wire(self) -> bool {
        match self.kind() {
            BlockKind::RedstoneWire => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `eggs` property."]
    pub fn has_eggs(self) -> bool {
        match self.kind() {
            BlockKind::TurtleEgg => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `enabled` property."]
    pub fn has_enabled(self) -> bool {
        match self.kind() {
            BlockKind::Hopper => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `extended` property."]
    pub fn has_extended(self) -> bool {
        match self.kind() {
            BlockKind::StickyPiston | BlockKind::Piston => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `eye` property."]
    pub fn has_eye(self) -> bool {
        match self.kind() {
            BlockKind::EndPortalFrame => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `face` property."]
    pub fn has_face(self) -> bool {
        match self.kind() {
            BlockKind::Lever
            | BlockKind::StoneButton
            | BlockKind::OakButton
            | BlockKind::SpruceButton
            | BlockKind::BirchButton
            | BlockKind::JungleButton
            | BlockKind::AcaciaButton
            | BlockKind::DarkOakButton => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `facing_cardinal` property."]
    pub fn has_facing_cardinal(self) -> bool {
        match self.kind() {
            BlockKind::WhiteBed
            | BlockKind::OrangeBed
            | BlockKind::MagentaBed
            | BlockKind::LightBlueBed
            | BlockKind::YellowBed
            | BlockKind::LimeBed
            | BlockKind::PinkBed
            | BlockKind::GrayBed
            | BlockKind::LightGrayBed
            | BlockKind::CyanBed
            | BlockKind::PurpleBed
            | BlockKind::BlueBed
            | BlockKind::BrownBed
            | BlockKind::GreenBed
            | BlockKind::RedBed
            | BlockKind::BlackBed
            | BlockKind::WallTorch
            | BlockKind::OakStairs
            | BlockKind::Chest
            | BlockKind::Furnace
            | BlockKind::OakDoor
            | BlockKind::Ladder
            | BlockKind::CobblestoneStairs
            | BlockKind::WallSign
            | BlockKind::Lever
            | BlockKind::IronDoor
            | BlockKind::RedstoneWallTorch
            | BlockKind::StoneButton
            | BlockKind::CarvedPumpkin
            | BlockKind::JackOLantern
            | BlockKind::Repeater
            | BlockKind::OakTrapdoor
            | BlockKind::SpruceTrapdoor
            | BlockKind::BirchTrapdoor
            | BlockKind::JungleTrapdoor
            | BlockKind::AcaciaTrapdoor
            | BlockKind::DarkOakTrapdoor
            | BlockKind::AttachedPumpkinStem
            | BlockKind::AttachedMelonStem
            | BlockKind::OakFenceGate
            | BlockKind::BrickStairs
            | BlockKind::StoneBrickStairs
            | BlockKind::NetherBrickStairs
            | BlockKind::EndPortalFrame
            | BlockKind::Cocoa
            | BlockKind::SandstoneStairs
            | BlockKind::EnderChest
            | BlockKind::TripwireHook
            | BlockKind::SpruceStairs
            | BlockKind::BirchStairs
            | BlockKind::JungleStairs
            | BlockKind::OakButton
            | BlockKind::SpruceButton
            | BlockKind::BirchButton
            | BlockKind::JungleButton
            | BlockKind::AcaciaButton
            | BlockKind::DarkOakButton
            | BlockKind::SkeletonWallSkull
            | BlockKind::WitherSkeletonWallSkull
            | BlockKind::ZombieWallHead
            | BlockKind::PlayerWallHead
            | BlockKind::CreeperWallHead
            | BlockKind::DragonWallHead
            | BlockKind::Anvil
            | BlockKind::ChippedAnvil
            | BlockKind::DamagedAnvil
            | BlockKind::TrappedChest
            | BlockKind::Comparator
            | BlockKind::QuartzStairs
            | BlockKind::AcaciaStairs
            | BlockKind::DarkOakStairs
            | BlockKind::IronTrapdoor
            | BlockKind::PrismarineStairs
            | BlockKind::PrismarineBrickStairs
            | BlockKind::DarkPrismarineStairs
            | BlockKind::WhiteWallBanner
            | BlockKind::OrangeWallBanner
            | BlockKind::MagentaWallBanner
            | BlockKind::LightBlueWallBanner
            | BlockKind::YellowWallBanner
            | BlockKind::LimeWallBanner
            | BlockKind::PinkWallBanner
            | BlockKind::GrayWallBanner
            | BlockKind::LightGrayWallBanner
            | BlockKind::CyanWallBanner
            | BlockKind::PurpleWallBanner
            | BlockKind::BlueWallBanner
            | BlockKind::BrownWallBanner
            | BlockKind::GreenWallBanner
            | BlockKind::RedWallBanner
            | BlockKind::BlackWallBanner
            | BlockKind::RedSandstoneStairs
            | BlockKind::SpruceFenceGate
            | BlockKind::BirchFenceGate
            | BlockKind::JungleFenceGate
            | BlockKind::AcaciaFenceGate
            | BlockKind::DarkOakFenceGate
            | BlockKind::SpruceDoor
            | BlockKind::BirchDoor
            | BlockKind::JungleDoor
            | BlockKind::AcaciaDoor
            | BlockKind::DarkOakDoor
            | BlockKind::PurpurStairs
            | BlockKind::WhiteGlazedTerracotta
            | BlockKind::OrangeGlazedTerracotta
            | BlockKind::MagentaGlazedTerracotta
            | BlockKind::LightBlueGlazedTerracotta
            | BlockKind::YellowGlazedTerracotta
            | BlockKind::LimeGlazedTerracotta
            | BlockKind::PinkGlazedTerracotta
            | BlockKind::GrayGlazedTerracotta
            | BlockKind::LightGrayGlazedTerracotta
            | BlockKind::CyanGlazedTerracotta
            | BlockKind::PurpleGlazedTerracotta
            | BlockKind::BlueGlazedTerracotta
            | BlockKind::BrownGlazedTerracotta
            | BlockKind::GreenGlazedTerracotta
            | BlockKind::RedGlazedTerracotta
            | BlockKind::BlackGlazedTerracotta
            | BlockKind::DeadTubeCoralWallFan
            | BlockKind::DeadBrainCoralWallFan
            | BlockKind::DeadBubbleCoralWallFan
            | BlockKind::DeadFireCoralWallFan
            | BlockKind::DeadHornCoralWallFan
            | BlockKind::TubeCoralWallFan
            | BlockKind::BrainCoralWallFan
            | BlockKind::BubbleCoralWallFan
            | BlockKind::FireCoralWallFan
            | BlockKind::HornCoralWallFan => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `facing_cardinal_and_down` property."]
    pub fn has_facing_cardinal_and_down(self) -> bool {
        match self.kind() {
            BlockKind::Hopper => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `facing_cubic` property."]
    pub fn has_facing_cubic(self) -> bool {
        match self.kind() {
            BlockKind::Dispenser
            | BlockKind::StickyPiston
            | BlockKind::Piston
            | BlockKind::PistonHead
            | BlockKind::MovingPiston
            | BlockKind::CommandBlock
            | BlockKind::Dropper
            | BlockKind::EndRod
            | BlockKind::RepeatingCommandBlock
            | BlockKind::ChainCommandBlock
            | BlockKind::Observer
            | BlockKind::ShulkerBox
            | BlockKind::WhiteShulkerBox
            | BlockKind::OrangeShulkerBox
            | BlockKind::MagentaShulkerBox
            | BlockKind::LightBlueShulkerBox
            | BlockKind::YellowShulkerBox
            | BlockKind::LimeShulkerBox
            | BlockKind::PinkShulkerBox
            | BlockKind::GrayShulkerBox
            | BlockKind::LightGrayShulkerBox
            | BlockKind::CyanShulkerBox
            | BlockKind::PurpleShulkerBox
            | BlockKind::BlueShulkerBox
            | BlockKind::BrownShulkerBox
            | BlockKind::GreenShulkerBox
            | BlockKind::RedShulkerBox
            | BlockKind::BlackShulkerBox => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `half_top_bottom` property."]
    pub fn has_half_top_bottom(self) -> bool {
        match self.kind() {
            BlockKind::OakStairs
            | BlockKind::CobblestoneStairs
            | BlockKind::OakTrapdoor
            | BlockKind::SpruceTrapdoor
            | BlockKind::BirchTrapdoor
            | BlockKind::JungleTrapdoor
            | BlockKind::AcaciaTrapdoor
            | BlockKind::DarkOakTrapdoor
            | BlockKind::BrickStairs
            | BlockKind::StoneBrickStairs
            | BlockKind::NetherBrickStairs
            | BlockKind::SandstoneStairs
            | BlockKind::SpruceStairs
            | BlockKind::BirchStairs
            | BlockKind::JungleStairs
            | BlockKind::QuartzStairs
            | BlockKind::AcaciaStairs
            | BlockKind::DarkOakStairs
            | BlockKind::IronTrapdoor
            | BlockKind::PrismarineStairs
            | BlockKind::PrismarineBrickStairs
            | BlockKind::DarkPrismarineStairs
            | BlockKind::RedSandstoneStairs
            | BlockKind::PurpurStairs => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `half_upper_lower` property."]
    pub fn has_half_upper_lower(self) -> bool {
        match self.kind() {
            BlockKind::TallSeagrass
            | BlockKind::OakDoor
            | BlockKind::IronDoor
            | BlockKind::Sunflower
            | BlockKind::Lilac
            | BlockKind::RoseBush
            | BlockKind::Peony
            | BlockKind::TallGrass
            | BlockKind::LargeFern
            | BlockKind::SpruceDoor
            | BlockKind::BirchDoor
            | BlockKind::JungleDoor
            | BlockKind::AcaciaDoor
            | BlockKind::DarkOakDoor => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `has_bottle_0` property."]
    pub fn has_has_bottle_0(self) -> bool {
        match self.kind() {
            BlockKind::BrewingStand => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `has_bottle_1` property."]
    pub fn has_has_bottle_1(self) -> bool {
        match self.kind() {
            BlockKind::BrewingStand => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `has_bottle_2` property."]
    pub fn has_has_bottle_2(self) -> bool {
        match self.kind() {
            BlockKind::BrewingStand => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `has_record` property."]
    pub fn has_has_record(self) -> bool {
        match self.kind() {
            BlockKind::Jukebox => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `hatch` property."]
    pub fn has_hatch(self) -> bool {
        match self.kind() {
            BlockKind::TurtleEgg => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `hinge` property."]
    pub fn has_hinge(self) -> bool {
        match self.kind() {
            BlockKind::OakDoor
            | BlockKind::IronDoor
            | BlockKind::SpruceDoor
            | BlockKind::BirchDoor
            | BlockKind::JungleDoor
            | BlockKind::AcaciaDoor
            | BlockKind::DarkOakDoor => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `in_wall` property."]
    pub fn has_in_wall(self) -> bool {
        match self.kind() {
            BlockKind::OakFenceGate
            | BlockKind::SpruceFenceGate
            | BlockKind::BirchFenceGate
            | BlockKind::JungleFenceGate
            | BlockKind::AcaciaFenceGate
            | BlockKind::DarkOakFenceGate => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `instrument` property."]
    pub fn has_instrument(self) -> bool {
        match self.kind() {
            BlockKind::NoteBlock => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `inverted` property."]
    pub fn has_inverted(self) -> bool {
        match self.kind() {
            BlockKind::DaylightDetector => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `layers` property."]
    pub fn has_layers(self) -> bool {
        match self.kind() {
            BlockKind::Snow => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `lit` property."]
    pub fn has_lit(self) -> bool {
        match self.kind() {
            BlockKind::Furnace
            | BlockKind::RedstoneOre
            | BlockKind::RedstoneTorch
            | BlockKind::RedstoneWallTorch
            | BlockKind::RedstoneLamp => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `locked` property."]
    pub fn has_locked(self) -> bool {
        match self.kind() {
            BlockKind::Repeater => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `moisture` property."]
    pub fn has_moisture(self) -> bool {
        match self.kind() {
            BlockKind::Farmland => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `north_connected` property."]
    pub fn has_north_connected(self) -> bool {
        match self.kind() {
            BlockKind::Fire
            | BlockKind::OakFence
            | BlockKind::BrownMushroomBlock
            | BlockKind::RedMushroomBlock
            | BlockKind::MushroomStem
            | BlockKind::IronBars
            | BlockKind::GlassPane
            | BlockKind::Vine
            | BlockKind::NetherBrickFence
            | BlockKind::Tripwire
            | BlockKind::CobblestoneWall
            | BlockKind::MossyCobblestoneWall
            | BlockKind::WhiteStainedGlassPane
            | BlockKind::OrangeStainedGlassPane
            | BlockKind::MagentaStainedGlassPane
            | BlockKind::LightBlueStainedGlassPane
            | BlockKind::YellowStainedGlassPane
            | BlockKind::LimeStainedGlassPane
            | BlockKind::PinkStainedGlassPane
            | BlockKind::GrayStainedGlassPane
            | BlockKind::LightGrayStainedGlassPane
            | BlockKind::CyanStainedGlassPane
            | BlockKind::PurpleStainedGlassPane
            | BlockKind::BlueStainedGlassPane
            | BlockKind::BrownStainedGlassPane
            | BlockKind::GreenStainedGlassPane
            | BlockKind::RedStainedGlassPane
            | BlockKind::BlackStainedGlassPane
            | BlockKind::SpruceFence
            | BlockKind::BirchFence
            | BlockKind::JungleFence
            | BlockKind::AcaciaFence
            | BlockKind::DarkOakFence
            | BlockKind::ChorusPlant => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `north_wire` property."]
    pub fn has_north_wire(self) -> bool {
        match self.kind() {
            BlockKind::RedstoneWire => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `note` property."]
    pub fn has_note(self) -> bool {
        match self.kind() {
            BlockKind::NoteBlock => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `occupied` property."]
    pub fn has_occupied(self) -> bool {
        match self.kind() {
            BlockKind::WhiteBed
            | BlockKind::OrangeBed
            | BlockKind::MagentaBed
            | BlockKind::LightBlueBed
            | BlockKind::YellowBed
            | BlockKind::LimeBed
            | BlockKind::PinkBed
            | BlockKind::GrayBed
            | BlockKind::LightGrayBed
            | BlockKind::CyanBed
            | BlockKind::PurpleBed
            | BlockKind::BlueBed
            | BlockKind::BrownBed
            | BlockKind::GreenBed
            | BlockKind::RedBed
            | BlockKind::BlackBed => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `open` property."]
    pub fn has_open(self) -> bool {
        match self.kind() {
            BlockKind::OakDoor
            | BlockKind::IronDoor
            | BlockKind::OakTrapdoor
            | BlockKind::SpruceTrapdoor
            | BlockKind::BirchTrapdoor
            | BlockKind::JungleTrapdoor
            | BlockKind::AcaciaTrapdoor
            | BlockKind::DarkOakTrapdoor
            | BlockKind::OakFenceGate
            | BlockKind::IronTrapdoor
            | BlockKind::SpruceFenceGate
            | BlockKind::BirchFenceGate
            | BlockKind::JungleFenceGate
            | BlockKind::AcaciaFenceGate
            | BlockKind::DarkOakFenceGate
            | BlockKind::SpruceDoor
            | BlockKind::BirchDoor
            | BlockKind::JungleDoor
            | BlockKind::AcaciaDoor
            | BlockKind::DarkOakDoor => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `part` property."]
    pub fn has_part(self) -> bool {
        match self.kind() {
            BlockKind::WhiteBed
            | BlockKind::OrangeBed
            | BlockKind::MagentaBed
            | BlockKind::LightBlueBed
            | BlockKind::YellowBed
            | BlockKind::LimeBed
            | BlockKind::PinkBed
            | BlockKind::GrayBed
            | BlockKind::LightGrayBed
            | BlockKind::CyanBed
            | BlockKind::PurpleBed
            | BlockKind::BlueBed
            | BlockKind::BrownBed
            | BlockKind::GreenBed
            | BlockKind::RedBed
            | BlockKind::BlackBed => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `persistent` property."]
    pub fn has_persistent(self) -> bool {
        match self.kind() {
            BlockKind::OakLeaves
            | BlockKind::SpruceLeaves
            | BlockKind::BirchLeaves
            | BlockKind::JungleLeaves
            | BlockKind::AcaciaLeaves
            | BlockKind::DarkOakLeaves => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `pickles` property."]
    pub fn has_pickles(self) -> bool {
        match self.kind() {
            BlockKind::SeaPickle => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `piston_kind` property."]
    pub fn has_piston_kind(self) -> bool {
        match self.kind() {
            BlockKind::PistonHead | BlockKind::MovingPiston => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `power` property."]
    pub fn has_power(self) -> bool {
        match self.kind() {
            BlockKind::RedstoneWire
            | BlockKind::LightWeightedPressurePlate
            | BlockKind::HeavyWeightedPressurePlate
            | BlockKind::DaylightDetector => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `powered` property."]
    pub fn has_powered(self) -> bool {
        match self.kind() {
            BlockKind::NoteBlock
            | BlockKind::PoweredRail
            | BlockKind::DetectorRail
            | BlockKind::OakDoor
            | BlockKind::Lever
            | BlockKind::StonePressurePlate
            | BlockKind::IronDoor
            | BlockKind::OakPressurePlate
            | BlockKind::SprucePressurePlate
            | BlockKind::BirchPressurePlate
            | BlockKind::JunglePressurePlate
            | BlockKind::AcaciaPressurePlate
            | BlockKind::DarkOakPressurePlate
            | BlockKind::StoneButton
            | BlockKind::Repeater
            | BlockKind::OakTrapdoor
            | BlockKind::SpruceTrapdoor
            | BlockKind::BirchTrapdoor
            | BlockKind::JungleTrapdoor
            | BlockKind::AcaciaTrapdoor
            | BlockKind::DarkOakTrapdoor
            | BlockKind::OakFenceGate
            | BlockKind::TripwireHook
            | BlockKind::Tripwire
            | BlockKind::OakButton
            | BlockKind::SpruceButton
            | BlockKind::BirchButton
            | BlockKind::JungleButton
            | BlockKind::AcaciaButton
            | BlockKind::DarkOakButton
            | BlockKind::Comparator
            | BlockKind::ActivatorRail
            | BlockKind::IronTrapdoor
            | BlockKind::SpruceFenceGate
            | BlockKind::BirchFenceGate
            | BlockKind::JungleFenceGate
            | BlockKind::AcaciaFenceGate
            | BlockKind::DarkOakFenceGate
            | BlockKind::SpruceDoor
            | BlockKind::BirchDoor
            | BlockKind::JungleDoor
            | BlockKind::AcaciaDoor
            | BlockKind::DarkOakDoor
            | BlockKind::Observer => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `powered_rail_shape` property."]
    pub fn has_powered_rail_shape(self) -> bool {
        match self.kind() {
            BlockKind::PoweredRail | BlockKind::DetectorRail | BlockKind::ActivatorRail => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `rail_shape` property."]
    pub fn has_rail_shape(self) -> bool {
        match self.kind() {
            BlockKind::Rail => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `rotation` property."]
    pub fn has_rotation(self) -> bool {
        match self.kind() {
            BlockKind::Sign
            | BlockKind::SkeletonSkull
            | BlockKind::WitherSkeletonSkull
            | BlockKind::ZombieHead
            | BlockKind::PlayerHead
            | BlockKind::CreeperHead
            | BlockKind::DragonHead
            | BlockKind::WhiteBanner
            | BlockKind::OrangeBanner
            | BlockKind::MagentaBanner
            | BlockKind::LightBlueBanner
            | BlockKind::YellowBanner
            | BlockKind::LimeBanner
            | BlockKind::PinkBanner
            | BlockKind::GrayBanner
            | BlockKind::LightGrayBanner
            | BlockKind::CyanBanner
            | BlockKind::PurpleBanner
            | BlockKind::BlueBanner
            | BlockKind::BrownBanner
            | BlockKind::GreenBanner
            | BlockKind::RedBanner
            | BlockKind::BlackBanner => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `short` property."]
    pub fn has_short(self) -> bool {
        match self.kind() {
            BlockKind::PistonHead => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `slab_kind` property."]
    pub fn has_slab_kind(self) -> bool {
        match self.kind() {
            BlockKind::PrismarineSlab
            | BlockKind::PrismarineBrickSlab
            | BlockKind::DarkPrismarineSlab
            | BlockKind::OakSlab
            | BlockKind::SpruceSlab
            | BlockKind::BirchSlab
            | BlockKind::JungleSlab
            | BlockKind::AcaciaSlab
            | BlockKind::DarkOakSlab
            | BlockKind::StoneSlab
            | BlockKind::SandstoneSlab
            | BlockKind::PetrifiedOakSlab
            | BlockKind::CobblestoneSlab
            | BlockKind::BrickSlab
            | BlockKind::StoneBrickSlab
            | BlockKind::NetherBrickSlab
            | BlockKind::QuartzSlab
            | BlockKind::RedSandstoneSlab
            | BlockKind::PurpurSlab => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `snowy` property."]
    pub fn has_snowy(self) -> bool {
        match self.kind() {
            BlockKind::GrassBlock | BlockKind::Podzol | BlockKind::Mycelium => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `south_connected` property."]
    pub fn has_south_connected(self) -> bool {
        match self.kind() {
            BlockKind::Fire
            | BlockKind::OakFence
            | BlockKind::BrownMushroomBlock
            | BlockKind::RedMushroomBlock
            | BlockKind::MushroomStem
            | BlockKind::IronBars
            | BlockKind::GlassPane
            | BlockKind::Vine
            | BlockKind::NetherBrickFence
            | BlockKind::Tripwire
            | BlockKind::CobblestoneWall
            | BlockKind::MossyCobblestoneWall
            | BlockKind::WhiteStainedGlassPane
            | BlockKind::OrangeStainedGlassPane
            | BlockKind::MagentaStainedGlassPane
            | BlockKind::LightBlueStainedGlassPane
            | BlockKind::YellowStainedGlassPane
            | BlockKind::LimeStainedGlassPane
            | BlockKind::PinkStainedGlassPane
            | BlockKind::GrayStainedGlassPane
            | BlockKind::LightGrayStainedGlassPane
            | BlockKind::CyanStainedGlassPane
            | BlockKind::PurpleStainedGlassPane
            | BlockKind::BlueStainedGlassPane
            | BlockKind::BrownStainedGlassPane
            | BlockKind::GreenStainedGlassPane
            | BlockKind::RedStainedGlassPane
            | BlockKind::BlackStainedGlassPane
            | BlockKind::SpruceFence
            | BlockKind::BirchFence
            | BlockKind::JungleFence
            | BlockKind::AcaciaFence
            | BlockKind::DarkOakFence
            | BlockKind::ChorusPlant => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `south_wire` property."]
    pub fn has_south_wire(self) -> bool {
        match self.kind() {
            BlockKind::RedstoneWire => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `stage` property."]
    pub fn has_stage(self) -> bool {
        match self.kind() {
            BlockKind::OakSapling
            | BlockKind::SpruceSapling
            | BlockKind::BirchSapling
            | BlockKind::JungleSapling
            | BlockKind::AcaciaSapling
            | BlockKind::DarkOakSapling => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `stairs_shape` property."]
    pub fn has_stairs_shape(self) -> bool {
        match self.kind() {
            BlockKind::OakStairs
            | BlockKind::CobblestoneStairs
            | BlockKind::BrickStairs
            | BlockKind::StoneBrickStairs
            | BlockKind::NetherBrickStairs
            | BlockKind::SandstoneStairs
            | BlockKind::SpruceStairs
            | BlockKind::BirchStairs
            | BlockKind::JungleStairs
            | BlockKind::QuartzStairs
            | BlockKind::AcaciaStairs
            | BlockKind::DarkOakStairs
            | BlockKind::PrismarineStairs
            | BlockKind::PrismarineBrickStairs
            | BlockKind::DarkPrismarineStairs
            | BlockKind::RedSandstoneStairs
            | BlockKind::PurpurStairs => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `structure_block_mode` property."]
    pub fn has_structure_block_mode(self) -> bool {
        match self.kind() {
            BlockKind::StructureBlock => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `triggered` property."]
    pub fn has_triggered(self) -> bool {
        match self.kind() {
            BlockKind::Dispenser | BlockKind::Dropper => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `unstable` property."]
    pub fn has_unstable(self) -> bool {
        match self.kind() {
            BlockKind::Tnt => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `up` property."]
    pub fn has_up(self) -> bool {
        match self.kind() {
            BlockKind::Fire
            | BlockKind::BrownMushroomBlock
            | BlockKind::RedMushroomBlock
            | BlockKind::MushroomStem
            | BlockKind::Vine
            | BlockKind::CobblestoneWall
            | BlockKind::MossyCobblestoneWall
            | BlockKind::ChorusPlant => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `water_level` property."]
    pub fn has_water_level(self) -> bool {
        match self.kind() {
            BlockKind::Water | BlockKind::Lava => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `waterlogged` property."]
    pub fn has_waterlogged(self) -> bool {
        match self.kind() {
            BlockKind::OakStairs
            | BlockKind::Chest
            | BlockKind::Sign
            | BlockKind::Ladder
            | BlockKind::CobblestoneStairs
            | BlockKind::WallSign
            | BlockKind::OakFence
            | BlockKind::OakTrapdoor
            | BlockKind::SpruceTrapdoor
            | BlockKind::BirchTrapdoor
            | BlockKind::JungleTrapdoor
            | BlockKind::AcaciaTrapdoor
            | BlockKind::DarkOakTrapdoor
            | BlockKind::IronBars
            | BlockKind::GlassPane
            | BlockKind::BrickStairs
            | BlockKind::StoneBrickStairs
            | BlockKind::NetherBrickFence
            | BlockKind::NetherBrickStairs
            | BlockKind::SandstoneStairs
            | BlockKind::EnderChest
            | BlockKind::SpruceStairs
            | BlockKind::BirchStairs
            | BlockKind::JungleStairs
            | BlockKind::CobblestoneWall
            | BlockKind::MossyCobblestoneWall
            | BlockKind::TrappedChest
            | BlockKind::QuartzStairs
            | BlockKind::WhiteStainedGlassPane
            | BlockKind::OrangeStainedGlassPane
            | BlockKind::MagentaStainedGlassPane
            | BlockKind::LightBlueStainedGlassPane
            | BlockKind::YellowStainedGlassPane
            | BlockKind::LimeStainedGlassPane
            | BlockKind::PinkStainedGlassPane
            | BlockKind::GrayStainedGlassPane
            | BlockKind::LightGrayStainedGlassPane
            | BlockKind::CyanStainedGlassPane
            | BlockKind::PurpleStainedGlassPane
            | BlockKind::BlueStainedGlassPane
            | BlockKind::BrownStainedGlassPane
            | BlockKind::GreenStainedGlassPane
            | BlockKind::RedStainedGlassPane
            | BlockKind::BlackStainedGlassPane
            | BlockKind::AcaciaStairs
            | BlockKind::DarkOakStairs
            | BlockKind::IronTrapdoor
            | BlockKind::PrismarineStairs
            | BlockKind::PrismarineBrickStairs
            | BlockKind::DarkPrismarineStairs
            | BlockKind::PrismarineSlab
            | BlockKind::PrismarineBrickSlab
            | BlockKind::DarkPrismarineSlab
            | BlockKind::RedSandstoneStairs
            | BlockKind::OakSlab
            | BlockKind::SpruceSlab
            | BlockKind::BirchSlab
            | BlockKind::JungleSlab
            | BlockKind::AcaciaSlab
            | BlockKind::DarkOakSlab
            | BlockKind::StoneSlab
            | BlockKind::SandstoneSlab
            | BlockKind::PetrifiedOakSlab
            | BlockKind::CobblestoneSlab
            | BlockKind::BrickSlab
            | BlockKind::StoneBrickSlab
            | BlockKind::NetherBrickSlab
            | BlockKind::QuartzSlab
            | BlockKind::RedSandstoneSlab
            | BlockKind::PurpurSlab
            | BlockKind::SpruceFence
            | BlockKind::BirchFence
            | BlockKind::JungleFence
            | BlockKind::AcaciaFence
            | BlockKind::DarkOakFence
            | BlockKind::PurpurStairs
            | BlockKind::DeadTubeCoral
            | BlockKind::DeadBrainCoral
            | BlockKind::DeadBubbleCoral
            | BlockKind::DeadFireCoral
            | BlockKind::DeadHornCoral
            | BlockKind::TubeCoral
            | BlockKind::BrainCoral
            | BlockKind::BubbleCoral
            | BlockKind::FireCoral
            | BlockKind::HornCoral
            | BlockKind::DeadTubeCoralWallFan
            | BlockKind::DeadBrainCoralWallFan
            | BlockKind::DeadBubbleCoralWallFan
            | BlockKind::DeadFireCoralWallFan
            | BlockKind::DeadHornCoralWallFan
            | BlockKind::TubeCoralWallFan
            | BlockKind::BrainCoralWallFan
            | BlockKind::BubbleCoralWallFan
            | BlockKind::FireCoralWallFan
            | BlockKind::HornCoralWallFan
            | BlockKind::DeadTubeCoralFan
            | BlockKind::DeadBrainCoralFan
            | BlockKind::DeadBubbleCoralFan
            | BlockKind::DeadFireCoralFan
            | BlockKind::DeadHornCoralFan
            | BlockKind::TubeCoralFan
            | BlockKind::BrainCoralFan
            | BlockKind::BubbleCoralFan
            | BlockKind::FireCoralFan
            | BlockKind::HornCoralFan
            | BlockKind::SeaPickle
            | BlockKind::Conduit => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `west_connected` property."]
    pub fn has_west_connected(self) -> bool {
        match self.kind() {
            BlockKind::Fire
            | BlockKind::OakFence
            | BlockKind::BrownMushroomBlock
            | BlockKind::RedMushroomBlock
            | BlockKind::MushroomStem
            | BlockKind::IronBars
            | BlockKind::GlassPane
            | BlockKind::Vine
            | BlockKind::NetherBrickFence
            | BlockKind::Tripwire
            | BlockKind::CobblestoneWall
            | BlockKind::MossyCobblestoneWall
            | BlockKind::WhiteStainedGlassPane
            | BlockKind::OrangeStainedGlassPane
            | BlockKind::MagentaStainedGlassPane
            | BlockKind::LightBlueStainedGlassPane
            | BlockKind::YellowStainedGlassPane
            | BlockKind::LimeStainedGlassPane
            | BlockKind::PinkStainedGlassPane
            | BlockKind::GrayStainedGlassPane
            | BlockKind::LightGrayStainedGlassPane
            | BlockKind::CyanStainedGlassPane
            | BlockKind::PurpleStainedGlassPane
            | BlockKind::BlueStainedGlassPane
            | BlockKind::BrownStainedGlassPane
            | BlockKind::GreenStainedGlassPane
            | BlockKind::RedStainedGlassPane
            | BlockKind::BlackStainedGlassPane
            | BlockKind::SpruceFence
            | BlockKind::BirchFence
            | BlockKind::JungleFence
            | BlockKind::AcaciaFence
            | BlockKind::DarkOakFence
            | BlockKind::ChorusPlant => true,
            _ => false,
        }
    }
    #[doc = "Determines whether or not a block has the `west_wire` property."]
    pub fn has_west_wire(self) -> bool {
        match self.kind() {
            BlockKind::RedstoneWire => true,
            _ => false,
        }
    }
}
