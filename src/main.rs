use std::f64::consts::{FRAC_PI_2, PI};

use piston_window::*;
use piston_window::{PistonWindow, WindowSettings};

use rand::*;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

const SCREEN_BUFFER: f64 = 15.0;

const BLACK_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Copy, Clone)]
struct Boid {
    point: Point,
    direction: f64, // Privremeno dok ne smislim kako cu ovo da pratim
    velocity: f64,
    color: [f32; 4],
}

fn rotate_point(point: Point, angle_rad: f64, d: f64) -> Point {
    let cos_theta = angle_rad.cos();
    let sin_theta = angle_rad.sin();

    let new_x = point.x - d * cos_theta;
    let new_y = point.y - d * sin_theta;

    Point { x: new_x, y: new_y }
}

fn center_of_mass_rule(boid: &mut Boid, all_boids: Vec<Boid>) -> () {}

impl Boid {
    fn new() -> Self {
        let mut random = rand::thread_rng();

        Self {
            point: Point {
                x: random.gen_range(100..=(SCREEN_WIDTH - 100)) as f64,
                y: random.gen_range(100..=(SCREEN_HEIGHT - 100)) as f64,
            },
            velocity: 1.0,
            direction: PI * random.gen_range(0.0..=(2.0 * PI)),
            color: [
                random.gen_range(0.0..=1.0),
                random.gen_range(0.0..=1.0),
                random.gen_range(0.0..=1.0),
                1.0,
            ],
        }
    }

    fn draw(&self, ctx: &Context, g: &mut G2d) {
        let angle = self.direction;

        let angle_rad = angle;

        let a: Point = rotate_point(self.point, FRAC_PI_2 + angle_rad, 5.0);
        let b: Point = rotate_point(self.point, FRAC_PI_2 + angle_rad, -5.0);
        let c: Point = rotate_point(self.point, angle_rad, 15.0);

        polygon(
            self.color,
            &[[a.x, a.y], [b.x, b.y], [c.x, c.y]],
            ctx.transform,
            g,
        );
    }

    fn update(&mut self, boids: Vec<Boid>) -> () {
        let point = rotate_point(self.point, self.direction, self.velocity);

        if point.x < SCREEN_BUFFER {
            self.direction = 1.0 * PI - self.direction;
        } else if point.y < SCREEN_BUFFER {
            self.direction = 2.0 * PI - self.direction;
        } else if point.x > (SCREEN_WIDTH as f64 - SCREEN_BUFFER) {
            self.direction = 1.0 * PI - self.direction;
        } else if point.y > (SCREEN_HEIGHT as f64 - SCREEN_BUFFER) {
            self.direction = 0.0 * PI - self.direction;
        }

        self.point = point;
    }
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Boids... now in rust", [SCREEN_WIDTH, SCREEN_HEIGHT])
            .resizable(false)
            .build()
            .unwrap();

    let mut boids: Vec<Boid> = Vec::new();

    for _ in 0..=75 {
        boids.push(Boid::new());
    }

    // dodati proveri za delta time, tako da imamo konstantan FPS

    let boid_ref: &mut Vec<Boid> = &mut boids;

    while let Some(event) = window.next() {
        window.draw_2d(&event, |ctx, g, _| {
            clear(BLACK_COLOR, g);

            for i in 0..boid_ref.len() {
                let vec = boid_ref.to_vec();
                boid_ref[i].update(vec);
                boid_ref[i].draw(&ctx, g);
            }
        });
    }
}
