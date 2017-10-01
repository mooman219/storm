use physics::aabb::AABB2D;

trait Collidable2D {
    fn get_aabb() -> AABB2D;
}
