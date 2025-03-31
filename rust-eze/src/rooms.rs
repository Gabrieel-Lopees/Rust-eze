use bevy::prelude::*;
use std::collections::HashMap;

use crate::player::Player;
use crate::enemies::Enemy;

/// Plugin responsável por gerenciar as salas do jogo.
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

/// Recurso que mantém o controle da sala atual em que o jogador se encontra.
#[derive(Resource)]
pub struct CurrentRoom {
    pub id: RoomId,
    pub entered_from: Option<Direction>,
}

impl Default for CurrentRoom {
    fn default() -> Self {
        CurrentRoom {
            id: RoomId::Central,
            entered_from: None,
        }
    }
}

/// Enum que identifica as salas. Pode ser a sala central ou salas geradas dinamicamente.
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

/// Enum para especificar direções.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// Componente que representa uma sala com suas conexões em cada direção.
#[derive(Component, Debug)]
pub struct Room {
    pub id: RoomId,
    pub north: Option<RoomId>,
    pub south: Option<RoomId>,
    pub east: Option<RoomId>,
    pub west: Option<RoomId>,
}

impl Room {
    /// Verifica se uma sala já possui conexão em uma determinada direção.
    pub fn has_connection(&self, direction: Direction) -> bool {
        match direction {
            Direction::North => self.north.is_some(),
            Direction::South => self.south.is_some(),
            Direction::East => self.east.is_some(),
            Direction::West => self.west.is_some(),
        }
    }
}

/// Componente que representa uma porta para transição entre salas.
#[derive(Component, Debug)]
pub struct Door {
    pub direction: Direction,
    pub room_id: RoomId,
}

/// Recurso que armazena o grafo de salas e o próximo ID a ser usado para uma nova sala.
#[derive(Resource)]
pub struct RoomGraph {
    rooms: HashMap<RoomId, Room>,
    next_id: usize,
}

impl Default for RoomGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl RoomGraph {
    /// Inicializa o grafo de salas com a sala central.
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
        info!("RoomGraph inicializado com sala central!");
        graph
    }

    /// Recupera uma sala pelo seu ID. Retorna `None` se a sala não for encontrada.
    pub fn get_room(&self, id: RoomId) -> Option<&Room> {
        self.rooms.get(&id)
    }

    /// Adiciona uma nova sala conectada à sala pai na direção especificada.
    pub fn add_room(&mut self, parent_id: RoomId, direction: Direction) -> Option<RoomId> {
        if let Some(parent_room) = self.rooms.get_mut(&parent_id) {
            // Garantir que não há outra sala na direção especificada.
            if parent_room.has_connection(direction) {
                error!(
                    "A sala {:?} já possui uma conexão na direção {:?}",
                    parent_id, direction
                );
                return None;
            }

            // Criar novo ID para a sala.
            let new_id = RoomId::Generated(self.next_id);
            self.next_id += 1;

            // Criar nova sala.
            let mut new_room = Room {
                id: new_id,
                north: None,
                south: None,
                east: None,
                west: None,
            };

            // Estabelecer as conexões.
            match direction {
                Direction::North => {
                    parent_room.north = Some(new_id);
                    new_room.south = Some(parent_id);
                }
                Direction::South => {
                    parent_room.south = Some(new_id);
                    new_room.north = Some(parent_id);
                }
                Direction::East => {
                    parent_room.east = Some(new_id);
                    new_room.west = Some(parent_id);
                }
                Direction::West => {
                    parent_room.west = Some(new_id);
                    new_room.east = Some(parent_id);
                }
            }

            // Adicionar a nova sala ao grafo.
            self.rooms.insert(new_id, new_room);
            info!(
                "Nova sala criada: {:?}, conectada com {:?} pela direção {:?}",
                new_id, parent_id, direction
            );
            Some(new_id)
        } else {
            error!("Sala pai não encontrada: {:?}", parent_id);
            None
        }
    }
}

/// Sistema para verificar transições de sala.
fn check_room_transition(
    _commands: Commands, // Prefixado com `_` para evitar warnings
    mut current_room: ResMut<CurrentRoom>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    room_graph: Res<RoomGraph>,
) {
    for (_player_entity, transform) in player_query.iter() {
        let position = transform.translation;

        if position.x < -100.0 {
            if let Some(next_room) = room_graph.rooms.get(&current_room.id).and_then(|room| room.west) {
                current_room.id = next_room;
                current_room.entered_from = Some(Direction::East);
                info!("Transição para a sala: {:?}", current_room.id);
            }
        }
    }
}

/// Sistema para criar novas salas dinamicamente.
fn generate_new_rooms(
    mut room_graph: ResMut<RoomGraph>,
    current_room: Res<CurrentRoom>,
    _enemy_query: Query<&Enemy>, // Prefixado com `_` para evitar warnings
) {
    let current_room_id = current_room.id;

    if room_graph
        .get_room(current_room_id)
        .map(|room| room.east.is_none())
        .unwrap_or(false)
    {
        room_graph.add_room(current_room_id, Direction::East);
    }
}

/// Sistema para gerar portas na sala atual.
fn spawn_doors(
    _commands: Commands, // Prefixado com `_` para evitar warnings
    current_room: Res<CurrentRoom>,
    room_graph: Res<RoomGraph>,
    _door_query: Query<(Entity, &Door)>, // Prefixado com `_` para evitar warnings
    _meshes: ResMut<Assets<Mesh>>,      // Prefixado com `_` para evitar warnings
    _materials: ResMut<Assets<ColorMaterial>>, // Prefixado com `_` para evitar warnings
) {
    if let Some(room) = room_graph.get_room(current_room.id) {
        if let Some(north_id) = room.north {
            info!("Porta para o norte na sala {:?}", north_id);
        }

        if let Some(south_id) = room.south {
            info!("Porta para o sul na sala {:?}", south_id);
        }
    }
}