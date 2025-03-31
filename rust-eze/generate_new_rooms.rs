fn generate_new_rooms(
    mut room_graph: ResMut<RoomGraph>,
    current_room: Res<CurrentRoom>,
    _enemy_query: Query<&Enemy>, // Prefixado com `_` para evitar warnings de variável não utilizada
) {
    // Obter `RoomId` da sala atual
    let current_room_id = current_room.id;

    // Separar a verificação do estado antes da mutação
    if room_graph
        .get_room(current_room_id)
        .map(|room| room.east.is_none())
        .unwrap_or(false)
    {
        // Mutação após a verificação
        room_graph.add_room(current_room_id, Direction::East);
    }
}