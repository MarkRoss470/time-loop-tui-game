#![cfg(test)]

use crate::{items::Food, combat::Damage, menu::tests::MockMenu};

use super::*;

/// Tests that the [`Player::get_remaining_time`] function returns correct results
#[test]
fn test_time_format() {
    let mut player = Player::init();

    player.remaining_turns = 0;
    assert_eq!(player.get_remaining_time(), "0:00");

    player.remaining_turns = 1;
    assert_eq!(player.get_remaining_time(), "0:20");

    player.remaining_turns = 3;
    assert_eq!(player.get_remaining_time(), "1:00");

    player.remaining_turns = 5;
    assert_eq!(player.get_remaining_time(), "1:40");

    player.remaining_turns = 10;
    assert_eq!(player.get_remaining_time(), "3:20");
}

#[test]
fn test_use_item() {
    // Eating food should heal by the right number of health
    {
        let mut player = Player::init();
        player.health = Health::new(5);
        player.max_health = Health::new(10);

        player.inventory.push(Item::Food(Food {
        name: "",
            description: "",
            heals_for: Damage::new(3),
        }));

        player.use_item(&mut MockMenu::default(), 0);
        assert_eq!(player.health, Health::new(8));
    }

    // Eating food should not heal past the player's maximum health
    {
        let mut player = Player::init();
        player.health = Health::new(5);
        player.max_health = Health::new(10);

        player.inventory.push(Item::Food(Food {
        name: "",
            description: "",
            heals_for: Damage::new(10),
        }));

        player.use_item(&mut MockMenu::new().unwrap(), 0);
        assert_eq!(player.health, Health::new(10));
    }
}