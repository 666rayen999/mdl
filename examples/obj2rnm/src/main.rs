use std::collections::HashMap;

fn main() -> Result<(), ()> {
    let path = "model.obj"; // should be in the main directory

    let (meshes, materials) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ignore_points: true,
            ignore_lines: true,
        },
    )
    .map_err(|e| eprintln!("[ERROR]: {e:?}"))?;

    let mut textures = HashMap::<String, (u8, rnm_3d::Image)>::new();

    let materials = materials.ok().map(|materials| {
        materials
            .into_iter()
            .map(|material| {
                let albedo = material
                    .diffuse_texture
                    .map(|url| load_texture(url, &mut textures))
                    .flatten()
                    .map(rnm_3d::Uniform::Texture)
                    .unwrap_or(rnm_3d::Uniform::Color(material.diffuse.unwrap_or_default()));
                let normal_texture = material
                    .normal_texture
                    .map(|url| load_texture(url, &mut textures))
                    .flatten();
                let roughness = material.shininess.unwrap_or(0.5) / 1000.0;
                let mre = rnm_3d::Mre::Value {
                    metallic: 0.0,
                    roughness,
                    emission: 0.0,
                };
                let specular = material
                    .specular
                    .map(|[r, g, b]| r.max(g).max(b))
                    .unwrap_or(0.5);

                rnm_3d::Material::new(albedo, mre, normal_texture, specular)
            })
            .collect()
    });

    let meshes = meshes
        .into_iter()
        .map(|mesh| {
            let positions = to_vec3(mesh.mesh.positions)
                .expect("[ERROR]: mesh \"{}\" doesnt have position attribute!");
            let tex_coords = to_vec2(mesh.mesh.texcoords);
            let normals = to_vec3(mesh.mesh.normals);
            let colors = to_vec3(mesh.mesh.vertex_color);
            let indices = to_vec3u(mesh.mesh.indices);
            let material = mesh
                .mesh
                .material_id
                .map(|x| u8::try_from(x).ok())
                .flatten();

            rnm_3d::Mesh::new(
                positions, tex_coords, normals, colors, None, None, indices, material,
            )
        })
        .collect();

    let images = {
        if textures.is_empty() {
            None
        } else {
            let mut images = textures.into_values().collect::<Vec<_>>();
            images.sort_unstable_by_key(|(idx, _)| *idx);
            Some(images.into_iter().map(|(_, image)| image).collect())
        }
    };

    let scene = rnm_3d::Scene::new(meshes, materials, None, images)
        .expect("[ERROR]: failed to create scene!");
    scene.save("output.rnm")?;
    println!("[INFO] scene saved!");

    println!("meshes: {}", scene.meshes().map(|v| v.len()).unwrap_or(0));
    println!("mats: {}", scene.materials().map(|v| v.len()).unwrap_or(0));
    println!("lights: {}", scene.lights().map(|v| v.len()).unwrap_or(0));
    println!("textures: {}", scene.images().map(|v| v.len()).unwrap_or(0));

    Ok(())
}

fn to_vec3(vector: Vec<f32>) -> Option<Vec<[f32; 3]>> {
    let v: Option<Vec<[f32; 3]>> = vector
        .windows(3)
        .step_by(3)
        .map(|w| {
            if let (Some(a), Some(b), Some(c)) = (w.get(0), w.get(1), w.get(2)) {
                Some([*a, *b, *c])
            } else {
                None
            }
        })
        .collect();
    if v.as_ref().map(Vec::len).unwrap_or(0) == 0 {
        None
    } else {
        v
    }
}

fn to_vec2(vector: Vec<f32>) -> Option<Vec<[f32; 2]>> {
    let v: Option<Vec<[f32; 2]>> = vector
        .windows(2)
        .step_by(2)
        .map(|w| {
            if let (Some(a), Some(b)) = (w.get(0), w.get(1)) {
                Some([*a, *b])
            } else {
                None
            }
        })
        .collect();
    if v.as_ref().map(Vec::len).unwrap_or(0) == 0 {
        None
    } else {
        v
    }
}

fn to_vec3u(vector: Vec<u32>) -> Option<Vec<[u16; 3]>> {
    let v: Option<Vec<[u16; 3]>> = vector
        .windows(3)
        .step_by(3)
        .map(|w| {
            if let (Some(a), Some(b), Some(c)) = (
                w.get(0).map(|x| u16::try_from(*x).ok()).flatten(),
                w.get(1).map(|x| u16::try_from(*x).ok()).flatten(),
                w.get(2).map(|x| u16::try_from(*x).ok()).flatten(),
            ) {
                Some([a, b, c])
            } else {
                None
            }
        })
        .collect();
    if v.as_ref().map(Vec::len).unwrap_or(0) == 0 {
        None
    } else {
        v
    }
}

fn load_texture(url: String, textures: &mut HashMap<String, (u8, rnm_3d::Image)>) -> Option<u8> {
    if let Some((idx, _)) = textures.get(&url) {
        Some(*idx)
    } else {
        let img = image::load(
            &mut std::io::BufReader::new(std::io::Cursor::new(std::fs::read(&url).ok()?)),
            image::ImageFormat::from_path(&url).ok()?,
        )
        .ok()?;

        let idx = textures.len().try_into().ok()?;

        if img.color().has_alpha() {
            textures.insert(
                url,
                (
                    idx,
                    rnm_3d::Image::from_raw(img.width(), img.height(), &img.to_rgba8()).unwrap(),
                ),
            );
        } else {
            textures.insert(
                url,
                (
                    idx,
                    rnm_3d::Image::from_raw(img.width(), img.height(), &img.to_rgb8()).unwrap(),
                ),
            );
        };
        Some(idx)
    }
}
