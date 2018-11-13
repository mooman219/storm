extern crate image;
extern crate storm;

use std::fs::File;
use std::path::Path;

use storm::render::texture::*;

/// Run with: cargo run --example packer --release
fn main() {
    let mut packer = TexturePacker::new(TexturePackerConfig {
        max_width: 1024,
        max_height: 1024,
        texture_padding: 0,
    });

    // Pack
    for i in 1..11 {
        let name = format!("{}.png", i);
        let path = format!("examples/packer/assets/{}", name);
        let rect = packer.pack_path(&Path::new(&path));
        println!("  {:7} : {:?}", name, rect);
    }

    // Save the result
    let exporter = packer.export().to_dynamic_image().unwrap();
    let mut file = File::create("examples/packer/output.png").unwrap();
    exporter.write_to(&mut file, image::PNG).unwrap();
}
