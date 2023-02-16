//! Contains the [Health] and [Damage] types for representing HP

use std::{ops::{Sub, Add, SubAssign, AddAssign}, fmt::Display};

/// The health of the player or an enemy
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Health (usize);

/// A change in [Health]. Note that it is unsigned - a [Damage] could represent healing as well, depending on the context.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Damage(usize);

impl Health {
    /// Creates a new [Health] from a number of HP
    pub const fn new(health: usize) -> Self {
        Self(health)
    }

    /// Checks whether the health is 0. Used to calculate whether the player or an enemy has lost a battle.
    pub const fn is_0(self) -> bool {
        self.0 == 0
    }

    /// Increases the [Health] by the given [Damage], up to the given max health.
    /// Returns how much the health increased by.
    pub fn heal_to_max(&mut self, heal_by: Damage, max: Self) -> Damage {
        let new_health = max.0.min(self.0 + heal_by.0);
        let diff = new_health - self.0;
        self.0 = new_health;
        Damage::new(diff)
    }

    /// Gets the value of the health as a [usize]. This is needed to do more advanced calculations than just adding and subtracting [Damage] values.
    pub const fn as_usize(self) -> usize {
        self.0
    }
}

impl Damage {
    /// Creates a new [Damage] from a change in HP
    pub const fn new(health: usize) -> Self {
        Self(health)
    }
}

impl Sub<Damage> for Health {
    type Output = Self;

    fn sub(self, rhs: Damage) -> Self::Output {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl Add<Damage> for Health {
    type Output = Self;

    fn add(self, rhs: Damage) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub<Self> for Health {
    type Output = Damage;

    fn sub(self, rhs: Self) -> Self::Output {
        Damage(self.0 - rhs.0)
    }
}

impl SubAssign<Damage> for Health {
    fn sub_assign(&mut self, rhs: Damage) {
        self.0 = self.0.saturating_sub(rhs.0);
    }
}

impl AddAssign<Damage> for Health {
    fn add_assign(&mut self, rhs: Damage) {
        self.0 += rhs.0;
    }
}

impl Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Damage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
