use bevy::prelude::*;
use bevy_proto::ProtoComponent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
pub struct Tile;
