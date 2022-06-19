#[cfg(test)]
pub mod utils {
    use bevy::{math::IVec2, prelude::Entity, utils::Uuid};
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
        items::components::{backpack::Backpack, can_take::CanTake, item::Item},
        network::server::ConnectionId,
        player::components::{character::Character, client::NetworkClient, online::Online},
        spatial::components::{collider::Collider, door::Door, position::Position, tile::Tile},
        visual::components::{details::Details, sprite::Sprite},
    };

    pub struct PlayerBundle {
        pub name: String,
        pub x: i32,
        pub y: i32,
        pub items: Vec<Entity>,
    }

    impl Default for PlayerBundle {
        fn default() -> Self {
            Self {
                name: Name().fake::<String>(),
                x: 0,
                y: 0,
                items: Vec::new(),
            }
        }
    }

    pub fn player_bundle(
        PlayerBundle { name, x, y, items }: PlayerBundle,
    ) -> (NetworkClient, Character, Position, Sprite, Backpack, Online) {
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
            Sprite {
                character: "@".to_string(),
                color: "FFFFFF".to_string(),
                background: None,
            },
            Backpack(items),
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
                color: "0F172A".to_string(),
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
            Sprite {
                character,
                color,
                background: None,
            },
            Position(IVec2::new(x, y)),
        )
    }

    pub struct ItemBundle {
        pub name: String,
        pub description: String,
        pub character: String,
        pub color: String,
        pub x: i32,
        pub y: i32,
    }

    impl Default for ItemBundle {
        fn default() -> Self {
            Self {
                name: Sentence(1..2).fake::<String>(),
                description: Paragraph(1..2).fake::<String>(),
                character: "x".into(),
                color: "0F172A".to_string(),
                x: 0,
                y: 0,
            }
        }
    }

    pub fn item_bundle(
        ItemBundle {
            name,
            description,
            character,
            color,
            x,
            y,
        }: ItemBundle,
    ) -> (Item, Details, Sprite, Position, CanTake) {
        (
            Item,
            Details {
                name: name.into(),
                description: description.into(),
            },
            Sprite {
                character,
                color,
                background: None,
            },
            Position(IVec2::new(x, y)),
            CanTake,
        )
    }

    pub fn item_in_backpack_bundle(
        ItemBundle {
            name,
            description,
            character,
            color,
            x: _,
            y: _,
        }: ItemBundle,
    ) -> (Item, Details, Sprite, CanTake) {
        (
            Item,
            Details {
                name: name.into(),
                description: description.into(),
            },
            Sprite {
                character,
                color,
                background: None,
            },
            CanTake,
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
                color: "FAFAFA".to_string(),
                background: None,
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
                color: "FAFAFA".to_string(),
                background: None,
            },
            Position(IVec2::new(x, y)),
        )
    }
}
