use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::components::{Alive, Person, PersonActions, PersonState, Position, Shop, DEFAULT_APPLE};
use crate::constants::{NUM_PLAYERS, NUM_SHOPS, START_GOLD, TRANSACTION_THRESHOLD, PERSON_HUNGRY_THRESHOLD};

pub fn setup(mut commands: Commands) {
    // Criação das Persons (não mostrado aqui, pois já existe)
    for i in 0..NUM_PLAYERS {
        commands.spawn((Person {
            name: format!("Person {}", i),
            position: Position { x: 0.0, y: 0.0 },
            gold: START_GOLD,
            ..Default::default()
        },
        Alive(true))
    );
    }
    print!("Persons created");
    // Criação dos Shops
    for i in 0..NUM_SHOPS {
        commands.spawn(Shop {
            apple_price: 10,
            stock: {
                let mut s = HashMap::new();
                s.insert(DEFAULT_APPLE, 10);
                s
            },
            // Exemplo: cada Shop com posição variada no eixo x
            position: Position { x: 100.0 + i as f32 * 50.0, y: 100.0 },
            transactions: {
                let mut t = HashMap::new();
                t.insert(DEFAULT_APPLE, (0, 0));
                t
            },
        });
    }
    print!("Shops created");
}

// --- Sistema de Fome ---
// Atualiza o estado da Person para Hungry se a saciedade (hunger) estiver baixa.
pub fn hunger_system(mut persons: Query<(&mut Person, &mut Alive), With<Person>>, time: Res<Time>) {
    for (mut person, mut alive) in persons.iter_mut() {
        // A saciedade diminui com o passar do tempo
        person.hunger -= 2.0 * time.delta_secs();
        if person.hunger < 0.0 {
            person.hunger = 0.0;
        }

        // Pessoa perde saúde se faminta
        if person.hunger <= 0.0 {
            person.health -= 1.0 * time.delta_secs();
            if person.health < 0.0 {
                person.health = 0.0;
                alive.0 = false;
            }

        } else {
            // A saude aumenta com o passar do tempo se saciado
            person.health += 1.0 * time.delta_secs();
            if person.health > 100.0 {
                person.health = 100.0;
            }
        }

        // Estado da pessoa muda para com fome se abaixo do limite
        person.state = match person.hunger {
            ..PERSON_HUNGRY_THRESHOLD => PersonState::Hungry,
            PERSON_HUNGRY_THRESHOLD..100.0 => PersonState::Healthy,   
            _ => person.state
        };
    }
}

/// Sistema que atualiza a energia da Pessoa de acordo com seu estado.
/// - Em Idle, não há gasto de energia (ou a reposição é lenta).
/// - Em ações que consomem energia (Walking, Buying, Selling), a energia diminui.
/// - Ao repor energia, se a pessoa estiver em ação, a taxa de decréscimo da fome aumenta.
/// - Se a fome estiver zerada, a energia se repõe mais rápido, mas utilizando a saúde.
pub fn energy_system(mut query: Query<&mut Person>, time: Res<Time>) {
    for mut person in query.iter_mut() {
        match person.state {
            PersonState::Healthy => {
                // Sem ação: energia se recupera lentamente.
                person.energy += 1.0 * time.delta_secs();
            },

            PersonState::Hungry => {
                // Se energia não estiver completa, há uma reposição com efeito colateral na fome.
                if person.hunger == 0.0 {
                    // Se a saciedade estiver zerada, a energia é reposta mais rápido, mas consumindo saúde.
                    let energy_boost = 10.0 * time.delta_secs();
                    person.energy += energy_boost;
                    person.health -= energy_boost * 0.5;
                    if person.health < 0.0 {
                        person.health = 0.0;
                    }
                } else {
                    // Se estiver com fome mas não estiver faminto recupera energia mais lentamente
                    person.energy += 0.7 * time.delta_secs();
                }
            }
        }

        // Garante que os valores máximos não sejam ultrapassados.
        if person.energy > 100.0 {
            person.energy = 100.0;
        }
    }
}


pub fn reasoning_system(mut persons: Query<(&mut Person, &mut Alive), With<Person>>) {
    // if Person is Hungry and has no apple in inventory change state to buying
    for (mut person, alive) in persons.iter_mut() {

        if alive.0 == false {
            continue;
        }

        let number_apples = person.inventory.get(&DEFAULT_APPLE).unwrap_or(&0).clone();

        // if healthy, idle, has apple and has gold, change to selling
        if person.state == PersonState::Healthy && person.action == PersonActions::Idle && number_apples > 0 && person.gold < 30 {
            person.action = PersonActions::Selling;
        }

        // if hungry and has apple, change to eating
        if person.action == PersonActions::Idle && person.state == PersonState::Hungry && number_apples > 0 {
            person.action = PersonActions::Eating;
        }
        
        // if hungry and has no apple and has gold, change to buying
        if person.action == PersonActions::Idle && person.state == PersonState::Hungry && number_apples == 0 && person.gold > 30 {
            person.action = PersonActions::Buying;
        }

        // if gold < 30, and idle and no apple, change to planting
        if person.gold < 30 &&  number_apples == 0 && person.action == PersonActions::Idle {
            person.action = PersonActions::Planting;
        }

        // if gold > 30, and idle and has apple, has 0.05% chance to planting
        if person.gold > 30 && person.action == PersonActions::Idle && number_apples > 0 {
            if rand::random_range(0..100) < 5 {
                person.action = PersonActions::Planting;
            }
        }

    }
}

// 3. Sistema de Planting: se o Person estiver no estado Planting por mais de 20 segundos consecutivos, ele recebe 10 maçãs.
pub fn planting_system(mut persons: Query<&mut Person>, time: Res<Time>) {
    for mut person in persons.iter_mut() {
        if person.action == PersonActions::Planting {
            person.planting_time += time.delta_secs();
            if person.planting_time >= 10.0 {
                // Adiciona 10 maçãs ao inventário da Person
                *person.inventory.entry(DEFAULT_APPLE).or_insert(0) += 10;
                // Opcional: reseta o timer para permitir ganhos periódicos se continuar plantando
                person.planting_time = 0.0;
                person.action = PersonActions::Idle;
            }
        } else {
            // Se o estado não for Planting, reseta o contador
            person.planting_time = 0.0;
        }
    }
}

// --- Sistema de Interação com a Loja ---
// Se o estado da Person for Buying, ela tenta comprar uma maçã de um Shop aleatório.
pub fn shop_interaction_system(
    mut persons: Query<&mut Person>,
    mut shops: Query<&mut Shop>,
) {
    use rand::Rng;
    let mut rng = rand::rng();
    let shop_count = shops.iter_mut().count();
    if shop_count == 0 {
        return;
    }

    for mut person in persons.iter_mut() {
        match person.action {
            PersonActions::Buying => {
                // Seleciona aleatoriamente um Shop
                let random_index = rng.random_range(0..shop_count);
                if let Some(mut shop) = shops.iter_mut().nth(random_index) {
                    // Se o Person ainda tiver gold suficiente
                    if person.gold >= shop.apple_price {
                        if let Some(stock) = shop.stock.get_mut(&DEFAULT_APPLE) {
                            if *stock > 0 {
                                *stock -= 1;
                                person.gold -= shop.apple_price;
                                *person.inventory.entry(DEFAULT_APPLE).or_insert(0) += 1;
                                // Registra a transação (compra)
                                if let Some((purchases, _)) = shop.transactions.get_mut(&DEFAULT_APPLE) {
                                    *purchases += 1;
                                }
                            }
                        }
                        person.action = PersonActions::Idle;
                    }
                }
            },
            PersonActions::Selling => {
                // Seleciona aleatoriamente um Shop
                let random_index = rng.random_range(0..shop_count);
                if let Some(mut shop) = shops.iter_mut().nth(random_index) {
                    // Se o Person ainda tiver a maçã em seu inventário
                    if let Some(count) = person.inventory.get_mut(&DEFAULT_APPLE) {
                        if *count > 0 {
                            let total_items = count.clone();
                            // Vende sempre todas as maçãs em seu inventário
                            *count = 0;

                            person.gold += shop.apple_price * (total_items as usize);

                            // Aumenta stock do Shop
                            if let Some(stock) = shop.stock.get_mut(&DEFAULT_APPLE) {
                                *stock += total_items;
                            }
                            // Registra a transação (venda)
                            if let Some((_, sales)) = shop.transactions.get_mut(&DEFAULT_APPLE) {
                                *sales += total_items as usize;
                            }

                            person.action = PersonActions::Idle;
                        }
                    }
                }
            },
            _ => {}
        }

    }
}


// --- Sistema de Alimentação ---
// Se o estado da Person for Eating, ela consome uma maçã para recuperar a saciedade.
pub fn feeding_system(mut persons: Query<&mut Person>, time: Res<Time>) {
    for mut person in persons.iter_mut() {
        if person.action == PersonActions::Eating {
            if let Some(count) = person.inventory.get_mut(&DEFAULT_APPLE) {
                if *count > 0 {
                    *count -= 1;
                    person.hunger += DEFAULT_APPLE.nutritional_value as f32;
                    if person.hunger > 100.0 {
                        person.hunger = 100.0;
                    }
                    // Após comer, retorna para Idle (ou outro estado adequado)
                    person.action = PersonActions::Idle;
                } 
            }
        }
    }
}

// Sistema de atualização do preço (função auxiliar)
fn update_price(old_price: usize, sales: usize, purchases: usize, stock: i32) -> usize {
    let total = sales + purchases;
    if total < TRANSACTION_THRESHOLD {
        return old_price;
    }
    let ratio = sales as f32 / (purchases as f32 + 1.0);
    let mut adjustment_factor = if ratio > 1.0 {
        1.0 + 0.1 * (ratio - 1.0)
    } else {
        1.0 - 0.1 * (1.0 - ratio)
    };

    if stock < 5 {
        adjustment_factor *= 1.1;
    } else if stock > 20 {
        adjustment_factor *= 0.9;
    }
    (old_price as f32 * adjustment_factor).max(1.0) as usize
}

// --- Sistema de Atualização de Preços ---
// Para cada Shop, se o total de transações atingir o limiar, atualiza o preço da maçã.
pub fn price_update_system(mut shops: Query<&mut Shop>, time: Res<Time>) {
    for mut shop in shops.iter_mut() {
        if let Some(&(sales, purchases)) = shop.transactions.get(&DEFAULT_APPLE) {
            let total = sales + purchases;
            if total >= TRANSACTION_THRESHOLD  || time.elapsed_secs() > 20.0 {
                if let Some(&stock) = shop.stock.get(&DEFAULT_APPLE) {
                    shop.apple_price = update_price(shop.apple_price, sales, purchases, stock);
                    // Reseta os contadores de transações
                    if let Some(entry) = shop.transactions.get_mut(&DEFAULT_APPLE) {
                        *entry = (0, 0);
                    }
                }
            }
        }
    }
}

pub fn despawn_dead_person_system(mut commands: Commands, query: Query<(Entity, &Person, &Alive)>) {
    for (entity, person, alive) in query.iter() {
        if !alive.0 {
            println!("Destruindo {} pois não está mais vivo!", person.name);
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn get_people_stats(people: Query<(&Person, &Alive), With<Person>>) {
   // get average people hunger, gold and health
    let mut average_hunger = 0.0;
    let mut average_gold = 0;
    let mut average_health = 0.0;
    for (person, alive) in people.iter() {
        if alive.0 {
            average_hunger += person.hunger;
            average_gold += person.gold;
            average_health += person.health;
        }
    }  

    average_hunger /= people.iter().count() as f32;
    average_gold /= people.iter().count();
    average_health /= people.iter().count() as f32;

    println!("Average hunger: {}. Average gold: {}, Average health: {}", average_hunger, average_gold, average_health);

}

pub fn get_shops_stats(shops: Query<&Shop, With<Shop>>) {

    // shows average stock of apples, price of apples, sales and purchases
    let mut average_stock = 0.0;
    let mut average_apple_price = 0;
    let mut average_sales = 0;
    let mut average_purchases = 0;
    
    for shop in shops.iter() {
        let stock = shop.stock.get(&DEFAULT_APPLE).unwrap_or(&0);
        let sales = shop.transactions.get(&DEFAULT_APPLE).unwrap_or(&(0, 0)).0;
        let purchases = shop.transactions.get(&DEFAULT_APPLE).unwrap_or(&(0, 0)).1;
        let apple_price = shop.apple_price;

        average_stock += *stock as f32;
        average_apple_price += apple_price;
        average_sales += sales;
        average_purchases += purchases;  
    }

    average_stock /= shops.iter().count() as f32;
    average_apple_price /= shops.iter().count();
    average_sales /= shops.iter().count();
    average_purchases /= shops.iter().count();

    println!("Average stock: {}, Average apple price: {}, Average sales: {}, Average purchases: {}", average_stock, average_apple_price, average_sales, average_purchases);

    
}
