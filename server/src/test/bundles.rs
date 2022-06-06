#[cfg(test)]
pub mod utils {
    use bevy::{math::IVec2, utils::Uuid};
    use fake::{faker::internet::raw::IPv4, locales::EN, Fake};

    use crate::{
        network::server::ConnectionId,
        player::components::{character::Character, client::Client, online::Online},
        spatial::components::{collider::Collider, door::Door, position::Position, tile::Tile},
        visual::components::{details::Details, sprite::Sprite},
    };

    pub fn connection_id() -> ConnectionId {
        let address = format!("{}:58142", IPv4(EN).fake::<String>())
            .parse()
            .unwrap();

        ConnectionId {
            address,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn player_bundle(
        id: ConnectionId,
        name: &str,
        x: i32,
        y: i32,
    ) -> (Client, Position, Character, Online) {
        (
            Client { id, width: 80 },
            Position(IVec2::new(x, y)),
            Character {
                name: name.into(),
                id: 1,
            },
            Online,
        )
    }

    pub fn tile_bundle(
        name: &str,
        description: &str,
        x: i32,
        y: i32,
    ) -> (Tile, Details, Position, Sprite) {
        (
            Tile,
            Details {
                name: name.into(),
                description: description.into(),
            },
            Position(IVec2::new(x, y)),
            Sprite {
                character: ".".into(),
                color: "black_bold".into(),
            },
        )
    }

    pub fn door_bundle(
        is_horizontal: bool,
        x: i32,
        y: i32,
    ) -> (Door, Position, Details, Sprite, Collider) {
        (
            Door {
                opened_character: "/".to_string(),
                closed_character: if is_horizontal { "|" } else { "-" }.to_string(),
            },
            Position(IVec2::new(x, y)),
            Details {
                name: "Door".into(),
                description: "A door.".into(),
            },
            Sprite {
                character: if is_horizontal { "|" } else { "-" }.to_string(),
                color: "white".to_string(),
            },
            Collider,
        )
    }

    pub fn open_door_bundle(
        is_horizontal: bool,
        x: i32,
        y: i32,
    ) -> (Door, Position, Details, Sprite) {
        (
            Door {
                opened_character: "/".to_string(),
                closed_character: if is_horizontal { "|" } else { "-" }.to_string(),
            },
            Position(IVec2::new(x, y)),
            Details {
                name: "Door".into(),
                description: "A door.".into(),
            },
            Sprite {
                character: if is_horizontal { "|" } else { "-" }.to_string(),
                color: "white".to_string(),
            },
        )
    }
}
