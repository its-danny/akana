use bevy::prelude::*;
use bevy_proto::ProtoComponent;
use serde::{Deserialize, Serialize};

/// Defines an entity that can be picked up or dropped by the player.
#[derive(Clone, Serialize, Deserialize, ProtoComponent, Component, Debug)]
pub struct CanTake;
