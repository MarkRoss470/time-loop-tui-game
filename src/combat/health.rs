use std::{ops::{Sub, Add, SubAssign, AddAssign}, fmt::Display};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Health (usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Damage(usize);

impl Health {
    pub const fn new(health: usize) -> Self {
        Self(health)
    }

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

    pub const fn as_usize(self) -> usize {
        self.0
    }
}

impl Damage {
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
