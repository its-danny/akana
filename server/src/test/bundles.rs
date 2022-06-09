#[cfg(test)]
pub mod utils {
    use bevy::{math::IVec2, utils::Uuid};
    use fake::{
        faker::{
            internet::raw::IPv4,
            lorem::en::{Paragraph, Sentence},
            name::en::Name,
        },
        locales::EN,
        Fake,
    };

    use crate::{
        network::server::ConnectionId,
        player::components::{character::Character, client::NetworkClient, online::Online},
        spatial::components::{collider::Collider, door::Door, position::Position, tile::Tile},
        visual::components::{details::Details, sprite::Sprite},
    };

    pub struct PlayerBundle {
        pub name: String,
        pub x: i32,
        pub y: i32,
    }

    impl Default for PlayerBundle {
        fn default() -> Self {
            Self {
                name: Name().fake::<String>(),
                x: 0,
                y: 0,
            }
        }
    }

    pub fn player_bundle(
        PlayerBundle { name, x, y }: PlayerBundle,
    ) -> (NetworkClient, Character, Position, Online) {
        (
            NetworkClient {
                id: ConnectionId {
                    address: format!("{}:58142", IPv4(EN).fake::<String>())
                        .parse()
                        .unwrap(),
                    uuid: Uuid::new_v4(),
                },
                width: 80,
            },
            Character {
                name: name.into(),
                id: 1,
            },
            Position(IVec2::new(x, y)),
            Online,
        )
    }

    pub struct TileBundle {
        pub name: String,
        pub description: String,
        pub character: String,
        pub color: String,
        pub x: i32,
        pub y: i32,
    }

    impl Default for TileBundle {
        fn default() -> Self {
            Self {
                name: Sentence(1..2).fake::<String>(),
                description: Paragraph(1..2).fake::<String>(),
                character: ".".into(),
                color: "black_bold".into(),
                x: 0,
                y: 0,
            }
        }
    }

    pub fn tile_bundle(
        TileBundle {
            name,
            description,
            character,
            color,
            x,
            y,
        }: TileBundle,
    ) -> (Tile, Details, Sprite, Position) {
        (
            Tile,
            Details {
                name: name.into(),
                description: description.into(),
            },
            Sprite { character, color },
            Position(IVec2::new(x, y)),
        )
    }

    pub struct DoorBundle {
        pub name: String,
        pub description: String,
        pub is_horizontal: bool,
        pub x: i32,
        pub y: i32,
    }

    impl Default for DoorBundle {
        fn default() -> Self {
            Self {
                name: Sentence(1..2).fake::<String>(),
                description: Paragraph(1..2).fake::<String>(),
                is_horizontal: true,
                x: 0,
                y: 0,
            }
        }
    }

    pub fn closed_door_bundle(
        DoorBundle {
            name,
            description,
            is_horizontal,
            x,
            y,
        }: DoorBundle,
    ) -> (Door, Details, Sprite, Position, Collider) {
        (
            Door {
                opened_character: "/".to_string(),
                closed_character: if is_horizontal { "|" } else { "-" }.to_string(),
            },
            Details {
                name: name.into(),
                description: description.into(),
            },
            Sprite {
                character: if is_horizontal { "|" } else { "-" }.into(),
                color: "white".to_string(),
            },
            Position(IVec2::new(x, y)),
            Collider,
        )
    }

    pub fn open_door_bundle(
        DoorBundle {
            name,
            description,
            is_horizontal,
            x,
            y,
        }: DoorBundle,
    ) -> (Door, Details, Sprite, Position) {
        (
            Door {
                opened_character: "/".to_string(),
                closed_character: if is_horizontal { "|" } else { "-" }.to_string(),
            },
            Details {
                name: name.into(),
                description: description.into(),
            },
            Sprite {
                character: "/".into(),
                color: "white".to_string(),
            },
            Position(IVec2::new(x, y)),
        )
    }
}
