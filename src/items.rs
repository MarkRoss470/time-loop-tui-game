use crate::combat::Damage;

#[derive(Debug, Hash)]
pub struct Food {
    /// The name of the food
    pub name: &'static str,
    /// A description of the food
    pub description: &'static str,
    /// How much health the player or an enemy gains by eating the food
    pub heals_for: Damage,
}

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

#[derive(Debug, Hash)]
pub enum Item {
    Food(Food),
    Weapon(Weapon)
}

impl Item {
    /// Gets the name of the item
    pub const fn get_name(&self) -> &'static str {
        match self {
            Self::Food(f) => f.name,
            Self::Weapon(w) => w.name,
        }
    }

    /// Gets the description of the item
    pub const fn get_description(&self) -> &'static str {
        match self {
            Self::Food(f) => f.description,
            Self::Weapon(w) => w.description,
        }
    }
}