use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::player::Player;
use crate::config::GameConfig;
use crate::enemies::Enemy;
use rand::Rng;
use std::collections::HashMap;

pub struct RoomsPlugin;

impl Plugin for RoomsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RoomGraph>()
            .init_resource::<CurrentRoom>()
            .add_systems(Update, (check_room_transition, generate_new_rooms));
    }
}

#[derive(Resource, Default)]
pub struct CurrentRoom {
    pub id: RoomId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoomId {
    Central,
    Generated(usize), // Salas geradas proceduralmente
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

        // Sala inicial (Central)
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

        // Conecta a nova sala à sala pai
        let mut parent_room = self.rooms.get_mut(&parent_id).unwrap();
        match direction {
            "north" => {
                parent_room.north = Some(new_id);
                self.rooms.insert(new_id, Room {
                    id: new_id,
                    south: Some(parent_id),
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

    let room = room_graph.get_room(current_room.id).unwrap();

    let mut new_room = None;
    if player_pos.y > y_bound && room.north.is_some() {
        new_room = room.north;
    } else if player_pos.y < -y_bound && room.south.is_some() {
        new_room = room.south;
    } else if player_pos.x > x_bound && room.east.is_some() {
        new_room = room.east;
    } else if player_pos.x < -x_bound && room.west.is_some() {
        new_room = room.west;
    }

    if let Some(new_room_id) = new_room {
        current_room.id = new_room_id;
        commands
            .entity(player_entity)
            .insert(Transform::from_xyz(0.0, 0.0, 0.0));
    }
}

fn generate_new_rooms(
    mut room_graph: ResMut<RoomGraph>,
    current_room: Res<CurrentRoom>,
) {
    let room = room_graph.get_room(current_room.id).unwrap();

    // Gera até 3 novas salas se a sala atual não tiver todas as conexões preenchidas
    let mut available_directions = vec![];
    if room.north.is_none() {
        available_directions.push("north");
    }
    if room.east.is_none() {
        available_directions.push("east");
    }
    if room.west.is_none() {
        available_directions.push("west");
    }

    let mut rng = rand::thread_rng();
    available_directions.shuffle(&mut rng);

    let num_new_rooms = available_directions.len().min(3);
    for i in 0..num_new_rooms {
        let direction = available_directions[i];
        room_graph.add_room(current_room.id, direction);
    }
}