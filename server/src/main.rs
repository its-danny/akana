mod auth;
mod network;
mod player;

use std::env;

use auth::AuthPlugin;
use bevy::{app::ScheduleRunnerSettings, log::LogPlugin, prelude::*, utils::Duration};
use dotenv::dotenv;
use network::{server::NetworkServer, NetworkPlugin};
use player::PlayerPlugin;

fn main() {
    dotenv().ok();

    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(NetworkPlugin)
        .add_plugin(AuthPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_network)
        .run();
}

fn setup_network(server: Res<NetworkServer>) {
    let server_url = env::var("SERVER_URL").expect("Could not read SERVER_URL from env");

    server.listen(server_url);
}
