//! Functions to create [`Weapon`] items

use crate::{items::{Item, Weapon}, combat::Damage};


/// Creates a new 'captain's blaster' item
pub(super) const fn captains_blaster() -> Item {
    Item::Weapon(Weapon {
        name: "Captain's blaster",
        description: "An energy weapon which the captain keeps by his command chair in case of emergency",

        straight_damage: Damage::new(7),
        dodge_damage: Damage::new(5),
        speed: 3
    })
}

/// Creates a new 'standard blaster' item
pub(super) const fn standard_blaster() -> Item {
    Item::Weapon(Weapon {
        name: "Standard issue blaster",
        description: "The blaster issued to every serving troop. It's slow, but well made.",

        straight_damage: Damage::new(5),
        dodge_damage: Damage::new(2),
        speed: 4
    })
}

/// Creates a new 'ISPD taser' item
pub(super) const fn ispd_taser() -> Item {
    Item::Weapon(Weapon {
        name: "ISPD taser",
        description: "A high-powered taser given to every officer in the Interstellar Police Department. It's fast and lethal if you're not careful (or if you are).",

        straight_damage: Damage::new(10),
        dodge_damage: Damage::new(5),
        speed: 2
    })
}

/// Creates a new 'throwing dart set' item
pub(super) const fn throwing_dart_set() -> Item {
    Item::Weapon(Weapon {
        name: "Set of throwing darts",
        description: "A set of sharp darts from the darts set in the bunks. They're not too sharp, but you can throw them fast as anything.",

        straight_damage: Damage::new(2),
        dodge_damage: Damage::new(2),
        speed: 1
    })
}