use std::io;
use rand::Rng;
use colored::*;
use std::process::Command;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Player {
    // Our player details
    name: String,
    hp: i32,
    maxHp: i32,
    mp: i32,
    maxMp: i32,
    equippedWeapon: Weapon,
    level: i32,
    strength: i32,
    defense: i32,
    exp: i32,
    gold: i32,
    inventory: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Enemy {
    // Our enemy details
    name: String,
    hp: i32,
    attack: i32,
    expToGive: i32,
    goldToGive: i32,
}

#[derive(Serialize, Deserialize)]
struct Weapon {
    // Our weapon details
    name: String,
    description: String,
    minDamage: i32,
    maxDamage: i32,
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/c", "cls"])
            .status()
            .expect("Failed to clear screen!");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear screen!");
    }
}

impl Player {
    fn show_Status(&self) { // Can be called at any time to show the player's status
        clear_screen();
        println!("-------------------------");
        println!("{}, you are Level {}", self.name, self.level);
        println!("HP: {}/{}", self.hp, self.maxHp);
        println!("Strength: {}", self.strength);
        println!("Defense: {}", self.defense);
        println!("Exp: {}", self.exp);
        println!("-------------------------");
    }

    fn take_Damage(&mut self, damage: i32) { // Make the player take damage
        self.hp -= damage;
        println!("{} has taken {} damage! {} has {} hp remaining!", self.name, damage, self.name, self.hp);
    }

    fn gain_health(&mut self, health: i32) { // Make the player gain health
        if self.hp + health > self.maxHp {
            self.hp = self.maxHp;
            println!("{} has gained {} hp! {} has max hp!", self.name, health, self.name);
            return;
        }
        else {
            self.hp += health;
            println!("{} has gained {} hp!", self.name, health);
            return;
        }
    }

    fn gain_Exp(&mut self, exp: i32) { // Make the player gain exp
        self.exp += exp;
        println!("{} has gained {} exp!", self.name, exp);
    }

    fn level_Up (&mut self) {
        self.level += 1;
        self.strength += 1;
        self.defense += 1;
        self.maxHp += 10;
    }

    fn gain_Gold(&mut self, gold: i32) { // Make the player gain gold
        self.gold += gold;
        println!("{} has gained {} gold!", self.name, gold);
    }

    fn add_item(&mut self, item: &str)  {
        self.inventory.push(item.to_string());
        println!("{} has added {} to their inventory!", self.name, item);
    }

    fn use_item(&mut self, item: &str) {
        if let Some(index) = self.inventory.iter().position(|i| i == item) {
            self.inventory.remove(index);
            match item {
                "Herb" | "herb" => {
                    self.gain_health(25);
                    println!("{} has used a herb! {} has gained 25 hp!", self.name, self.name);
                }
                _ => println!("{} has used {} with no effect!", self.name, item),
            }
        } else {
            println!("{} does not have {} in their inventory!", self.name, item);
        }
    }

    fn show_inventory(&self) {
        clear_screen();
        println!("-------------------------");
        println!("Equipped weapon: {} - Attack range Min: {} | Max: {}", self.equippedWeapon.name, self.equippedWeapon.minDamage, self.equippedWeapon.maxDamage);
        println!("Gold: {}", self.gold);
        println!("{}'s inventory:", self.name);
        for item in &self.inventory {
            println!("{}", item);
        }
        println!("-------------------------");
    }

    fn save(&self) {
        let filename = format!("{}.json", self.name);
        let serialized = serde_json::to_string(&self).expect("Failed to serialize player data");
        fs::write(filename, serialized).expect("Failed to save player data");
        println!("Game saved successfully!");
    }

    fn load(name: &str) -> Option<Self> {
        let filename = format!("{}.json", name);
        if let Ok(data) = fs::read_to_string(filename) {
            if let Ok(player) = serde_json::from_str(&data) {
                println!("-------------------------");
                println!("Game loaded successfully!");
                println!("Welcome back to Zyveria!");
                println!("-------------------------");
                return Some(player);
            }
        }
        None
    }
}

fn user_Input() -> String { // This is the users input for interacting with the out of combat menu
    println!("{}", "What would you like to do?".blue());
    println!("{}", "> Status");
    println!("{}", "> Inventory");
    println!("{}", "> Use item");
    println!("{}", "> Shop");
    println!("{}", "> Smithy");
    println!("{}", "> Shrine");
    println!("{}", "> Look for a fight");
    println!("{}", "> Save game");
    println!("{}", "< Quit game");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}

fn shop(player: &mut Player) {
    clear_screen();
    println!("-------------------------");
    println!("Welcome to the shop!");
    println!("What would you like to buy?");
    println!("> Herb - 10 gold (Heals 25 hp)");
    println!("> Mana Stone - 10 gold (Restores 5 mp)");
    println!("< Back - Return to the main menu.");
    println!("-------------------------");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim() {
        "Herb" | "herb" => {
            if player.gold < 10 {
                println!("You do not have enough gold to buy a herb!");
                return;
            }
            else {
                player.gain_Gold(-10);
                player.add_item("Herb");
                println!("{} has bought a herb for {} gold!", player.name, 10);
            }
        }
        "Mana Stone" | "mana stone" => {
            if player.gold < 10 {
                println!("You do not have enough gold to buy a mana stone!");
                return;
            }
            else {
                player.gain_Gold(-10);
                player.add_item("Mana Stone");
                println!("{} has bought a mana stone for {} gold!", player.name, 10);
            }
        }
        "quit" | "Quit" | "back" | "Back" => return,
        _ => println!("Invalid item!"),
    }
}

fn smithy (player: &mut Player) {
    clear_screen();
    println!("-------------------------");
    println!("Welcome to the smithy!");
    println!("What would you like to buy?");
    println!("> Sword - 25 gold (Increases attack range to 5-10)");
    println!("> Hammer - 25 gold (Increases attack range to 3-13)");
    println!("< Back - Return to the main menu.");
    println!("-------------------------");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim() {
        "Sword" | "sword" => {
            if player.gold < 25 {
                println!("You do not have enough gold to buy a sword!");
                return;
            }
            else {
                player.gain_Gold(-25);
                player.equippedWeapon = Weapon {
                    name: "Sword".to_string(),
                    description: "A decent sword.".to_string(),
                    minDamage: 5,
                    maxDamage: 10,
                };
                println!("{} has bought a sword for {} gold!", player.name, 25);
            }
        }
        "Hammer" | "hammer" => {
            if player.gold < 25 {
                println!("You do not have enough gold to buy a hammer!");
                return;
            }
            else {
                player.gain_Gold(-25);
                player.equippedWeapon = Weapon {
                    name: "Hammer".to_string(),
                    description: "An unwieldy hammer.".to_string(),
                    minDamage: 3,
                    maxDamage: 13,
                };
                println!("{} has bought a hammer for {} gold!", player.name, 25);
            }
        }
        "quit" | "Quit" | "back" | "Back" => return,
        _ => println!("Invalid item!"),
    }
}

fn look_For_Fight2(player: &mut Player) {
    clear_screen();
    println!("-------------------------");
    println!("Where would you like to go?");
    println!("> Plains - Smaller prey, less risk, less rewards.");
    println!("> Forest - Medium prey, mid risk, mid rewards.");
    println!("> Mountains - Larger prey, more risk, more rewards.");
    println!("Back - Return to the main menu.");
    println!("-------------------------");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut enemy = Enemy {
        name: "Goblin".green().to_string(),
        hp: 0,
        attack: 0,
        expToGive: 0,
        goldToGive: 0,
    };
    let mut rng = rand::thread_rng();

    match input.trim() {
        "Plains" | "plains" => {
            enemy.name = "Goblin".green().to_string();
            enemy.hp = rng.gen_range(5..10);
            enemy.attack = rng.gen_range(1..6);
            enemy.expToGive = enemy.hp.max(2) / 2;
            enemy.goldToGive = enemy.hp.max(2) / 2;
            println!("You have encountered a {} with {} hp!", enemy.name, enemy.hp);
        }
        "Forest" | "forest" => {
            enemy.name = "Bear".yellow().to_string();
            enemy.hp = rng.gen_range(10..20);
            enemy.attack = rng.gen_range(5..11);
            enemy.expToGive = enemy.hp.max(2) / 2;
            enemy.goldToGive = enemy.hp.max(2) / 2;
            println!("You have encountered a {} with {} hp!", enemy.name, enemy.hp);
        }
        "Mountains" | "mountains" => {
            enemy.name = "Troll".red().to_string();
            enemy.hp = rng.gen_range(35..50);
            enemy.attack = rng.gen_range(10..16);
            enemy.expToGive = enemy.hp.max(2) / 2;
            enemy.goldToGive = enemy.hp.max(2) / 2;
            println!("You have encountered a {} with {} hp!", enemy.name, enemy.hp);
        }
        "quit" | "Quit" | "back" | "Back" => return,
        _ => {
            println!("Invalid location!");
            return;
        },
    }

    loop {
        let mut input = String::new();
        println!("What would you like to do?");
        println!("attack");
        println!("run");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim() {
            "attack" => {
                let damage = rng.gen_range(player.equippedWeapon.minDamage..=player.equippedWeapon.maxDamage.max(1) + player.strength);
                enemy.hp -= damage;
                println!("You have dealt {} damage to the {}! The {} has {} hp remaining!", damage, enemy.name, enemy.name, enemy.hp);
                if enemy.hp <= 0 {
                    clear_screen();
                    println!("You have defeated the {}! You have gained {} exp!", enemy.name, enemy.expToGive);
                    player.gain_Exp(enemy.expToGive);
                    player.gain_Gold(enemy.goldToGive);
                    break;
                }
                let damage = rng.gen_range(1..=enemy.attack.max(1) - player.defense);
                if damage < 0 {
                    player.take_Damage(1);
                    if player.hp <= 0 {
                        println!("You have been defeated by the {}!", enemy.name);
                        break;
                    }
                    println!("The {} has dealt {} damage to you! You have {} hp remaining!", enemy.name, damage, player.hp);
                }
                else {
                    player.take_Damage(damage);
                    if player.hp <= 0 {
                        println!("You have been defeated by the {}!", enemy.name);
                        break;
                    }
                    println!("The {} has dealt {} damage to you! You have {} hp remaining!", enemy.name, damage, player.hp);
                }
            }
            "run" => {
                clear_screen();
                println!("You have run away from the fight!");
                break;
            }
            "quit" => break,
            _ => println!("Invalid command!"),
        }
    }
}

fn shrine(player: &mut Player) {
    clear_screen();
    println!("-------------------------");
    println!("What services would you like to use? (You have {} exp)", player.exp);
    println!("Level Up - {} exp", (player.level + 1) * 50);
    println!("Back - Return to the main menu.");
    println!("-------------------------");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim() {
        "Level Up" | "level up" | "level" | "Level" => {
            if player.exp < (player.level + 1) * 50 {
                println!("You do not have enough exp to level up!");
                return;
            }
            else {
                player.gain_Exp(-(player.level + 1) * 50);
                player.level_Up();
                println!("{} has leveled up to level {}!", player.name, player.level);
            }
        }
        "quit" | "Quit" | "back" | "Back" => return,
        _ => println!("Invalid service!"),
    }
}

fn main() {
    clear_screen();
    let mut input = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let name = input.trim().to_string();

    let mut player = if let Some(loaded_player) = Player::load(&name) {
        loaded_player
    } else {
        println!("Creating a new profile for {}.", name);
        Player {
            name,
            hp: 100,
            maxHp: 100,
            mp: 5,
            maxMp: 5,
            equippedWeapon: Weapon {
                name: "Fist".to_string(),
                description: "A rusty fist.".to_string(),
                minDamage: 1,
                maxDamage: 5,
            },
            exp: 0,
            gold: 0,
            inventory: Vec::new(),
            level: 0,
            strength: 0,
            defense: 0,
        }
    };

    loop {
        let choice = user_Input();
        match choice.as_str() {
            "status" | "Status" => player.show_Status(),
            "inventory" | "Inventory" => player.show_inventory(),
            "use item" | "Use item" | "use" | "Use" => {
                println!("Enter the name of the item you would like to use:");
                let mut item = String::new();
                io::stdin().read_line(&mut item).expect("Failed to read line");
                player.use_item(item.trim());
            }
            "shop" | "Shop" => shop(&mut player),
            "smithy" | "Smithy" => smithy(&mut player),
            "shrine" | "Shrine" => shrine(&mut player),
            "look for a fight" | "Look for a fight" | "look" | "Look" | "fight" | "Fight" => look_For_Fight2(&mut player),
            "save game" | "Save game" | "save" | "Save" => player.save(),
            "quit" | "Quit" => break,
            _ => println!("Invalid command!"),
        }
    }
}
