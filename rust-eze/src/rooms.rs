use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::sprite::MaterialMesh2dBundle; // Importa MaterialMesh2dBundle
use bevy::math::primitives::Rectangle; // Importa Rectangle do módulo correto
use crate::player::Player;
use crate::config::GameConfig;
use crate::enemies::Enemy;
use std::collections::HashMap;

pub struct RoomsPlugin;

impl Plugin for RoomsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RoomGraph>()
            .init_resource::<CurrentRoom>()
            .add_systems(Update, check_room_transition.before(generate_new_rooms))
            .add_systems(Update, generate_new_rooms)
            .add_systems(Update, spawn_doors);
    }
}

#[derive(Resource)]
pub struct CurrentRoom {
    pub id: RoomId,
    pub entered_from: Option<&'static str>,
}

impl Default for CurrentRoom {
    fn default() -> Self {
        CurrentRoom {
            id: RoomId::Central,
            entered_from: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoomId {
    Central,
    Generated(usize),
}

impl Default for RoomId {
    fn default() -> Self {
        RoomId::Central
    }
}

#[derive(Component, Debug)]
pub struct Room {
    pub id: RoomId,
    pub north: Option<RoomId>,
    pub south: Option<RoomId>,
    pub east: Option<RoomId>,
    pub west: Option<RoomId>,
}

#[derive(Component)]
pub struct Door {
    pub direction: &'static str,
    pub room_id: RoomId,
}

#[derive(Resource, Default)]
pub struct RoomGraph {
    rooms: HashMap<RoomId, Room>,
    next_id: usize,
}

impl RoomGraph {
    pub fn new() -> Self {
        let mut graph = RoomGraph {
            rooms: HashMap::new(),
            next_id: 0,
        };

        let central_room = Room {
            id: RoomId::Central,
            north: None,
            south: None,
            east: None,
            west: None,
        };
        graph.rooms.insert(RoomId::Central, central_room);
        info!("RoomGraph inicializado com sala Central: {:?}", graph.rooms.get(&RoomId::Central));

        graph
    }

    pub fn get_room(&self, id: RoomId) -> Option<&Room> {
        let room = self.rooms.get(&id);
        if room.is_none() {
            error!("Sala não encontrada no grafo: {:?}", id);
        }
        room
    }

    pub fn add_room(&mut self, parent_id: RoomId, direction: &str) -> RoomId {
        let new_id = RoomId::Generated(self.next_id);
        self.next_id += 1;

        let new_room = Room {
            id: new_id,
            north: None,
            south: None,
            east: None,
            west: None,
        };

        let parent_room = match self.rooms.get_mut(&parent_id) {
            Some(room) => room,
            None => {
                error!("Sala pai não encontrada ao adicionar nova sala: {:?}", parent_id);
                return new_id;
            }
        };
        match direction {
            "north" => {
                parent_room.north = Some(new_id);
                self.rooms.insert(new_id, Room {
                    id: new_id,
                    south: Some(parent_id),
                    ..new_room
                });
            }
            "south" => {
                parent_room.south = Some(new_id);
                self.rooms.insert(new_id, Room {
                    id: new_id,
                    north: Some(parent_id),
                    ..new_room
                });
            }
            "east" => {
                parent_room.east = Some(new_id);
                self.rooms.insert(new_id, Room {
                    id: new_id,
                    west: Some(parent_id),
                    ..new_room
                });
            }
            "west" => {
                parent_room.west = Some(new_id);
                self.rooms.insert(new_id, Room {
                    id: new_id,
                    east: Some(parent_id),
                    ..new_room
                });
            }
            _ => {}
        }

        info!("Nova sala adicionada: {:?}", new_id);
        new_id
    }
}

fn check_room_transition(
    mut commands: Commands,
    mut current_room: ResMut<CurrentRoom>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Enemy>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    room_graph: Res<RoomGraph>,
) {
    let enemy_count = enemy_query.iter().filter(|enemy| enemy.room == current_room.id).count();
    if enemy_count > 0 {
        return;
    }

    let (player_entity, player_transform) = player_query.single();
    let player_pos = player_transform.translation.xy();

    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();
    let x_bound = (window_width - GameConfig::WALL_THICKNESS - GameConfig::PLAYER_SIZE) / 2.0;
    let y_bound = (window_height - GameConfig::WALL_THICKNESS - GameConfig::PLAYER_SIZE) / 2.0;

    let room = match room_graph.get_room(current_room.id) {
        Some(room) => room,
        None => {
            error!("Sala atual não encontrada no grafo: {:?}", current_room.id);
            return;
        }
    };

    let mut new_room = None;
    let mut entered_from = None;
    if player_pos.y > y_bound && room.north.is_some() {
        new_room = room.north;
        entered_from = Some("south");
    } else if player_pos.y < -y_bound && room.south.is_some() {
        new_room = room.south;
        entered_from = Some("north");
    } else if player_pos.x > x_bound && room.east.is_some() {
        new_room = room.east;
        entered_from = Some("west");
    } else if player_pos.x < -x_bound && room.west.is_some() {
        new_room = room.west;
        entered_from = Some("east");
    }

    if let Some(new_room_id) = new_room {
        info!("Transitando para nova sala: {:?}", new_room_id);
        current_room.id = new_room_id;
        current_room.entered_from = entered_from;
        commands
            .entity(player_entity)
            .insert(Transform::from_xyz(0.0, 0.0, 0.0));
    }
}

fn generate_new_rooms(
    mut room_graph: ResMut<RoomGraph>,
    current_room: Res<CurrentRoom>,
    enemy_query: Query<&Enemy>,
    powerup_query: Query<&crate::projectiles::powerups::PowerUp>,
) {
    let enemy_count = enemy_query.iter().filter(|enemy| enemy.room == current_room.id).count();
    if enemy_count > 0 {
        return;
    }

    let powerup_count = powerup_query.iter().filter(|powerup| powerup.powerup_type != crate::projectiles::powerups::PowerUpType::ExtraLife).count();
    if powerup_count > 0 {
        return;
    }

    let available_directions: Vec<&str> = {
        let room = match room_graph.get_room(current_room.id) {
            Some(room) => room,
            None => {
                error!("Sala atual não encontrada no grafo: {:?}", current_room.id);
                return;
            }
        };

        if room.north.is_some() && room.south.is_some() && room.east.is_some() && room.west.is_some() {
            return;
        }

        let directions = if current_room.id == RoomId::Central {
            vec!["north", "south", "east", "west"]
        } else {
            let mut dirs = vec!["north", "south", "east", "west"];
            if let Some(entered_from) = current_room.entered_from {
                dirs.retain(|&dir| dir != entered_from);
            }
            dirs
        };

        directions.into_iter().filter(|&dir| {
            match dir {
                "north" => room.north.is_none(),
                "south" => room.south.is_none(),
                "east" => room.east.is_none(),
                "west" => room.west.is_none(),
                _ => false,
            }
        }).collect()
    };

    for direction in available_directions {
        room_graph.add_room(current_room.id, direction);
    }
}

fn spawn_doors(
    mut commands: Commands,
    current_room: Res<CurrentRoom>,
    room_graph: Res<RoomGraph>,
    door_query: Query<(Entity, &Door)>,
    enemy_query: Query<&Enemy>,
    powerup_query: Query<&crate::projectiles::powerups::PowerUp>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let enemy_count = enemy_query.iter().filter(|enemy| enemy.room == current_room.id).count();
    if enemy_count > 0 {
        return;
    }

    let powerup_count = powerup_query.iter().filter(|powerup| powerup.powerup_type != crate::projectiles::powerups::PowerUpType::ExtraLife).count();
    if powerup_count > 0 {
        return;
    }

    let room = match room_graph.get_room(current_room.id) {
        Some(room) => room,
        None => {
            error!("Sala atual não encontrada no grafo: {:?}", current_room.id);
            return;
        }
    };

    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();
    let x_bound = (window_width - GameConfig::WALL_THICKNESS) / 2.0;
    let y_bound = (window_height - GameConfig::WALL_THICKNESS) / 2.0;

    // Remove portas existentes na sala atual
    for (entity, door) in door_query.iter() {
        if door.room_id == current_room.id {
            commands.entity(entity).despawn();
        }
    }

    // Spawna portas visíveis para as direções disponíveis
    if room.north.is_some() {
        commands.spawn((
            Door {
                direction: "north",
                room_id: current_room.id,
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(50.0, 20.0)).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_xyz(0.0, y_bound, 0.0),
                ..default()
            },
        ));
    }
    if room.south.is_some() {
        commands.spawn((
            Door {
                direction: "south",
                room_id: current_room.id,
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(50.0, 20.0)).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_xyz(0.0, -y_bound, 0.0),
                ..default()
            },
        ));
    }
    if room.east.is_some() {
        commands.spawn((
            Door {
                direction: "east",
                room_id: current_room.id,
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(20.0, 50.0)).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_xyz(x_bound, 0.0, 0.0),
                ..default()
            },
        ));
    }
    if room.west.is_some() {
        commands.spawn((
            Door {
                direction: "west",
                room_id: current_room.id,
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(20.0, 50.0)).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_xyz(-x_bound, 0.0, 0.0),
                ..default()
            },
        ));
    }
}