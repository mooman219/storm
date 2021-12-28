use cgmath::*;

/// Provides tools for interacting with and creating axis aligned bounding boxes in 2D.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB2D {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

impl AABB2D {
    pub const fn new(minx: f32, miny: f32, maxx: f32, maxy: f32) -> AABB2D {
        AABB2D {
            min: Vector2 {
                x: minx,
                y: miny,
            },
            max: Vector2 {
                x: maxx,
                y: maxy,
            },
        }
    }

    pub fn from_min_max(min: Vector2<f32>, max: Vector2<f32>) -> AABB2D {
        AABB2D {
            min,
            max,
        }
    }

    pub fn from_pos_size(pos: Vector2<f32>, size: Vector2<f32>) -> AABB2D {
        AABB2D {
            min: pos,
            max: pos + size,
        }
    }

    #[inline(always)]
    pub fn intersects(&self, other: &AABB2D) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    #[inline(always)]
    pub fn contains(&self, other: &AABB2D) -> bool {
        self.min.x <= other.min.x
            && self.max.x >= other.max.x
            && self.min.y <= other.min.y
            && self.max.y >= other.max.y
    }

    #[inline(always)]
    pub fn contains_point(&self, point: &Vector2<f32>) -> bool {
        self.min.x <= point.x && self.max.x >= point.x && self.min.y <= point.y && self.max.y >= point.y
    }

    pub fn slide(&mut self, mov: &Vector2<f32>, others: &[AABB2D]) -> bool {
        if mov.x == 0f32 && mov.y == 0f32 {
            return false;
        }
        let mut result = false;
        let mut res = *mov;
        let mut aabb = *self;

        // Y movement

        if mov.y < 0f32 {
            for other in others {
                if aabb.max.x > other.min.x && aabb.min.x < other.max.x && other.max.y <= aabb.min.y {
                    let min = other.max.y - aabb.min.y;
                    if min > res.y {
                        result = true;
                        res.y = min;
                    }
                }
            }
        } else if mov.y > 0f32 {
            for other in others {
                if aabb.max.x > other.min.x && aabb.min.x < other.max.x && other.min.y >= aabb.max.y {
                    let max = other.min.y - aabb.max.y;
                    if max < res.y {
                        result = true;
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
                        result = true;
                        res.x = min;
                    }
                }
            }
        } else if mov.x > 0f32 {
            for other in others {
                if aabb.max.y > other.min.y && aabb.min.y < other.max.y && other.min.x >= aabb.max.x {
                    let max = other.min.x - aabb.max.x;
                    if max < res.x {
                        result = true;
                        res.x = max;
                    }
                }
            }
        }

        aabb.min.x += res.x;
        aabb.max.x += res.x;
        *self = aabb;

        result
    }
}

// ////////////////////////////////////////////////////////////////////////////
// Tests
// ////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use alloc::vec;

    #[test]
    fn slide() {
        let v = vec![AABB2D::new(2f32, 0f32, 3f32, 1f32), AABB2D::new(0f32, 1f32, 1f32, 2f32)];
        let mut aabb = AABB2D::new(0f32, 0f32, 1f32, 1f32);

        {
            let mot = Vector2::new(2f32, 0f32);
            aabb.slide(&mot, &v);
            assert_eq!(aabb, AABB2D::new(1f32, 0f32, 2f32, 1f32));
        }

        {
            let mot = Vector2::new(-4f32, 1f32);
            aabb.slide(&mot, &v);
            assert_eq!(aabb, AABB2D::new(1f32, 1f32, 2f32, 2f32));
        }
    }
}
