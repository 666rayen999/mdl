fn main() {
    let (document, buffers, images) = gltf::import("model.gltf").unwrap();

    let images = images
        .into_iter()
        .filter_map(|image| match image.format {
            gltf::image::Format::R8G8B8 | gltf::image::Format::R8G8B8A8 => {
                rnm_3d::Image::from_raw(image.width, image.height, &image.pixels).ok()
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    let materials: Vec<rnm_3d::Material> = document
        .materials()
        .map(|material| {
            let albedo = material
                .pbr_metallic_roughness()
                .base_color_texture()
                .map(|t| u8::try_from(t.texture().source().index()).ok())
                .flatten()
                .map(|t| rnm_3d::Uniform::Texture(t))
                .unwrap_or({
                    let color = material.pbr_metallic_roughness().base_color_factor();
                    let color = [color[0], color[1], color[2]];
                    rnm_3d::Uniform::Color(color)
                });
            let mre = material
                .pbr_metallic_roughness()
                .metallic_roughness_texture()
                .map(|t| u8::try_from(t.texture().source().index()).ok())
                .flatten()
                .map(|t| rnm_3d::Mre::Texture(t))
                .unwrap_or({
                    let metallic = material.pbr_metallic_roughness().metallic_factor();
                    let roughness = material.pbr_metallic_roughness().roughness_factor();
                    let emission = material.emissive_factor();
                    let emission = emission[0].max(emission[1]).max(emission[2]);
                    rnm_3d::Mre::Value {
                        metallic,
                        roughness,
                        emission,
                    }
                });
            let normal = material
                .normal_texture()
                .map(|t| u8::try_from(t.texture().source().index()).ok())
                .flatten();
            rnm_3d::Material::new(albedo, mre, normal, 0.5)
        })
        .collect();

    let meshes: Option<Vec<rnm_3d::Mesh>> = document
        .meshes()
        .map(|mesh| {
            Vec::from_iter(
                mesh.primitives()
                    .map(|primitive| load_mesh(primitive, &buffers)),
            )
        })
        .flatten()
        .collect();

    let scene = rnm_3d::Scene::new(meshes, Some(materials), None, Some(images)).unwrap();
    scene.save("model.rnm").unwrap();
}

fn load_mesh(mesh: gltf::Primitive, buffers: &[gltf::buffer::Data]) -> Option<rnm_3d::Mesh> {
    let reader = mesh.reader(|buffer| Some(&*buffers[buffer.index()]));
    let indices = if let Some(indices) = reader.read_indices() {
        let indices = indices
            .into_u32()
            .map(|i| u16::try_from(i).ok())
            .collect::<Option<Vec<u16>>>()?;

        let indices = indices
            .windows(3)
            .step_by(3)
            .map(|x| (x.get(0), x.get(1), x.get(2)))
            .map(|(a, b, c)| {
                if let (Some(a), Some(b), Some(c)) = (a, b, c) {
                    Some([*a, *b, *c])
                } else {
                    None
                }
            })
            .collect::<Option<Vec<[u16; 3]>>>()?;

        Some(indices)
    } else {
        None
    };
    let positions: Vec<[f32; 3]> = reader.read_positions()?.collect();
    let normals: Option<Vec<[f32; 3]>> = reader.read_normals().map(Iterator::collect);
    let colors: Option<Vec<[f32; 3]>> = reader
        .read_colors(0)
        .map(|vec| vec.into_rgb_f32().collect());
    let uvs: Option<Vec<[f32; 2]>> = reader
        .read_tex_coords(0)
        .map(|vec| vec.into_f32().collect());
    let tangents: Option<Vec<[f32; 4]>> = reader.read_tangents().map(|vec| vec.collect());
    let material = mesh
        .material()
        .index()
        .map(|x| u8::try_from(x).ok())
        .flatten();

    rnm_3d::Mesh::new(positions, uvs, normals, colors, tangents, indices, material)
}
