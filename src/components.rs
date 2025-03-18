use std::fmt;

use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

#[derive(Component, Debug)]
pub struct Person {
    pub name: String,
    pub health: f32,
    pub hunger: f32,
    pub energy: f32,
    pub state: PersonState,
    pub action: PersonActions,
    pub gold: usize,
    pub inventory: HashMap<Item, i32>,
    pub position: Position,
    pub planting_time: f32, // tempo acumulado em Planting (em segundos)
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: "Person".to_string(),
            health: 100.0,
            hunger: 100.0,
            energy: 100.0,
            state: PersonState::Healthy,
            action: PersonActions::Idle,
            gold: 100,
            inventory: HashMap::new(),
            position: Position { x: 0.0, y: 0.0 },
            planting_time: 0.0,
        }
    }
}

#[derive(Component, Debug)]
pub struct Alive(pub bool);

#[derive(Component, Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum PersonState {
    Healthy,
    Hungry,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum PersonActions {
    Idle,
    Walking,
    Eating,
    Hungry,
    Planting,
    Buying,
    Selling,
}

#[derive(Component)]
pub struct Inventory(pub HashMap<String, i32>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
    pub name: String,
    pub price: usize,
    pub stock: i32,
    pub transactions: (usize, usize), // (sales, purchases)
    pub item_type: ItemType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemType {
    Food { nutritional_value: u32 },
    // Futuramente, pode ser adicionado:
    Weapon { attack_damage: u32 },
}

impl Item {
    // Construtor para um item do tipo Food
    pub fn food(name: &str, price: usize, stock: i32, nutritional_value: u32) -> Self {
        Self {
            name: name.to_string(),
            price,
            stock,
            transactions: (0, 0),
            item_type: ItemType::Food { nutritional_value },
        }
    }

    pub fn weapon(name: &str, price: usize, stock: i32, attack_damage: u32) -> Self {
        Self {
            name: name.to_string(),
            price,
            stock,
            transactions: (0, 0),
            item_type: ItemType::Weapon { attack_damage },
        }
    }
}

#[derive(Debug, Clone)]
pub struct PriceRecord {
    pub timestamp: f32,
    pub price: usize,
}

#[derive(Component, Debug)]
pub struct Shop {
    pub items: HashMap<Item, ItemDetails>,
    pub position: Position,
    pub price_history: HashMap<Item, Vec<PriceRecord>>,
}

#[derive(Debug, Clone)]
pub struct ItemDetails {
    pub price: usize,
    pub stock: usize,
    pub transactions: (usize, usize), // (sales, purchases)
}

impl fmt::Display for Shop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Shop {{ items: {:?}, position: {:?}, price_history: {:?} }}",
            self.items, self.position, self.price_history
        )
    }
}

impl Default for Shop {
    fn default() -> Self {
        let mut items = HashMap::new();
        // Exemplo: Item "Apple" do tipo Food com preço 10, estoque 10 e transações zeradas.
        let apple = default_apple();
        let apple_details = ItemDetails {
            price: 10,
            stock: 10,
            transactions: (0, 0),
        };
        items.insert(apple.clone(), apple_details);

        let mut price_history = HashMap::new();
        price_history.insert(
            apple,
            vec![PriceRecord {
                timestamp: 0.0,
                price: 10,
            }],
        );

        Self {
            items,
            position: Position { x: 100.0, y: 100.0 },
            price_history,
        }
    }
}

// Definição da struct Apple com valor alimentar
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Apple {
    pub nutritional_value: u32,
}

// Função auxiliar para obter o item "Apple" padrão
pub fn default_apple() -> Item {
    // Cria um item do tipo Food com valores iniciais.
    // Esses valores de price, stock e transações são os iniciais na loja.
    Item::food("Apple", 10, 10, 50)
}

// New City structure
#[derive(Component, Debug, Clone)]
pub struct City {
    pub name: String,
    pub shops: Vec<Entity>,   // Store Bevy entities for shops
    pub persons: Vec<Entity>, // Store Bevy entities for persons
    pub position: Position,
}

impl Default for City {
    fn default() -> Self {
        Self {
            name: "Default City".to_string(),
            shops: Vec::new(),
            persons: Vec::new(),
            position: Position { x: 0.0, y: 0.0 },
        }
    }
}

// Estate structure
#[derive(Component)]
pub struct State {
    pub name: String,
    pub cities: Vec<Entity>, // Store Bevy entities for cities
    pub terrain_type: TerrainType,
}

#[derive(Debug, Clone, Copy)]
pub enum TerrainType {
    Grassland,
    Forest,
    Mountain,
    Desert,
}

impl Default for State {
    fn default() -> Self {
        Self {
            name: "Default Estate".to_string(),
            cities: Vec::new(),
            terrain_type: TerrainType::Grassland,
        }
    }
}

// Country structure
#[derive(Component)]
pub struct Country {
    pub name: String,
    pub estates: Vec<Entity>, // Store Bevy entities for estates
    pub population: usize,
    pub total_gold: usize,
}

impl Default for Country {
    fn default() -> Self {
        Self {
            name: "Default Country".to_string(),
            estates: Vec::new(),
            population: 0,
            total_gold: 0,
        }
    }
}
