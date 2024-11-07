# **RNM**: Blazingly Fast + Tiny 3D Format

its 3d format for my game.\
how is it blazigly fast ?\
thanks to:
- [rkyv](https://github.com/rkyv/rkyv) and zero-copy deserialization.
- [lz4_flex](https://github.com/pseitz/lz4_flex) for realtime decompression.
- [qoi_rs](https://github.com/aldanor/qoi-rust) for ultra fast image format.

<kbd>
60% smaller | 300% faster
</kbd>

## Types:

#### Scene:

```rust
Scene {
    meshes: [Mesh]?,
    materials: [Material]?,
    lights: [Light]?,
    textures: [ImageData]?, // QOI format
}
```

#### Mesh:

```rust
Mesh {
    positions: [Vec3],
    uvs: [Vec2]?,
    normals: [Vec3]?,
    colors: [Color]?,
    tangents: [Vec3]?,
    bitangents: [Vec3]?,
    indices: [3 x u16]?, // must be triangles
    material: u8?, // material index in scene
}
```

#### Material:

```rust
Material {
    albedo: Color | u8,
    metallic_roughness_emission: (m: f32, r: f32, e: f32) | u8,
    normal_texture: u8?,
    specular: f32,
}
// all u8 are textures index in scene
```

#### Lights:

```rust
Light (
    Point {
        position: Vec3,
        color: Color,
        power: f32,
        radius: f32,
    } | Sun {
        direction: Vec3,
        color: Color,
        power: f32,
    } | Spot {
        position: Vec3,
        direction: Vec3,
        color: Color,
        power: f32,
        radius: f32,
        blend: f32,
    }
)
```

#### Image:

```rust
Image {
    width: u32,
    height: u32,
    data: [u8],
}
```

## Example:

```rust
// from raw data:
let positions = vec![...];
let colors = vec![...];
let indices = vec![...];
let mesh = Mesh::new(positions, None, None, Some(colors), None, None, Some(indices), None).unwrap();
let meshes = vec![mesh];
let scene = Scene::new(Some(meshes), None, None, None).unwrap();
// save:
scene.save(path).unwrap();
// load:
let scene = Scene::load(include_bytes!(path)).unwrap();         // embedded
let scene = Scene::load(std::fs::read(path).unwrap()).unwrap(); // from file
```

## Todo:
- [ ] Documentation
- [ ] Create a tool to convert between RNM and other formats (.obj, .gltf, .fbx)
- [ ] Animation support
- [X] LZ4 Compression (realtime decompression)
- [X] Features
- [ ] Other languages support

## Contributing:

Feel free to open issues or submit pull requests to improve the format.

## License:

This project is licensed under the MIT License.
