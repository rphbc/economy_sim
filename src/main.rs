use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

mod components;
mod constants;
mod entities;
mod systems;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, systems::setup)
        .add_systems(Update, systems::hunger_system)
        .add_systems(Update, systems::energy_system)
        // .add_systems(
        //     Update,
        //     systems::get_people_stats.run_if(on_timer(Duration::from_secs(2))),
        // )
        // .add_systems(
        //     Update,
        //     systems::get_shops_stats.run_if(on_timer(Duration::from_secs(2))),
        // )
        // .add_systems(
        //     Update,
        //     systems::get_city_stats.run_if(on_timer(Duration::from_secs(5))),
        // )
        .add_systems(
            Update,
            systems::get_state_stats.run_if(on_timer(Duration::from_secs(5))),
        )
        .add_systems(
            Update,
            systems::get_country_stats.run_if(on_timer(Duration::from_secs(5))),
        )
        .add_systems(Update, systems::test_system.run_if(on_timer(Duration::from_secs(5))))
        .add_systems(
            Update,
            systems::despawn_dead_person_system.run_if(on_timer(Duration::from_secs(20))),
        )
        .add_systems(Update, systems::reasoning_system)
        .add_systems(Update, systems::shop_interaction_system)
        .add_systems(Update, systems::price_update_system)
        .add_systems(Update, systems::feeding_system)
        .add_systems(Update, systems::planting_system)
        .run();
}
