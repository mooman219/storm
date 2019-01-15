const PI: f32 = 3.141592653589793238f32;
const SIN_BITS: i32 = 11i32;
const SIN_MASK: i32 = !(-1i32 << SIN_BITS);
const SIN_COUNT: i32 = SIN_MASK + 1i32;
const RAD_FULL: f32 = PI * 2f32;
const DEG_FULL: f32 = 360f32;
const DEG_INDEX: f32 = (SIN_COUNT as f32) / DEG_FULL;
const SIZE: usize = (SIN_COUNT as usize);

fn main() {
    {
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
        print(&sin, "SIN");
    }
    {
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
        print(&cos, "COS");
    }
}

fn print(array: &[f32; SIZE], var: &str) {
    let mut n = 0;
    println!("#[cfg_attr(rustfmt, rustfmt_skip)]");
    print!("static {}: [f32; SIZE] = [", var);
    while n < SIZE {
        print!("{}f32, ", array[n]);
        n += 1;
        if n % 512 == 0 {
            println!("");
        }
    }
    println!("];");
}
