use cgmath::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB2D {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

impl AABB2D {
    pub fn new(minx: f32, miny: f32, maxx: f32, maxy: f32) -> AABB2D {
        AABB2D {
            min: Vector2 { x: minx, y: miny },
            max: Vector2 { x: maxx, y: maxy },
        }
    }

    pub fn assign(&mut self, other: &AABB2D) {
        self.min.x = other.min.x;
        self.min.y = other.min.y;
        self.max.x = other.max.x;
        self.max.y = other.max.y;
    }

    pub fn intersects(&self, other: AABB2D) -> bool {
        if self.max.x < other.min.x || self.min.x > other.max.x {
            return false;
        }
        if self.max.y < other.min.y || self.min.y > other.max.y {
            return false;
        }
        true
    }

    pub fn slide<'a>(&mut self, mov: &Vector2<f32>, others: &Vec<AABB2D>) {
        if mov.x == 0f32 && mov.y == 0f32 {
            return;
        }

        let mut res = *mov; // Copy
        let mut aabb = *self; // Copy

        // Y movement

        if mov.y < 0f32 {
            for other in others {
                if aabb.max.x > other.min.x && aabb.min.x < other.max.x && other.max.y <= aabb.min.y {
                    let min = other.max.y - aabb.min.y;
                    if min > res.y {
                        res.y = min;
                    }
                }
            }
        }

        if mov.y > 0f32 {
            for other in others {
                if aabb.max.x > other.min.x && aabb.min.x < other.max.x && other.min.y >= aabb.max.y {
                    let max = other.min.y - aabb.max.y;
                    if max < res.y {
                        res.y = max;
                    }
                }
            }
        }

        aabb.min.y += res.y;
        aabb.max.y += res.y;

        // X movement

        if mov.x < 0f32 {
            for other in others {
                if aabb.max.y > other.min.y && aabb.min.y < other.max.y && other.max.x <= aabb.min.x {
                    let min = other.max.x - aabb.min.x;
                    if min > res.x {
                        res.x = min;
                    }
                }
            }
        }

        if mov.x > 0f32 {
            for other in others {
                if aabb.max.y > other.min.y && aabb.min.y < other.max.y && other.min.x >= aabb.max.x {
                    let max = other.min.x - aabb.max.x;
                    if max < res.x {
                        res.x = max;
                    }
                }
            }
        }

        aabb.min.x += res.x;
        aabb.max.x += res.x;
        *self = aabb;
    }
}
