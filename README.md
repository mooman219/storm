# Storm Engine
Playing around with opengl in rust.

## API Plan

### Input
```rust
// Fetches a new input event if one is present
input_poll() -> Option<InputEvent>
```

### Render
```rust
// Layer
layer_create(LayerDescription) -> LayerReference
layer_update(LayerReference, LayerDescription)
layer_remove(LayerReference)

// LayerDescription
struct LayerDescription {
  depth: u8 // 0: back, 255: front | Default: 0
  translation: Vector2<f32> // Default: 0, 0
  scale: f32 // Default: 1
  visible: bool // If the layer is rendered or not | Default: true
}

// Quad
quad_create(LayerReference, QuadDescription) -> QuadReference
quad_update(QuadReference, QuadDescription)
quad_remove(QuadReference)

// QuadDescription
struct QuadDescription {
  pos: Vector3<f32> // Default 0, 0, 0
  size: Vector2<f32> // Default: 1, 1
  color: Color // Default: White
  texture: TextureReference // Default: Blank
}

// Texture
texture_load(&str) -> TextureReference
texture_create(Vec<u8>) -> TextureReference
texture_default() -> TextureReference

// Window
window_title(&str)
window_commit()
```

### Engine
```rust
// If the input thread is running
engine_input_alive() -> bool

// If the render thread is running
engine_render_alive() -> bool

// Stops the input and render threads. Queues will no long be read from.
engine_shutdow()
```

### Example
```rust
fn main() {
  // Creates an engine instance
  let engine = storm::new();
  let layer = engine.layer_create(...); // Some layer description
  let quad = engine.quad_create(layer, ...); // Some quad description
  engine.window_commit();

  loop{
    match engine.input_poll() {
      Some(event) => {
        // if escape, break
      }
    }
  }
}
```
