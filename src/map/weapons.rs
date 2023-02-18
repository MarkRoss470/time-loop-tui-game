//! Functions to create [`Weapon`] items

use crate::{
    combat::Damage,
    items::{Item, Weapon},
};

/// Creates a new 'intruders blaster' item
pub(super) const fn intruders_blaster() -> Item {
    Item::Weapon(Weapon {
        name: "Intruders Blaster",
        description: "An energy weapon kept on the wall in the bridge to use if an enemy boards the ship.",

        straight_damage: Damage::new(5),
        dodge_damage: Damage::new(3),
        speed: 3,
    })
}

/// Creates a new 'captain's blaster' item
pub(super) const fn captains_blaster() -> Item {
    Item::Weapon(Weapon {
        name: "Captain's Blaster",
        description: "An energy weapon which the captain keeps by their side through sunshine and rain, through of course they've seen neither in a long time.",

        straight_damage: Damage::new(7),
        dodge_damage: Damage::new(5),
        speed: 3,
    })
}

/// Creates a new 'standard blaster' item
pub(super) const fn standard_blaster() -> Item {
    Item::Weapon(Weapon {
        name: "Standard Issue Blaster",
        description: "The blaster issued to every serving troop. It's slow, but well made.",

        straight_damage: Damage::new(5),
        dodge_damage: Damage::new(2),
        speed: 4,
    })
}

/// Creates a new 'ISPD taser' item
pub(super) const fn ispd_taser() -> Item {
    Item::Weapon(Weapon {
        name: "ISPD Taser",
        description: "A high-powered taser given to every officer in the Interstellar Police Department. It's fast and lethal if you're not careful (or if you are).",

        straight_damage: Damage::new(10),
        dodge_damage: Damage::new(5),
        speed: 2
    })
}

/// Creates a new 'throwing dart set' item
pub(super) const fn throwing_dart_set() -> Item {
    Item::Weapon(Weapon {
        name: "Set of Throwing Darts",
        description: "A set of sharp darts from the darts set in the bunks. They're not too sharp, but you can throw them fast as anything.",

        straight_damage: Damage::new(2),
        dodge_damage: Damage::new(2),
        speed: 1
    })
}

/// Creates a new 'shaving razor' item
pub(super) const fn shaving_razor() -> Item {
    Item::Weapon(Weapon {
        name: "Shaving Razor",
        description: "A razor you found in the wash room. It's sharp, but it's not really a weapon.",

        straight_damage: Damage::new(3),
        dodge_damage: Damage::new(2),
        speed: 5
    })
}


/// Creates a new 'wrench' item
pub(super) const fn wrench() -> Item {
    Item::Weapon(Weapon {
        name: "Wrench",
        description: "A wrench from the engine room. It's weighty and you could do some good damage with it.",

        straight_damage: Damage::new(6),
        dodge_damage: Damage::new(4),
        speed: 3
    })
}


/// Creates a new 'eating knife' item
pub(super) const fn eating_knife() -> Item {
    Item::Weapon(Weapon {
        name: "Eating Knife",
        description: "A sharp steel knife. Synthetic protein is tough, so it's sharp and sturdy",

        straight_damage: Damage::new(5),
        dodge_damage: Damage::new(5),
        speed: 2    
    })
}