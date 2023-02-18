//! Functionality related to items

use crate::combat::Damage;

/// A food item which heals the player when used
#[derive(Debug, Hash)]
pub struct Food {
    /// The name of the food
    pub name: &'static str,
    /// A description of the food
    pub description: &'static str,
    /// How much health the player or an enemy gains by eating the food
    pub heals_for: Damage,
}

/// A weapon which can be used in a battle
#[derive(Debug, Hash)]
pub struct Weapon {
    /// The name of the weapon
    pub name: &'static str,
    /// A description of the weapon
    pub description: &'static str,

    /// How much damage the weapon deals if it hits an opponent who didn't dodge
    pub straight_damage: Damage,
    /// How much damage the weapon deals if it hits an opponent who dodged
    pub dodge_damage: Damage,
    /// The weapon's speed. A lower speed means the weapon will act faster.
    pub speed: usize,
}

/// An item which can be stored in the [player][crate::player::Player]'s or an [enemy][crate::combat::Enemy]'s inventory
#[derive(Debug, Hash)]
pub enum Item {
    /// A food item
    Food(Food),
    /// A weapon
    Weapon(Weapon),
    /// The maps which are needed to fly the escape pod
    Maps,
    /// The keys to the escape pod
    EscapePodKeys,
}

impl Item {
    /// Gets the name of the item
    pub const fn get_name(&self) -> &'static str {
        match self {
            Self::Food(f) => f.name,
            Self::Weapon(w) => w.name,
            Self::Maps => "Galactic Maps 2168 Edition",
            Self::EscapePodKeys => "Escape Pod Keys"
        }
    }

    /// Gets the description of the item
    pub const fn get_description(&self) -> &'static str {
        match self {
            Self::Food(f) => f.description,
            Self::Weapon(w) => w.description,
            Self::Maps => "A map of the galaxy in the format which spacecraft use to plot routes",
            Self::EscapePodKeys => "A key card labelled 'escape pod'. The label is beginning to wear.",
        }
    }
}
