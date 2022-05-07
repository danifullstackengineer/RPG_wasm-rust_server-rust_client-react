use serde::{Deserialize, Serialize};

// General Item structure.
#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    // Name of the item
    name: String,
    // Description of the item
    description: String,
    // If it is equippable then it is Some(Equippable). Not consumable.
    equip: Option<Equippable>,
    // If it is consumable then it is Some(Consumable). Not equippable.
    consum: Option<Consumable>
}
// General Item status structure. Similar to status of player, however
// different items give different combinations of status
// ex: something might only give py_attack + mg_attack
#[derive(Deserialize, Serialize, Debug)]
struct ItemStatus {
    py_attack: Option<u16>,
    mg_attack: Option<u16>,
    defence: Option<u16>,
    health: Option<u16>,
    speed: Option<u16>,
}
// Struct for Equippable.
#[derive(Deserialize, Serialize, Debug)]
struct Equippable {
    // The status that this item gives when equipped.
    status: ItemStatus,
    // The quality of this item.
    quality: EquippableQuality
}
// Different types of 'quality' exist.
#[derive(Deserialize, Serialize, Debug)]
struct EquippableQuality {
    // Tier goes from 1 up to 5. Is improved by raising level to 9 and upgrading.
    tier: ItemTier,
    // Level goes from 1 up to 9.
    // At level 9 when upgrading it increases tier by a factor of 1.
    level: ItemLevel,
    // Quality goes from 1 up to 3. It is improved by raising tier to 5
    // and upgrading. It is optional because only weapons have this.
    quality: Option<ItemQuality>
}
#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize, Debug)]
enum ItemTier {
    ONE_t,
    TWO_t,
    THREE_t,
    FOUR_t,
    FIVE_t
}
#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize, Debug)]
enum ItemLevel {
    ONE_L,
    TWO_L,
    Three_L,
    FOUR_l,
    FIVE_l,
    SIX_l,
    SEVEN_l,
    EIGHT_l,
    NINE_l
}
#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize, Debug)]
enum ItemQuality {
    ONE_q,
    TWO_q,
    THREE_q
}
// Consumable struct.
#[derive(Deserialize, Serialize, Debug)]
struct Consumable {
    // The status the consumable gives. Optional because some items,
    // such as chests, do not give status effects.
    status_effect: Option<ItemStatus>,
    // The tier of the consumable.
    tier: ItemTier,
    // Time when effect is valid. After time expires, effect is no
    // longer valid.
    time: Option<u16>
}