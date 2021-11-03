use crate::glx::WindowSizeInfo;
use crate::shader_programs::points::{Point, Position, Velocity};
use crate::utils::calculate_relative_brightness;
use anyhow::{Context, Result};
use cgmath::{Basis2, Rad, Rotation, Rotation2, Vector2};
use image::io::Reader as ImageReader;
use image::RgbImage;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;
use rayon::prelude::*;

pub struct PointsSimulator {
    pub points: Vec<Point>,
    img: RgbImage,
    space_size: SimulationSpace,
}

#[derive(Copy, Clone)]
struct SimulationSpace {
    width: f32,
    height: f32,
}

impl PointsSimulator {
    pub fn new(window_info: WindowSizeInfo) -> Result<Self> {
        // let img = ImageReader::open("./assets/rustacean-happy.png")
        let img = ImageReader::open("./assets/pugs.jpg")
            .context("Failed to open background image")?
            .decode()
            .context("Failed to decode background image")?;
        let img = img.to_rgb8();
        let space_size = SimulationSpace {
            width: window_info.width as f32,
            height: window_info.height as f32,
        };

        let mut points = Vec::<Point>::with_capacity(200_000);
        points.resize(points.capacity(), Point::default());
        Self::internal_init_points(space_size, &mut points);

        Ok(PointsSimulator {
            points,
            img,
            space_size,
        })
    }

    pub fn init_points(&mut self) {
        Self::internal_init_points(self.space_size, &mut self.points);
    }

    pub fn update(&mut self) {
        let get_pixel_brightness = Self::get_pixel_brightness(&self.img, self.space_size);
        let periodize_point = Self::periodize_point(self.space_size);

        let vel_space = Range::new(0., 10.0);
        // let ang_space = Range::new(0., 6.28);
        let ang_space = Range::new(-1.0, 1.0);
        self.points.par_iter_mut().for_each(|p| {
            let mut rng = rand::thread_rng();
            let a = ang_space.ind_sample(&mut rng);
            let m = vel_space.ind_sample(&mut rng);
            p.velocity = Basis2::from_angle(Rad(a)).rotate_vector(Vector2::new(0., m))
                * (1.2 - get_pixel_brightness(p.position.x, p.position.y));
            p.position = periodize_point(p.position + p.velocity / 5.0);
        });
    }

    fn internal_init_points(space_size: SimulationSpace, points: &mut Vec<Point>) {
        // Random position initialization
        points.par_iter_mut().for_each(|p| {
            let mut rng = rand::thread_rng();
            *p = Point {
                position: Position {
                    x: rng.gen::<f32>() * space_size.width,
                    y: rng.gen::<f32>() * space_size.height,
                },
                velocity: Velocity { x: 0.0, y: 0.0 },
            }
        });

        // Circle initialization
        // let get_pos = |t: f32| Position {
        //     x: space_size.width * (0.5 + 0.4 * f32::cos(t)),
        //     y: space_size.height * (0.5 + 0.4 * f32::sin(t)),
        // };
        //
        // let mut v: f32 = 0.0;
        // points.resize_with(points.capacity(), || {
        //     v += 1.0;
        //     Point {
        //         position: get_pos(v),
        //         velocity: Velocity { x: 0.0, y: 0.0 },
        //     }
        // });
    }

    fn get_pixel_brightness<'a>(
        img: &'a RgbImage,
        space_size: SimulationSpace,
    ) -> impl Fn(f32, f32) -> f32 + 'a {
        let (img_width, img_height) = img.dimensions();
        move |x: f32, y: f32| {
            let pixel = img.get_pixel(
                u32::clamp(
                    (x / space_size.width * img_width as f32) as u32,
                    0,
                    img_width - 1,
                ),
                u32::clamp(
                    (y / space_size.height * img_height as f32) as u32,
                    0,
                    img_height - 1,
                ),
            );
            let v = pixel.0;
            calculate_relative_brightness(v[0], v[1], v[2])
        }
    }

    fn periodize_point(space_size: SimulationSpace) -> impl Fn(Position) -> Position {
        move |mut p: Position| -> Position {
            // 'while' version, seems to be faster than 'if' version
            while p.x < 0.0 {
                p.x += space_size.width
            }
            while p.x > space_size.width {
                p.x -= space_size.width
            }
            while p.y < 0.0 {
                p.y += space_size.height
            }
            while p.y > space_size.height {
                p.y -= space_size.height
            }
            p
        }
    }
}
