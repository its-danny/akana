mod auth;
mod network;
mod player;
mod social;
mod spatial;
mod test;
mod visual;
mod world;

use bevy::{
    app::ScheduleRunnerSettings, asset::AssetPlugin, log::LogPlugin, prelude::*, utils::Duration,
};
use bevy_proto::{prelude::ProtoDataOptions, ProtoPlugin};
use dotenv::dotenv;

use auth::AuthPlugin;
use network::NetworkPlugin;
use player::PlayerPlugin;
use social::SocialPlugin;
use spatial::SpatialPlugin;
use world::WorldPlugin;

fn main() {
    dotenv().ok();

    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin)
        .add_plugin(ProtoPlugin {
            options: Some(ProtoDataOptions {
                directories: vec![String::from("server/assets/prototypes")],
                recursive_loading: true,
                ..Default::default()
            }),
        })
        .add_plugin(LogPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(NetworkPlugin)
        .add_plugin(AuthPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(SpatialPlugin)
        .add_plugin(SocialPlugin)
        .run();
}
