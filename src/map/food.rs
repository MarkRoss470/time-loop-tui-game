//! functions to create [`Food`] items

use crate::{
    combat::Damage,
    items::{Food, Item},
};

/// Creates a new 'bread roll' item
pub(super) const fn bread_roll() -> Item {
    Item::Food(Food {
        name: "Bread roll",
        description: "A soft white bread roll. It's tasty, but not substantial.",
        heals_for: Damage::new(5),
    })
}
