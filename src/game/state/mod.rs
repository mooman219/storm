pub mod collision;
pub mod movement;
pub mod render;

use self::collision::CollisionState;
use self::movement::MovementState;
use self::render::RenderState;

pub struct State {
    collision: CollisionState,
    movement: MovementState,
    render: RenderState,
}
