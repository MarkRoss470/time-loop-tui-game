#[derive(Debug)]
pub struct Food {
    pub name: &'static str,
    pub description: &'static str,
    pub heals_for: usize,
}

#[derive(Debug)]
pub struct Weapon {
    pub name: &'static str,
    pub description: &'static str,
    pub damage: usize
}

#[derive(Debug)]
pub enum Item {
    Food(Food),
    Weapon(Weapon)
}

impl Item {
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::Food(f) => f.name,
            Self::Weapon(w) => w.name,
        }
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            Self::Food(f) => f.description,
            Self::Weapon(w) => w.description,
        }
    }
}