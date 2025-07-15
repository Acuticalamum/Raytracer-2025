use crate::bvh::BVHNode;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Material, Metal};
use crate::texture::{ImageTexture, SolidColor};
use crate::triangle::Triangle;
use crate::vec3::Point3;
use std::sync::Arc;
use tobj;

pub fn load_obj_model(path: &str, scale: f64) -> HittableList {
    let mut object = HittableList::new();

    let (models, materials) = tobj::load_obj(
        &path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: false,
            ignore_points: true,
            ignore_lines: true,
        },
    )
    .expect("Failed to load OBJ file");

    let default_mat = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.8, 0.8, 0.8,
    )))));

    let materials = materials.expect("Failed to load MTL file");
    for model in models.iter() {
        let mesh = &model.mesh;
        let cur_mat: Arc<dyn Material> = match mesh.material_id {
            Some(id) => {
                let mat = materials[id].clone();
                if let Some(diffuse) = mat.diffuse {
                    Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
                        diffuse[0] as f64,
                        diffuse[1] as f64,
                        diffuse[2] as f64,
                    )))))
                } else if let Some(specular) = mat.specular {
                    let shininess = mat.shininess.unwrap_or(0.0);
                    let fuzz = 1.0 - (shininess / 1000.0).clamp(0.0, 1.0) as f64;
                    Arc::new(Metal::new(
                        Color::new(specular[0] as f64, specular[1] as f64, specular[2] as f64),
                        fuzz,
                    ))
                } else if let Some(diffuse_texture) = &mat.diffuse_texture {
                    let tex = Arc::new(ImageTexture::new(diffuse_texture));
                    Arc::new(Lambertian::new(tex.clone()))
                } else {
                    default_mat.clone()
                }
            }
            None => default_mat.clone(),
        };
        let positions = &mesh.positions;
        let indices = &mesh.indices;
        for idx in 0..indices.len() / 3 {
            let idx0 = indices[idx * 3 + 0] as usize;
            let idx1 = indices[idx * 3 + 1] as usize;
            let idx2 = indices[idx * 3 + 2] as usize;
            let (v0, v1, v2) = (
                Point3::new(
                    positions[idx0 * 3 + 0] as f64 * scale,
                    positions[idx0 * 3 + 1] as f64 * scale,
                    positions[idx0 * 3 + 2] as f64 * scale,
                ),
                Point3::new(
                    positions[idx1 * 3 + 0] as f64 * scale,
                    positions[idx1 * 3 + 1] as f64 * scale,
                    positions[idx1 * 3 + 2] as f64 * scale,
                ),
                Point3::new(
                    positions[idx2 * 3 + 0] as f64 * scale,
                    positions[idx2 * 3 + 1] as f64 * scale,
                    positions[idx2 * 3 + 2] as f64 * scale,
                ),
            );
            let triangle = Triangle::new_with_points(v0, v1, v2, cur_mat.clone());
            object.add(Arc::new(triangle));
        }
    }
    let object = HittableList::from(Arc::new(BVHNode::new_from_list(&mut object)));
    object
}
