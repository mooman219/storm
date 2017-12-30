use game::world::World;

pub struct Universe {
    worlds: Vec<World>,
    input: u32,
    render: u32,
    network: u32,
}
