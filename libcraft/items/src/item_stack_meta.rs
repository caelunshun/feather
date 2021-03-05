/// All the possible meta tag groups that an
/// `ItemStack` can have.
pub enum ItemMetaTag {
    General(GeneralTags),
    Block,
    Enchantments,
    AttributeModifiers,
    PotionEffects,
    Crossbows,
    DisplayProperties,
    WrittenBooks,
    BooksAndQuills,
    PlayerHeads,
    Fireworks,
    ArmorStandsSpawnEggs,
    FishBuckets,
    Maps,
    SuspiciousStew,
    DebugSticks,
    Compasses,
}

/// Contains the general NBT tags.
/// * Damage to the item
/// * If the item is unbreakable
/// * A list of blocks that the item can destroy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GeneralTags {

    /// The damage done to the item.
    damage: Option<u32>,

    /// If the item is unbreakable or not.
    unbreakable: Option<bool>,

    /// List of blocks that the item can destroy in
    /// adventure mode.
    can_destroy: Option<Vec<String>>,
    // TODO Investigate what CustomModelData is.
}