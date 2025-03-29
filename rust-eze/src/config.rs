pub struct GameConfig;

impl GameConfig {
    pub const PLAYER_SIZE: f32 = 25.0;  // Raio do jogador
    pub const ENEMY_SIZE: f32 = 20.0;  // Raio do inimigo
    pub const WALL_THICKNESS: f32 = 20.0;  // Espessura das paredes
    pub const PLAYER_SPEED: f32 = 300.0;  // Velocidade do jogador
    pub const ENEMY_SPEED: f32 = 200.0;  // Velocidade dos inimigos
    pub const COLLISION_DISTANCE: f32 = Self::PLAYER_SIZE + Self::ENEMY_SIZE;  // Distância de colisão
}
