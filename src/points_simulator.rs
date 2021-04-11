use crate::glx::WindowSizeInfo;
use crate::shader_programs::points::{Point, Position};
use crate::utils::calculate_relative_brightness;
use cgmath::{Basis2, Point2, Rad, Rotation, Rotation2, Vector2, Zero};
use image::io::Reader as ImageReader;
use image::RgbImage;
use rand::distributions::{IndependentSample, Range};
use rand::ThreadRng;

pub struct PointsSimulator {
    pub points: Vec<Point>,
    img: RgbImage,
    rng: ThreadRng,
}

// use image::DynamicImage::ImageBgr8;

impl PointsSimulator {
    pub fn new(window_info: WindowSizeInfo) -> Self {
        let img = ImageReader::open("./assets/rustacean-happy.png")
            .unwrap()
            .decode()
            .unwrap();
        let img = img.to_rgb8();

        let mut points = Vec::<Point>::with_capacity(10_000);
        let get_pos = |t: f32| Point2 {
            x: (window_info.width as f32) * (0.5 + 0.4 * f32::cos(t)),
            y: (window_info.height as f32) * (0.5 + 0.4 * f32::sin(t)),
        };

        let mut v: f32 = 0.0;
        points.resize_with(points.capacity(), || {
            v += 1.0;
            Point {
                position: get_pos(v),
                velocity: Vector2::zero(),
            }
        });

        PointsSimulator {
            points,
            img,
            rng: rand::thread_rng(),
        }
    }

    pub fn update(&mut self, window_size: (u32, u32)) {
        let get_pixel_brightness = Self::get_pixel_brightness(&self.img, window_size);
        let periodize_point = Self::periodize_point(window_size);

        let vel_space = Range::new(0., 5.0);
        // let ang_space = Range::new(0., 6.28);
        let ang_space = Range::new(-1.0, 1.0);
        for p in &mut self.points {
            let a = ang_space.ind_sample(&mut self.rng);
            let m = vel_space.ind_sample(&mut self.rng);
            p.velocity = Basis2::from_angle(Rad(a)).rotate_vector(Vector2::new(0., m))
                * (1.0 - get_pixel_brightness(p.position.x, p.position.y));
            p.position = periodize_point(p.position + p.velocity / 5.0);
        }
    }

    fn get_pixel_brightness<'a>(
        img: &'a RgbImage,
        window_size: (u32, u32),
    ) -> impl Fn(f32, f32) -> f32 + 'a {
        let (width, height) = window_size;
        let (img_width, img_height) = img.dimensions();
        move |x: f32, y: f32| {
            let pixel = img.get_pixel(
                u32::clamp(
                    (x / width as f32 * img_width as f32) as u32,
                    0,
                    img_width - 1,
                ),
                u32::clamp(
                    (y / height as f32 * img_height as f32) as u32,
                    0,
                    img_height - 1,
                ),
            );
            let v = pixel.0;
            calculate_relative_brightness(v[0], v[1], v[2])
        }
    }

    fn periodize_point(window_size: (u32, u32)) -> impl Fn(Position) -> Position {
        let (width, height) = (window_size.0 as f32, window_size.1 as f32);
        move |mut p: Position| -> Position {
            // 'while' version, seems to be faster than 'if' version
            while p.x < 0.0 {
                p.x += width
            }
            while p.y < 0.0 {
                p.y += height
            }
            while p.x > width {
                p.x -= width
            }
            while p.y > height {
                p.y -= height
            }
            p
        }
    }
}
