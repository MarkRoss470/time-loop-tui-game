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

/// Creates a new 'bar of chocolate' item
pub(super) const fn bar_of_chocolate() -> Item {
    Item::Food(Food {
        name: "Bar of Chocolate",
        description: "A bar of dark chocolate. It says on the label that it's made from real cacao, bred from plants that trace their lineage all the way back to Earth!",
        heals_for: Damage::new(10),
    })
}