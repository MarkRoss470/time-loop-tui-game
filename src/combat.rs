mod health;

use std::{cmp::Ordering, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use crate::{items::Item, player::Player, menu::{Menu, Screen}};

pub use health::{Health, Damage};

#[derive(Debug, Hash)]
pub struct Enemy {
    pub name: &'static str,
    pub description: &'static str,
    
    pub inventory: Vec<Item>,
    pub health: Health,
    pub max_health: Health,
}

impl Enemy {
    pub fn describe_combat_action(&self, action: Action) -> String {
        use Action::*;

        match action {
            AttackLeft(w) => format!("The {} attacks to the left with their {}", self.name, self.inventory[w].get_name()),
            AttackRight(w) => format!("The {} attacks to the right with their {}", self.name, self.inventory[w].get_name()),
            AttackStraight(w) => format!("The {} attacks in front of you with their {}", self.name, self.inventory[w].get_name()),
            EatFood(f) => format!("The {} attempts to eat their {}", self.name, self.inventory[f].get_name()),
            
            DodgeLeft => format!("The {} dodges to the left", self.name),
            DodgeRight => format!("The {} dodges to the right", self.name),
            Nothing => format!("The {} does nothing", self.name),
        }
    }
}

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleResult {
    PlayerWin,
    PlayerLoss,
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Nothing,
    EatFood(usize),
    AttackStraight(usize),
    AttackLeft(usize),
    AttackRight(usize),
    DodgeLeft,
    DodgeRight
}

impl Enemy {
    fn hash_with_turn(&self, turn_number: usize) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        turn_number.hash(&mut s);
        s.finish()
    }

    fn choose_combat_action(&mut self, turn_number: usize) -> Action {
        // If enemy is at less than half health and has food, then eat it
        if self.health.as_usize() * 2 <= self.max_health.as_usize() {
            if let Some(food_index) = self.inventory.iter().position(|i|matches!(i, Item::Food(_))) {
                return Action::EatFood(food_index);
            }
        }

        // Find the index of the first weapon in the inventory, if there is one
        let weapon_index = self.inventory.iter().position(|i|matches!(i, Item::Weapon(_)));
        // Get a hash of self using the turn number
        let hash = self.hash_with_turn(turn_number);
        
        // Pseudorandomly pick an action
        match weapon_index {
            Some(weapon_index) => match hash % 8 {
                0 => Action::AttackLeft(weapon_index),
                1..=3 => Action::AttackStraight(weapon_index),
                4 => Action::AttackRight(weapon_index),
                5 => Action::DodgeLeft,
                6 => Action::DodgeRight,
                7 => Action::Nothing,
                _ => unreachable!()
            }
            None => {
                match hash % 7 {
                    0..=1 => Action::DodgeLeft,
                    2..=4 => Action::Nothing,
                    5..=6 => Action::DodgeRight,
                    _ => unreachable!()
                }
            }
        }
     }
}

pub fn battle(player: &mut Player, mut enemy: Enemy, turn_number: &mut usize, menu: &mut impl Menu) -> BattleResult {
    let screen = Screen {
        title: &format!("You are spotted by {}", enemy.name),
        content: &format!("The {} sees you and blocks your path.", enemy.description),
    };

    menu.show_screen(screen);

    // Loop until either the player or the enemy reaches 0 health
    loop {
        // Get the player and enemy's actions
        let player_action = player.choose_combat_action(menu);
        let enemy_action = enemy.choose_combat_action(*turn_number);

        // Carry out the actions
        let turn_text = execute_actions(player, &mut enemy, player_action, enemy_action);

        // Show the result of the turn
        let turn_text = format!("{turn_text}\nYou are now at {}/{} HP.\nThe {} is now at {}/{} HP",
            player.health, player.max_health,
            enemy.name, enemy.health, enemy.max_health,
        );

        let screen = Screen {
            title: "Turn Result",
            content: &turn_text,
        };

        menu.show_screen(screen);

        if player.health.is_0() {
            return BattleResult::PlayerLoss;
        }
        if enemy.health.is_0() {
            win_battle(player, enemy, menu);
            return  BattleResult::PlayerWin;
        }

        *turn_number += 1;
    }
}

fn win_battle(player: &mut Player, enemy: Enemy, menu: &mut impl Menu) {
    let mut result_text = "You won the battle!\n\n".to_string();

    if !enemy.inventory.is_empty() {
        result_text += &format!("You pick up the items that the {} was carrying:\n", enemy.name);
    }

    for item in &enemy.inventory {
        result_text += &format!("• {} - {}\n", item.get_name(), item.get_description());
    }

    let screen = Screen {
        title: "Battle Result",
        content: &result_text
    };

    menu.show_screen(screen);

    for item in enemy.inventory {
        player.pick_up_item(item);
    }
}

fn execute_actions(player: &mut Player, enemy: &mut Enemy, player_action: Action, enemy_action: Action) -> String {
    use Action::*;

    // Take the turn
    let result_text = match (player_action, enemy_action) {
        // Player hits enemy straight
        (AttackStraight(p), Nothing | AttackLeft(_) | AttackRight(_) | EatFood(_)) => {
            let Item::Weapon(weapon) = &player.inventory[p] else {unreachable!()};
            let damage = weapon.straight_damage;
            enemy.health -= damage;

            format!("You hit the {} with your {} and dealt {} damage.", enemy.name, weapon.name, damage)
        }
        // Enemy hits player straight
        (Nothing | AttackLeft(_) | AttackRight(_) | EatFood(_), AttackStraight(e)) => {
            let Item::Weapon(weapon) = &enemy.inventory[e] else {unreachable!()};
            let damage = weapon.straight_damage;
            player.health -= damage;

            format!("You hit the {} with your {} and dealt {} damage.", enemy.name, weapon.name, damage)
        }
        // Both attack straight
        (AttackStraight(p), AttackStraight(e)) => {
            let Item::Weapon(p_weapon) = &player.inventory[p] else {unreachable!()};
            let Item::Weapon(e_weapon) = &enemy.inventory[e] else {unreachable!()};

            let p_damage = p_weapon.straight_damage;
            let e_damage = e_weapon.straight_damage;

            match p_weapon.speed.cmp(&e_weapon.speed) {
                Ordering::Less => {
                    enemy.health -= p_damage;
                    "You both attacked, and you were faster and got away unscathed".to_string()
                }
                Ordering::Greater => {
                    player.health -= e_damage;
                    format!("You both attacked, but the {} was faster and you couldn't get a hit in.", enemy.name)
                }
                Ordering::Equal => {
                    enemy.health -= p_damage;
                    player.health -= e_damage;
                    "You both attacked with the same speed, and you both got hit.".to_string()
                }
            }
        }
        // Both heal
        (EatFood(p), EatFood(e)) => {
            let Item::Food(p_food) = player.inventory.remove(p) else {unreachable!()};
            let Item::Food(e_food) = enemy.inventory.remove(e) else {unreachable!()};

            let p_inc = player.health.heal_to_max(p_food.heals_for, player.max_health);
            let e_inc = enemy.health.heal_to_max(e_food.heals_for, enemy.max_health);

            format!(
                "You both took some time out of the fight to eat some food - how peaceful.\nYou ate your {} and were healed {} HP. The {} ate their {} and was healed {} HP.",
                p_food.name, p_inc, enemy.name, e_food.name, e_inc
            )
        }
        // Player heals
        (EatFood(p), _) => {
            let Item::Food(p_food) = player.inventory.remove(p) else {unreachable!()};
            let p_inc = player.health.heal_to_max(p_food.heals_for, player.max_health);

            format!("You ate your {} and were healed by {} HP", p_food.name, p_inc)
        }
        // Enemy heals
        (_, EatFood(e)) => {
            let Item::Food(e_food) = enemy.inventory.remove(e) else {unreachable!()};
            let e_inc = enemy.health.heal_to_max(e_food.heals_for, enemy.max_health);

            format!("The {} ate their {} and was healed by {} HP", enemy.name,  e_food.name, e_inc)
        }
        // Enemy dodges but player hits
        (AttackLeft(p), DodgeLeft) | (AttackRight(p), DodgeRight) => {
            let Item::Weapon(p_weapon) = &player.inventory[p] else {unreachable!()};

            let prev_enemy_health = enemy.health;
            enemy.health -= p_weapon.dodge_damage;

            format!("The {} dodged, but you caught them and dealt {} damage.", enemy.name, prev_enemy_health - enemy.health)
        }
        // Player dodges but enemy hits
        (DodgeLeft, AttackLeft(e)) | (DodgeRight, AttackRight(e)) => {
            let Item::Weapon(e_weapon) = &enemy.inventory[e] else {unreachable!()};

            let prev_player_health = player.health;
            player.health -= e_weapon.dodge_damage;

            format!("You dodged, but the {} caught you and dealt {} damage.", enemy.name, prev_player_health - player.health)
        }
        // Neither the player or the enemy attacks
        (Nothing | DodgeLeft | DodgeRight, Nothing | DodgeLeft | DodgeRight) => {
            "Neither of you attacked. What a waste of time.".to_string()
        }
        // The player attacks but it is dodged
        (AttackLeft(_) | AttackStraight(_) | AttackRight(_), _) => {
            "You attacked but it didn't connect".to_string()
        }
        // The enemy attacks but it is dodged
        (_, AttackLeft(_) | AttackStraight(_) | AttackRight(_)) => {
            "The enemy attacked but it didn't connect.".to_string()
        }
    };

    format!("{}\n{}n{result_text}", 
        player.describe_combat_action(player_action),
        enemy.describe_combat_action(enemy_action),
    )
}