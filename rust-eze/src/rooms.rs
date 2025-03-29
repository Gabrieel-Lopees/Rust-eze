use bevy::prelude::*;
use bevy::window::PrimaryWindow;
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
            .add_systems(Update, generate_new_rooms);
    }
}

#[derive(Resource)]
pub struct CurrentRoom {
    pub id: RoomId,
    pub entered_from: Option<&'static str>, // "north", "south", "east", "west", ou None
}

impl Default for CurrentRoom {
    fn default() -> Self {
        CurrentRoom {
            id: RoomId::Central,
            entered_from: None, // A sala inicial não tem direção de entrada
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

#[derive(Component)]
pub struct Room {
    pub id: RoomId,
    pub north: Option<RoomId>,
    pub south: Option<RoomId>,
    pub east: Option<RoomId>,
    pub west: Option<RoomId>,
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

        graph
    }

    pub fn get_room(&self, id: RoomId) -> Option<&Room> {
        self.rooms.get(&id)
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

    let mut room = match room_graph.get_room(current_room.id) {
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
        entered_from = Some("south"); // Veio do sul da nova sala
    } else if player_pos.y < -y_bound && room.south.is_some() {
        new_room = room.south;
        entered_from = Some("north"); // Veio do norte da nova sala
    } else if player_pos.x > x_bound && room.east.is_some() {
        new_room = room.east;
        entered_from = Some("west"); // Veio do oeste da nova sala
    } else if player_pos.x < -x_bound && room.west.is_some() {
        new_room = room.west;
        entered_from = Some("east"); // Veio do leste da nova sala
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
) {
    let enemy_count = enemy_query.iter().filter(|enemy| enemy.room == current_room.id).count();
    if enemy_count > 0 {
        return;
    }

    // Primeiro, obtemos uma cópia das conexões atuais da sala
    let room_connections = if let Some(room) = room_graph.get_room(current_room.id) {
        (room.north, room.south, room.east, room.west)
    } else {
        error!("Sala atual não encontrada no grafo: {:?}", current_room.id);
        return;
    };

    // Lista de direções disponíveis para criar novas salas
    let mut available_directions = vec!["north", "south", "east", "west"];

    if let Some(entered_from) = current_room.entered_from {
        available_directions.retain(|&dir| dir != entered_from);
    }

    // Criamos uma lista de novas salas para adicionar
    let mut new_rooms = Vec::new();

    for &direction in &available_directions {
        match direction {
            "north" if room_connections.0.is_none() => new_rooms.push("north"),
            "south" if room_connections.1.is_none() => new_rooms.push("south"),
            "east" if room_connections.2.is_none() => new_rooms.push("east"),
            "west" if room_connections.3.is_none() => new_rooms.push("west"),
            _ => {}
        }
    }

    // Agora, criamos as novas salas
    for &direction in &new_rooms {
        let new_id = room_graph.add_room(current_room.id, direction);
        if let Some(room) = room_graph.rooms.get_mut(&current_room.id) {
            match direction {
                "north" => room.north = Some(new_id),
                "south" => room.south = Some(new_id),
                "east" => room.east = Some(new_id),
                "west" => room.west = Some(new_id),
                _ => {}
            }
        }
    }
}
