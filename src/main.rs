use std::sync::{Arc, Mutex};
use std::thread;

use image::{ImageBuffer, Rgb, RgbImage};

use geometry::color::Color;
use geometry::ray::Ray;
use geometry::vector::{Point, Vector3};
use materials::dielectric::Dielectric;
use materials::lambertian::Lambertian;
use materials::metal::Metal;
use objects::sphere::{MovingSphere, Sphere};
use utils::{random_f32, random_f32_range};
use utils::INF_F32;
use world::camera::Camera;

use crate::objects::bvh_node::{BVHNode, Node};
use crate::objects::hittable::Hittable;
use crate::textures::checkered::CheckeredTexture;
use crate::textures::solid::SolidColor;

mod geometry;
mod objects;
mod world;
mod materials;
mod utils;
mod textures;

fn ray_color(ray: Ray, world: Arc<dyn Node>, depth: u32) -> Color {
    if depth == 0 {
        return Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
    }

    let hit_rec = world.hit(&ray, 0.01, INF_F32);
    if hit_rec.is_some() {
        let rec = hit_rec.unwrap();
        let object = rec.object;
        let color = object.color(rec.intersection);
        let scattered = object.scatter(ray, rec.intersection);
        return color * ray_color(scattered, world, depth - 1);
    }

    let t = 0.5 * (ray.direction.y + 1.0);
    (1.0 - t) * Color { r: 1.0, g: 1.0, b: 1.0, } + t * Color { r: 0.5, g: 0.7, b: 1.0, }
}

fn scene() -> Vec<Arc<dyn Hittable>> {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    // Ground
    world.push(Arc::new(Sphere {
        center: Point {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(CheckeredTexture {
                even: Arc::new(SolidColor {
                    color: Color { r: 0.2, g: 0.3, b: 0.1 }
                }),
                odd: Arc::new(SolidColor {
                    color: Color { r: 0.9, g: 0.9, b: 0.9 }
                }),
            }),
        }),
    }));

    // Spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32();
            let center = Point { x: a as f32 + 0.9 * random_f32(), y: 0.2, z: b as f32 + 0.9 * random_f32() };

            if (center - Point { x: 4.0, y: 0.2, z: 0.0 }).length() > 0.9 {
                if choose_mat < 0.8 {
                    let color = Color::random() * Color::random();
                    world.push(Arc::new(MovingSphere {
                        centre0: center,
                        center1: center + Vector3 {x: 0.0, y: random_f32() / 4.0, z: 0.0},
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Arc::new(Lambertian {
                            albedo: Arc::new(SolidColor {
                                color,
                            }),
                        }),
                    }))
                } else if choose_mat < 0.95 {
                    let color = 0.5 * Color::random() + Color { r: 0.5, g: 0.5, b: 0.5 } ;
                    let fuzz = random_f32_range(0.0, 0.5);
                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal {
                            color,
                            fuzz,
                        })
                    }))
                } else {
                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectric {
                            refractive_index: 1.5,
                        })
                    }));
                }
            }
        }
    }

    world.push(Arc::new(Sphere {
        center: Point {x: 0.0, y: 1.0, z: 0.0 },
        radius: 1.0,
        material: Arc::new(Dielectric {
            refractive_index: 1.5,
        })
    }));

    world.push(Arc::new(Sphere {
        center: Point {x: -4.0, y: 1.0, z: 0.0 },
        radius: 1.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(SolidColor {
                color: Color {
                    r: 0.4,
                    g: 0.2,
                    b: 0.1,
                }
            }),
        }),
    }));

    world.push(Arc::new(Sphere {
        center: Point {x: 4.0, y: 1.0, z: 0.0 },
        radius: 1.0,
        material: Arc::new(Metal {
            color: Color {
                r: 0.7,
                g: 0.6,
                b: 0.5
            },
            fuzz: 0.0,
        })
    }));

    world
}

struct ImageBlockInfo {
    start_row: u32,
    end_row: u32,
    image_height: u32,
    image_width: u32,
    spp: u32,
    max_depth: u32,
    image_block: Vec<Vec<Rgb<u8>>>,
}

fn process_block(mut block_info: ImageBlockInfo, image_blocks: Arc<Mutex<Vec<ImageBlockInfo>>>, camera: Camera, world: Arc<dyn Node>) {
    for j in block_info.start_row..block_info.end_row {
        let mut row: Vec<Rgb<u8>> = Vec::with_capacity(block_info.image_width as usize) ;
        for i in 0..block_info.image_width {
            let mut pixel_color = Color { r: 0.0, g: 0.0, b: 0.0 };
            for _ in 0..block_info.spp {
                let u = (i as f32 + random_f32()) / (block_info.image_width - 1) as f32;
                let v = (j as f32 + random_f32()) / (block_info.image_height - 1) as f32;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(ray, world.clone(), block_info.max_depth);
            }
            row.push(pixel_color.get_pixel(block_info.spp));
        }
        block_info.image_block.push(row);
    }
    
    let mut image = image_blocks.lock().unwrap();
    image.push(block_info);
}

fn main() {
    // Image
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = 800;
    const ASPECT_RATIO: f32 = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;


    //Camera
    let look_from = Point { x: 13.0, y: 2.0, z: 3.0 };
    let look_at = Point { x: 0.0, y: 0.0, z: 0.0 };
    let v_up = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
    let v_fov = 20.0;
    let aperture = 0.0;
    let focus_dist = 10.0;

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        v_fov,
        ASPECT_RATIO,
        aperture,
        focus_dist,
        0.0,
        1.0,
    );


    // World
    let world = BVHNode::create_tree(&mut scene(), 0.0, 1.0);


    // Render
    const NTHREADS: u32 = 8;
    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();
    let image_blocks: Arc<Mutex<Vec<ImageBlockInfo>>> = Arc::new(Mutex::new(Vec::new()));

    let block_size = IMAGE_HEIGHT / NTHREADS;
    let end_block_size = block_size + (IMAGE_HEIGHT % NTHREADS);

    for i in 0..NTHREADS {
        let block_info = ImageBlockInfo {
            start_row: i * block_size,
            end_row: i * block_size + ( if i == NTHREADS - 1 { end_block_size } else { block_size } ),
            image_height: IMAGE_HEIGHT,
            image_width: IMAGE_WIDTH,
            spp: SAMPLES_PER_PIXEL,
            max_depth: MAX_DEPTH,
            image_block: Vec::with_capacity(block_size as usize),
        };

        let camera_new = camera.clone();
        let image_blocks_new = image_blocks.clone();
        let world_new = world.clone();

        let handle = thread::spawn(|| {
            process_block(block_info, image_blocks_new, camera_new, world_new);
        });
        threads.push(handle);
    }

    for thread in threads.into_iter() {
        thread.join().unwrap();
    }

    let final_blocks = image_blocks.lock().unwrap();
    let mut img_buf: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for block in final_blocks.iter() {
        for y in 0..block.image_block.len() {
            for x in 0..block.image_block[0].len() {
                let u = x as u32;
                let v = block.start_row + y as u32;
                img_buf.put_pixel(u, v, block.image_block[y][x]);
            }
        }
    }
    img_buf.save("render.png").unwrap();
}
