use std;

/// Average error of 4%.
/// Speedup of 1.02x over f32.sqrt();
#[inline]
pub fn sqrt(value: f32) -> f32 {
    unsafe {
        let mut n = std::mem::transmute::<f32, i32>(value);
        n += 127 << 23;
        n >>= 1;
        std::mem::transmute::<i32, f32>(n)
    }
}

const PI: f32 = 3.141592653589793238f32;
const PI_2: f32 = 1.570796326794896619f32;
const PI_NEG_2: f32 = -1.570796326794896619f32;
const CONST: f32 = 0.28087f32; // Trial and error

/// Average error of 0.00231 radians.
/// Largest error of 0.00488 radians.
/// Speedup of 20.67x over f32.atan2(y);
#[inline]
pub fn atan2(y: f32, x: f32) -> f32 {
    if x == 0f32 {
        if y > 0f32 {
            return PI_2;
        }
        if y == 0f32 {
            return 0f32;
        }
        return PI_NEG_2;
    }
    let z: f32 = y / x;
    let atan: f32;
    if z.abs() < 1f32 {
        atan = z / (z * z * CONST + 1f32);
        if x < 0f32 {
            if y < 0f32 {
                return atan - PI;
            }
            return atan + PI;
        }
        return atan;
    }
    atan = PI_2 - z / (z * z + CONST);
    if y < 0f32 {
        return atan - PI;
    }
    return atan;
}

const SIN_BITS: i32 = 11i32;
const SIN_MASK: i32 = !(-1i32 << SIN_BITS);
const SIN_COUNT: i32 = SIN_MASK + 1i32;
const RAD_FULL: f32 = PI * 2f32;
const DEG_FULL: f32 = 360f32;
const RAD_INDEX: f32 = (SIN_COUNT as f32) / RAD_FULL;
const DEG_INDEX: f32 = (SIN_COUNT as f32) / DEG_FULL;
const SIZE: usize = (SIN_COUNT as usize);
lazy_static! {
    static ref SIN: [f32; SIZE] = {
        let mut sin = [0f32; SIZE];
        for n in 0..SIZE {
            sin[n] = (((n as f32) + 0.5f32) / (SIN_COUNT as f32) * RAD_FULL).sin();
        }
        let mut n = 0;
        while n < 360 {
            let i = ((((n as f32) * DEG_INDEX) as i32) & SIN_MASK) as usize;
            sin[i] = ((n as f32) * PI / 180f32).sin();
            n += 90;
        }
        sin
    };
    static ref COS: [f32; SIZE] = {
        let mut cos = [0f32; SIZE];
        for n in 0..SIZE {
            cos[n] = (((n as f32) + 0.5f32) / (SIN_COUNT as f32) * RAD_FULL).cos();
        }
        let mut n = 0;
        while n < 360 {
            let i = ((((n as f32) * DEG_INDEX) as i32) & SIN_MASK) as usize;
            cos[i] = ((n as f32) * PI / 180f32).cos();
            n += 90;
        }
        cos
    };
}

/// Average error of 0.00060 radians.
/// Largest error of 0.00229 radians.
/// Speedup of 17.98x over f32.sin();
/// Input in radians, output in radians.
#[inline]
pub fn sin_rad(rad: f32) -> f32 {
    unsafe { *SIN.get_unchecked((((rad * RAD_INDEX) as i32) & SIN_MASK) as usize) }
}

/// Average error of 0.00060 radians.
/// Largest error of 0.00229 radians.
/// Speedup of 17.98x over f32.cos();
/// Input in radians, output in radians.
#[inline]
pub fn cos_rad(rad: f32) -> f32 {
    unsafe { *COS.get_unchecked((((rad * RAD_INDEX) as i32) & SIN_MASK) as usize) }
}

/// Average error of 0.00060 radians.
/// Largest error of 0.00229 radians.
/// Speedup of 17.98x over f32.sin();
/// Input in degrees, output in radians.
#[inline]
pub fn sin_deg(deg: f32) -> f32 {
    unsafe { *SIN.get_unchecked((((deg * DEG_INDEX) as i32) & SIN_MASK) as usize) }
}

/// Average error of 0.00060 radians.
/// Largest error of 0.00229 radians.
/// Speedup of 17.98x over f32.cos();
/// Input in degrees, output in radians.
#[inline]
pub fn cos_deg(deg: f32) -> f32 {
    unsafe { *COS.get_unchecked((((deg * DEG_INDEX) as i32) & SIN_MASK) as usize) }
}
