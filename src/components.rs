use std::fmt;

use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Person {
    pub name: String,
    pub health: f32,
    pub hunger: f32,
    pub energy: f32,
    pub state: PersonState,
    pub action: PersonActions,
    pub gold: usize,
    pub inventory: HashMap<Apple, i32>, // inventário com chave Apple e quantidade
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
            planting_time: 0.0
        }
    }
}

#[derive(Component, Debug)]
pub struct Alive(pub bool);

#[derive(Component, Debug)]
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

// 2. Adicione o componente Shop:
#[derive(Component, Debug)]
pub struct Shop {
    pub apple_price: usize,
    pub stock: HashMap<Apple, i32>,
    pub position: Position,
    // Registro das transações para cada produto: (vendas, compras)
    pub transactions: HashMap<Apple, (usize, usize)>,
}


impl fmt::Display for Shop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Shop {{ apple_price: {}, stock: {:?}, position: {:?}, transactions: {:?} }}", self.apple_price, self.stock, self.position, self.transactions)
    }
}

// Default para Shop, com estoque inicial de maçãs e uma posição definida
impl Default for Shop {
    fn default() -> Self {
        let mut stock = HashMap::new();
        stock.insert(DEFAULT_APPLE, 10);
        let mut transactions = HashMap::new();
        transactions.insert(DEFAULT_APPLE, (0, 0));
        Self {
            apple_price: 10,
            stock,
            position: Position { x: 100.0, y: 100.0 },
            transactions,
        }
    }
}

// Definição da struct Apple com valor alimentar
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Apple {
    pub nutritional_value: u32,
}

// Constante para facilitar o uso da Apple padrão
pub const DEFAULT_APPLE: Apple = Apple {
    nutritional_value: 50,
};


// New City structure
#[derive(Component)]
pub struct City {
    pub name: String,
    pub shops: Vec<Entity>, // Store Bevy entities for shops
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
pub struct Estate {
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

impl Default for Estate {
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