use bevy::{prelude::*, utils::HashMap};

use rand::seq::IndexedRandom;
use rand::seq::SliceRandom;
use rand::{self, Rng};

use crate::components::{
    default_apple, Alive, City, Country, State, Item, ItemDetails, ItemType, Person,
    PersonActions, PersonState, Position, PriceRecord, Shop, TerrainType,
};
use crate::constants::*;

pub fn setup(mut commands: Commands) {
    // Use the ThreadRng alternative since thread_rng() is deprecated.
    let mut rng = rand::rng();

    let mut countries: Vec<Entity> = Vec::new();

    // Create Countries
    for i in 0..NUM_COUNTRIES {
        let country_entity = commands
            .spawn(Country {
                name: format!("Country {}", i),
                ..default()
            })
            .id();
        countries.push(country_entity);
    }

    // Create States (formerly Estates) and assign each to a random Country
    let mut states: Vec<Entity> = Vec::new();
    for i in 0..NUM_STATES {
        let terrain_type = match i % 4 {
            0 => TerrainType::Grassland,
            1 => TerrainType::Forest,
            2 => TerrainType::Mountain,
            _ => TerrainType::Desert,
        };

        let state_entity = commands
            .spawn(State {
                name: format!("State {}", i),
                terrain_type,
                ..default()
            })
            .id();

        // Randomly assign this state to one of the countries
        if let Some(&country_entity) = countries.choose(&mut rng) {
            commands.entity(country_entity).add_child(state_entity);
        }
        states.push(state_entity);
    }


    // Create the cities and also store their components in the map
    let mut cities_map: HashMap<Entity, City> = HashMap::new();
    let mut cities: Vec<Entity> = Vec::new();
    for i in 0..NUM_CITIES {
        let city = City {
            name: format!("City {}", i),
            position: Position {
                x: rng.random_range(0.0..200.0),
                y: rng.random_range(0.0..200.0),
            },
            ..default()
        };
        let city_entity = commands.spawn(city.clone()).id();
        // Update parent's children later (assign to a random state)
        if let Some(&state) = states.choose(&mut rng) {
            commands.entity(state).add_child(city_entity);
        }
        cities.push(city_entity);
        cities_map.insert(city_entity, city);
    }

    // Randomly distribute Persons among the Cities
    for i in 0..NUM_PERSONS {
        if let Some(&city_entity) = cities.choose(&mut rng) {
            let person_entity = commands
                .spawn((
                    Person {
                        name: format!("Person {}", i),
                        position: Position {
                            x: rng.random_range(0.0..100.0),
                            y: rng.random_range(0.0..100.0),
                        },
                        gold: START_GOLD,
                        ..default()
                    },
                    Alive(true),
                ))
                .id();

            // Add person as a child of the city
            commands.entity(city_entity).add_child(person_entity);

            // Also update the City component's persons vector
            if let Some(mut city) = cities_map.get_mut(&city_entity) {
                city.persons.push(person_entity);
            }
        }
    }

    // Randomly distribute Shops among the Cities
    for _ in 0..NUM_SHOPS {
        if let Some(&city_entity) = cities.choose(&mut rng) {
            // Prepare the items for the shop (for example, the default apple)
            let mut items = HashMap::new();
            let mut price_history = HashMap::new();

            let apple = default_apple();
            let apple_details = ItemDetails {
                price: 10,
                stock: 10,
                transactions: (0, 0),
            };

            items.insert(apple.clone(), apple_details);
            price_history.insert(
                apple,
                vec![PriceRecord {
                    timestamp: 0.0,
                    price: 10,
                }],
            );

            let shop_entity = commands
                .spawn(Shop {
                    items,
                    position: Position {
                        x: rng.random_range(0.0..100.0),
                        y: rng.random_range(0.0..100.0),
                    },
                    price_history,
                })
                .id();

            // Add shop as a child of the city
            commands.entity(city_entity).add_child(shop_entity);

            // Also update the City component's shops vector
            if let Some(mut city) = cities_map.get_mut(&city_entity) {
                city.shops.push(shop_entity);
            }
        }
    }
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
            _ => person.state,
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
            }

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
    // Get the default apple key for inventory lookups
    let apple_key = default_apple();

    for (mut person, alive) in persons.iter_mut() {
        // Skip processing for dead persons
        if !alive.0 {
            continue;
        }

        // Retrieve the number of apples in the person's inventory
        let number_apples = *person.inventory.get(&apple_key).unwrap_or(&0);

        // If the person is healthy, idle, has at least one apple, and gold is less than 30,
        // change the action to selling.
        if person.state == PersonState::Healthy
            && person.action == PersonActions::Idle
            && number_apples > 0
            && person.gold < 30
        {
            person.action = PersonActions::Selling;
        }

        // If the person is hungry, idle, and has at least one apple,
        // change the action to eating.
        if person.action == PersonActions::Idle
            && person.state == PersonState::Hungry
            && number_apples > 0
        {
            person.action = PersonActions::Eating;
        }

        // If the person is hungry, idle, has no apple, and has more than 30 gold,
        // change the action to buying.
        if person.action == PersonActions::Idle
            && person.state == PersonState::Hungry
            && number_apples == 0
            && person.gold > 30
        {
            person.action = PersonActions::Buying;
        }

        // If the person has less than 30 gold, is idle, and has no apple,
        // change the action to planting.
        if person.gold < 30 && number_apples == 0 && person.action == PersonActions::Idle {
            person.action = PersonActions::Planting;
        }

        // If the person has more than 30 gold, is idle, and has at least one apple,
        // then with a 5% chance change the action to planting.
        if person.gold > 30 && person.action == PersonActions::Idle && number_apples > 0 {
            let mut rng = rand::rngs::ThreadRng::default();
            if rng.gen_range(0..100) < 5 {
                person.action = PersonActions::Planting;
            }
        }
    }
}

// 3. Sistema de Planting: se o Person estiver no estado Planting por mais de 20 segundos consecutivos, ele recebe 10 maçãs.
pub fn planting_system(mut persons: Query<&mut Person>, time: Res<Time>) {
    let apple_key = default_apple();
    for mut person in persons.iter_mut() {
        if person.action == PersonActions::Planting {
            person.planting_time += time.delta_secs();
            if person.planting_time >= 10.0 {
                // Adiciona 10 maçãs ao inventário do Person usando o apple_key
                *person.inventory.entry(apple_key.clone()).or_insert(0) += 10;
                // Reseta o timer e retorna ao estado Idle
                person.planting_time = 0.0;
                person.action = PersonActions::Idle;
            }
        } else {
            // Se o estado não for Planting, reseta o contador de tempo
            person.planting_time = 0.0;
        }
    }
}

// --- Sistema de Interação com a Loja ---
// Se o estado da Person for Buying, ela tenta comprar uma maçã de um Shop aleatório.
pub fn shop_interaction_system(mut persons: Query<&mut Person>, mut shops: Query<&mut Shop>) {
    use rand::Rng;
    let mut rng = rand::rngs::ThreadRng::default();
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
                    // Obtém a "chave" para o item Apple
                    let apple_key = default_apple();
                    if let Some(details) = shop.items.get_mut(&apple_key) {
                        // Se o Person tiver gold suficiente para comprar o item
                        if person.gold >= details.price {
                            if details.stock > 0 {
                                details.stock -= 1;
                                person.gold -= details.price;
                                // Atualiza o inventário do Person
                                *person.inventory.entry(apple_key.clone()).or_insert(0) += 1;
                                // Registra a transação de compra (incrementa o contador de compras)
                                details.transactions.0 += 1;
                            }
                        }
                        person.action = PersonActions::Idle;
                    }
                }
            }
            PersonActions::Selling => {
                // Seleciona aleatoriamente um Shop
                let random_index = rng.gen_range(0..shop_count);
                if let Some(mut shop) = shops.iter_mut().nth(random_index) {
                    let apple_key = default_apple();
                    // Verifica se o Person possui o item "Apple" em seu inventário
                    if let Some(count) = person.inventory.get_mut(&apple_key) {
                        if *count > 0 {
                            let total_items = *count;
                            // Remove todas as "Apple" do inventário do Person
                            *count = 0;
                            if let Some(details) = shop.items.get_mut(&apple_key) {
                                person.gold += details.price * (total_items as usize);
                                details.stock += total_items as usize;
                                // Registra a transação de venda (incrementa o contador de vendas)
                                details.transactions.1 += total_items as usize;
                            }
                            person.action = PersonActions::Idle;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

// --- Sistema de Alimentação ---
// Se o estado da Person for Eating, ela consome uma maçã para recuperar a saciedade.
pub fn feeding_system(mut persons: Query<&mut Person>, _time: Res<Time>) {
    // Obtém a chave padrão para o item Apple.
    let apple_key = default_apple();

    for mut person in persons.iter_mut() {
        if person.action == PersonActions::Eating {
            if let Some(count) = person.inventory.get_mut(&apple_key) {
                if *count > 0 {
                    *count -= 1;
                    // Extraí o valor nutricional do item Apple.
                    if let ItemType::Food { nutritional_value } = &apple_key.item_type {
                        person.hunger += *nutritional_value as f32;
                    }
                    if person.hunger > 100.0 {
                        person.hunger = 100.0;
                    }
                    // Após comer, o Person retorna ao estado Idle.
                    person.action = PersonActions::Idle;
                }
            }
        }
    }
}

// Função auxiliar de atualização de preço
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

// Updated price update system
pub fn price_update_system(mut shops: Query<&mut Shop>, time: Res<Time>) {
    for mut shop in shops.iter_mut() {
        let elapsed_secs = time.elapsed_secs();
        // Vetor temporário para armazenar os itens que terão seu preço atualizado
        let mut updates = Vec::new();

        // Itera sobre os itens e atualiza os detalhes
        for (item, details) in shop.items.iter_mut() {
            let (sales, purchases) = details.transactions;
            let total = sales + purchases;
            if total >= TRANSACTION_THRESHOLD || elapsed_secs > 20.0 {
                let new_price = update_price(details.price, sales, purchases, details.stock as i32);
                details.price = new_price;
                details.transactions = (0, 0); // Reseta os contadores de transações
                updates.push((item.clone(), new_price));
            }
        }

        // Agora, atualiza o histórico de preços para os itens coletados
        for (item, new_price) in updates {
            shop.price_history
                .entry(item)
                .or_insert_with(Vec::new)
                .push(PriceRecord {
                    timestamp: elapsed_secs,
                    price: new_price,
                });
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

    println!(
        "Average hunger: {}. Average gold: {}, Average health: {}",
        average_hunger, average_gold, average_health
    );
}

// pub fn update_price_history_system(mut shops: Query<&mut Shop>, time: Res<Time>) {
//     for mut shop in shops.iter_mut() {
//         for (item, details) in shop.items.iter() {
//             if let Some(history) = shop.price_history.get_mut(item) {
//                 history.push(PriceRecord {
//                     timestamp: time.elapsed_secs(),
//                     price: details.price,
//                 });
//             }
//         }
//     }
// }

// Calculate inflation for a specific item
fn calculate_item_inflation(price_history: &[PriceRecord]) -> Option<f32> {
    if price_history.len() < 2 {
        return None;
    }

    let first_price = price_history.first().unwrap().price as f32;
    let last_price = price_history.last().unwrap().price as f32;

    Some(((last_price - first_price) / first_price) * 100.0)
}

pub fn get_shops_stats(shops: Query<&Shop, With<Shop>>) {
    let mut total_stock: f32 = 0.0;
    let mut total_price: usize = 0;
    let mut total_sales: usize = 0;
    let mut total_purchases: usize = 0;
    let mut count_items = 0;

    let mut total_inflation: f32 = 0.0;
    let mut count_inflation: usize = 0;

    // Iterate over all shops
    for shop in shops.iter() {
        // Iterate over each item in the shop for basic stats
        for (_item, details) in shop.items.iter() {
            total_stock += details.stock as f32;
            total_price += details.price;
            total_sales += details.transactions.0;
            total_purchases += details.transactions.1;
            count_items += 1;
        }

        // Iterate over each item's price history to calculate inflation
        for (_item, history) in shop.price_history.iter() {
            if let Some(first_record) = history.first() {
                if let Some(last_record) = history.last() {
                    if first_record.price > 0 {
                        // Compute inflation as the percentage change from the first price to the last price
                        let inflation = (last_record.price as f32 - first_record.price as f32)
                            / first_record.price as f32
                            * 100.0;
                        total_inflation += inflation;
                        count_inflation += 1;
                    }
                }
            }
        }
    }

    if count_items > 0 {
        let average_stock = total_stock / count_items as f32;
        let average_price = total_price / count_items;
        let average_sales = total_sales / count_items;
        let average_purchases = total_purchases / count_items;
        let average_inflation = if count_inflation > 0 {
            total_inflation / count_inflation as f32
        } else {
            0.0
        };

        println!(
            "Shops stats -> Average stock: {}, Average price: {}, Average sales: {}, Average purchases: {}, Average inflation: {:.2}%",
            average_stock, average_price, average_sales, average_purchases, average_inflation
        );
    } else {
        println!("No items found in shops.");
    }
}

pub fn get_city_stats(
    cities: Query<(&City, &Children), With<City>>,
    shops: Query<&Shop, With<Shop>>,
    people: Query<(&Person, &Alive), With<Person>>,
) {
    use std::collections::HashMap;

    // Iterate over all cities
    for (city, children) in cities.iter() {
        // Total number of persons and shops in the city
        let mut total_persons = 0;
        let mut total_shops = 0;

        // Map to accumulate inflation data per product.
        // Key: product name, Value: (total inflation, count of records)
        let mut inflation_data: HashMap<String, (f32, usize)> = HashMap::new();

        for child in children.iter() {
            if let Ok(person) = people.get(*child) {
                total_persons += 1;
            }    
        }

        for child in children.iter() {
            if let Ok(shop) = shops.get(*child) {
                total_shops += 1;

                for (item, history) in shop.price_history.iter() {
                    // Ensure there is at least one record and a valid starting price.
                    if let (Some(first_record), Some(last_record)) =
                        (history.first(), history.last())
                    {
                        if first_record.price > 0 {
                            // Calculate inflation as percentage change
                            let inflation = (last_record.price as f32 - first_record.price as f32)
                                / first_record.price as f32
                                * 100.0;
                            // Accumulate inflation data for the product (using product name as key)
                            let entry = inflation_data.entry(item.name.clone()).or_insert((0.0, 0));
                            entry.0 += inflation;
                            entry.1 += 1;
                        }
                    }
                }
            }

        }
        
        // Compute the average inflation per product
        let mut average_inflation: HashMap<String, f32> = HashMap::new();
        for (product, (total_inflation, count)) in inflation_data.iter() {
            if *count > 0 {
                average_inflation.insert(product.clone(), total_inflation / (*count as f32));
            }
        }

        // Print results for the city
        println!(
            "City: {} - Total persons: {}, Total shops: {}",
            city.name, total_persons, total_shops
        );
        println!("Average inflation per product:");
        for (product, inflation) in average_inflation.iter() {
            println!("  {}: {:.2}%", product, inflation);
        }
    }
}


pub fn get_state_stats(
    states: Query<(&State, &Children), With<State>>,
    cities: Query<(&City, &Children), With<City>>,
    shops: Query<&Shop, With<Shop>>,
    people: Query<&Person, With<Person>>,
) {

    // Iterate over all states (estates)
    for (state, state_children) in states.iter() {
        let mut total_cities = 0;
        let mut total_persons = 0;
        let mut total_gold = 0;
        let mut total_shops = 0;
        // Map to accumulate inflation data per product.
        // Key: product name, Value: (total inflation, count)
        let mut inflation_data: HashMap<String, (f32, usize)> = HashMap::new();

        // Each child of the state is a City
        for &city_entity in state_children.iter() {
            if let Ok((city, city_children)) = cities.get(city_entity) {
                total_cities += 1;
                // For each child of the city, check if it's a Person or a Shop
                for &child in city_children.iter() {
                    if let Ok(person) = people.get(child) {
                        total_persons += 1;
                        total_gold += person.gold;
                    }
                    if let Ok(shop) = shops.get(child) {
                        total_shops += 1;
                        // Process each shop's price history
                        for (item, history) in shop.price_history.iter() {
                            if let (Some(first_record), Some(last_record)) = (history.first(), history.last()) {
                                if first_record.price > 0 {
                                    let inflation = (last_record.price as f32 - first_record.price as f32)
                                        / first_record.price as f32
                                        * 100.0;
                                    let entry = inflation_data.entry(item.name.clone()).or_insert((0.0, 0));
                                    entry.0 += inflation;
                                    entry.1 += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Compute average inflation per product for this state
        let mut average_inflation: HashMap<String, f32> = HashMap::new();
        for (product, (total_inflation, count)) in inflation_data.iter() {
            if *count > 0 {
                average_inflation.insert(product.clone(), total_inflation / (*count as f32));
            }
        }

        println!(
            "State: {} - Total cities: {} - Total persons: {} - Total gold: {}, Total shops: {}",
            state.name, total_cities, total_persons, total_gold, total_shops
        );
        println!("Average inflation per product:");
        for (product, inflation) in average_inflation.iter() {
            println!("  {}: {:.2}%", product, inflation);
        }
    }
}


pub fn get_country_stats(
    countries: Query<(&Country, &Children), With<Country>>,
    estates: Query<(&State, &Children), With<State>>,
    cities: Query<(&City, &Children), With<City>>,
    shops: Query<&Shop, With<Shop>>,
    people: Query<(&Person, &Alive), With<Person>>,
) {
    use std::collections::HashMap;

    // Iterate over all countries
    for (country, country_children) in countries.iter() {
        let mut total_persons = 0;
        let mut total_shops = 0;
        let mut total_states = 0;
        let mut total_cities = 0;
        // Map to accumulate inflation data per product.
        let mut inflation_data: HashMap<String, (f32, usize)> = HashMap::new();

        // Each child of a country is a State (Estate)
        for &state_entity in country_children.iter() {
            if let Ok((estate, estate_children)) = estates.get(state_entity) {
                total_states += 1;
                // Each child of the state is a City
                for &city_entity in estate_children.iter() {
                    if let Ok((city, city_children)) = cities.get(city_entity) {
                        total_cities += 1;
                        // Process each child of the city
                        for &child in city_children.iter() {
                            if people.get(child).is_ok() {
                                total_persons += 1;
                            }
                            if let Ok(shop) = shops.get(child) {
                                total_shops += 1;
                                // Process the shop's price history
                                for (item, history) in shop.price_history.iter() {
                                    if let (Some(first_record), Some(last_record)) = (history.first(), history.last()) {
                                        if first_record.price > 0 {
                                            let inflation = (last_record.price as f32 - first_record.price as f32)
                                                / first_record.price as f32
                                                * 100.0;
                                            let entry = inflation_data.entry(item.name.clone()).or_insert((0.0, 0));
                                            entry.0 += inflation;
                                            entry.1 += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Compute average inflation per product for the country
        let mut average_inflation: HashMap<String, f32> = HashMap::new();
        for (product, (total_inflation, count)) in inflation_data.iter() {
            if *count > 0 {
                average_inflation.insert(product.clone(), total_inflation / (*count as f32));
            }
        }

        println!(
            "Country: {} - Total cities: {} - Total persons: {}, Total shops: {}",
            country.name, total_cities, total_persons, total_shops
        );
        println!("Average inflation per product:");
        for (product, inflation) in average_inflation.iter() {
            println!("  {}: {:.2}%", product, inflation);
        }
    }
}

pub fn test_system(
    cities: Query<(&City, &Children), With<City>>,
    persons: Query<&Person, With<Person>>
) {
    for (city, children) in cities.iter() {
        let mut total_gold = 0;
        let mut total_persons = 0;
        for child in children.iter() {
            if let Ok(person) = persons.get(*child) {
                total_gold += person.gold;
                total_persons += 1;
            }
        }
        println!("City: {},Total persons: A1 - {},A2 - {} , Total gold: {}", city.name, city.persons.len(),total_persons , total_gold);
    }
}