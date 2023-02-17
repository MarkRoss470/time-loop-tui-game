//! Functions to create [enemies][Enemy]

use crate::combat::{Enemy, Health};

use super::weapons;

/// Creates a new 'cook' enemy
pub(super) fn cook() -> Enemy {
    Enemy {
        name: "Cook",
        description: "The ship's cook. There's not much to do when there aren't any troops, so they're sitting around watching the game.",
        inventory: vec![weapons::standard_blaster()],

        health: Health::new(7),
        max_health: Health::new(7),
    }
}

/// Creates a new 'mechanic' enemy
pub(super) fn mechanic() -> Enemy {
    Enemy {
        name: "Mechanic",
        description: "The ship's mechanic. They check the ship every cycle and fix anything that's broken. They were previously a high ranking ISPD officer and still carry a taser everywhere with them. \
At the moment they're checking the ship's comms, while listening to music through a pair of particularly bulky headphones. Bad practice, of course, but you don't mind.",
        inventory: vec![weapons::ispd_taser()],

        health: Health::new(10),
        max_health: Health::new(10),
    }
}

/// Creates a new 'skipper' enemy
pub(super) fn skipper() -> Enemy {
    Enemy {
        name: "Skipper",
        description: "The ship's captain. Having served in the 2143-2152 inter-system war, they have great experience in combat. On the other hand, they're very good at forgetting things.",
        inventory: vec![],

        health: Health::new(15),
        max_health: Health::new(15),
    }
}
