//! Functions to create [`Weapon`] items

use crate::{items::{Item, Weapon}, combat::Damage};


/// Creates a new 'captain's blaster' item
pub(super) fn captains_blaster() -> Item {
    Item::Weapon(Weapon {
        name: "Captain's blaster",
        description: "An energy weapon which the captain keeps by his command chair in case of emergency",

        straight_damage: Damage::new(5),
        dodge_damage: Damage::new(5),
        speed: 2
    })
}