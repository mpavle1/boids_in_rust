use std::f64::consts::{FRAC_PI_2, PI, SQRT_2};

use piston_window::*;
use piston_window::{PistonWindow, WindowSettings};

use rand::*;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

const SCREEN_BUFFER: f64 = 15.0;
const NUMBER_OF_BOIDS: usize = 50;

const BLACK_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

#[derive(Copy, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Copy, Clone)]
struct Boid {
    point: Point,
    direction: f64,
    velocity: f64,
    color: [f32; 4],
}

fn are_two_points_in_range(a: Point, b: Point) -> bool {
    ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).sqrt() < 30.0
}

fn rotate_point(point: Point, angle_rad: f64, d: f64) -> Point {
    let cos_theta = angle_rad.cos();
    let sin_theta = angle_rad.sin();

    let new_x = point.x - d * cos_theta;
    let new_y = point.y - d * sin_theta;

    Point { x: new_x, y: new_y }
}

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
    // This functionality has been moved to struct Boids as i currenlty do not know how
    // to fix the mutable reference passing issue
    // fn update(&mut self, boids: &[Boid]) -> () {
    //     let point = rotate_point(self.point, self.direction, self.velocity);

    //     if point.x < SCREEN_BUFFER {
    //         self.direction = 1.0 * PI - self.direction;
    //     } else if point.y < SCREEN_BUFFER {
    //         self.direction = 2.0 * PI - self.direction;
    //     } else if point.x > (SCREEN_WIDTH as f64 - SCREEN_BUFFER) {
    //         self.direction = 1.0 * PI - self.direction;
    //     } else if point.y > (SCREEN_HEIGHT as f64 - SCREEN_BUFFER) {
    //         self.direction = 0.0 * PI - self.direction;
    //     }

    //     self.point = point;
    // }
}

struct Boids {
    boids: [Boid; NUMBER_OF_BOIDS],
}

impl Boids {
    fn new() -> Self {
        Self {
            boids: std::array::from_fn(|_| Boid::new()),
        }
    }

    fn update(&mut self) -> () {
        let boids = &mut self.boids;
        let number_of_boids = boids.len();

        for i in 0..number_of_boids {
            let point = rotate_point(boids[i].point, boids[i].direction, boids[i].velocity);

            // update boid position due to the edge of the screen
            if point.x < SCREEN_BUFFER {
                boids[i].direction = 1.0 * PI - boids[i].direction;
            } else if point.y < SCREEN_BUFFER {
                boids[i].direction = 2.0 * PI - boids[i].direction;
            } else if point.x > (SCREEN_WIDTH as f64 - SCREEN_BUFFER) {
                boids[i].direction = 1.0 * PI - boids[i].direction;
            } else if point.y > (SCREEN_HEIGHT as f64 - SCREEN_BUFFER) {
                boids[i].direction = 0.0 * PI - boids[i].direction;
            } else {
                let mut average_x = 0.0;
                let mut average_y = 0.0;

                let mut number_of_boids_in_range = 0;

                for j in 0..number_of_boids {
                    if boids[i].point == boids[j].point {
                        continue;
                    }

                    if are_two_points_in_range(boids[i].point, boids[j].point) {
                        number_of_boids_in_range += 1;
                        average_x += boids[j].point.x;
                        average_y += boids[j].point.y;
                    }
                }

                // Proveriti logiku. Trenutno izgleda kao da delimicno radi. Tj ako imamo
                // samo ovo pravilo onda bi trebali da se grupisu jedni na druge u slucaju da je update
                // u svakoj iteraciji.
                if number_of_boids_in_range > 0 {
                    average_x = average_x / number_of_boids as f64;
                    average_y = average_y / number_of_boids as f64;

                    let new_angle =
                        (average_y - boids[i].point.y).atan2(average_x - boids[i].point.x);

                    let new_direction = (new_angle - boids[i].direction) / 300.0;

                    boids[i].direction += new_direction;
                }
            }

            boids[i].point = point;
        }
    }

    fn draw(&self, ctx: &Context, g: &mut G2d) {
        for boid in self.boids {
            boid.draw(&ctx, g);
        }
    }
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Boids... now in rust", [SCREEN_WIDTH, SCREEN_HEIGHT])
            .resizable(false)
            .build()
            .unwrap();

    let boids = &mut Boids::new();

    // dodati proveri za delta time, tako da imamo konstantan FPS

    while let Some(event) = window.next() {
        window.draw_2d(&event, |ctx, g, _| {
            clear(BLACK_COLOR, g);

            boids.update();
            boids.draw(&ctx, g);
        });
    }
}
