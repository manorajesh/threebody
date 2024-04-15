use macroquad::prelude::*;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

const G: f32 = 1.0e-1;
const NUM_OF_BODIES: usize = 10000;
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const FRICTION: f32 = 0.99;
const MAX_VELOCITY: f32 = 10.0;

#[derive(Clone, Copy, PartialEq, Debug)]
struct Body {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    force: Vec2,
    mass: f32,
    radius: f32,
    freezed: bool,
}

impl Body {
    fn new(initial_pos: Vec2) -> Self {
        Body {
            position: initial_pos,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            force: Vec2::ZERO,
            mass: 1000.0,
            radius: 10.0,
            freezed: false,
        }
    }

    fn random(initial_pos: Option<Vec2>) -> Self {
        let position = initial_pos.unwrap_or(
            vec2(rand::gen_range(0.0, SCREEN_WIDTH), rand::gen_range(0.0, SCREEN_HEIGHT))
        );
        let velocity = vec2(
            rand::gen_range(-0.5, 0.5), // Random initial velocities
            rand::gen_range(-0.5, 0.5)
        );
        let mass = rand::gen_range(500.0, 1500.0); // Random mass between 500 and 1500
        let radius = mass / 150.0;

        Body {
            position,
            velocity,
            acceleration: Vec2::ZERO,
            force: Vec2::ZERO,
            mass,
            radius,
            freezed: false,
        }
    }

    fn get_distance(&self, other_body: &Self) -> f32 {
        let x = other_body.position.x - self.position.x;
        let y = other_body.position.y - self.position.y;

        (x.powi(2) + y.powi(2)).sqrt()
    }

    fn calculate_force(&self, other_body: &Self) -> Vec2 {
        let distance = self.get_distance(other_body);
        if distance < 2.0 * self.radius {
            // Adjust to avoid division by zero in force calculation
            return Vec2::ZERO; // Collision detected, no force applied
        }

        let numer = self.mass * other_body.mass;
        let denom = distance.powi(2);
        let magnitude = G * (numer / denom);

        // separate into directions
        let x_dir = magnitude * ((other_body.position.x - self.position.x) / distance);
        let y_dir = magnitude * ((other_body.position.y - self.position.y) / distance);

        vec2(x_dir, y_dir)
    }

    pub fn update_force(&mut self, other_body: &Self) {
        self.force = self.calculate_force(other_body);
    }

    pub fn update_acceleration(&mut self) {
        self.acceleration.x = self.force.x / self.mass;
        self.acceleration.y = self.force.y / self.mass;
    }

    pub fn update_velocity(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;

        // Limit the velocity to prevent the simulation from exploding
        if self.velocity.length() > MAX_VELOCITY {
            self.velocity = self.velocity.normalize() * MAX_VELOCITY;
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }

    pub fn check_and_resolve_collision(&mut self, other_body: &mut Body) {
        if
            self.get_distance(other_body) < 2.0 * self.radius ||
            self.get_distance(other_body) < 2.0 * other_body.radius
        {
            let temp_velocity = self.velocity;
            self.velocity = other_body.velocity * FRICTION;
            other_body.velocity = temp_velocity * FRICTION;
        }
    }

    pub fn check_boundary_collisions(&mut self) {
        // Check for collision with the left or right boundary
        if self.position.x <= self.radius || self.position.x >= SCREEN_WIDTH - self.radius {
            self.velocity.x = -self.velocity.x * FRICTION;
            self.position.x = self.position.x.clamp(self.radius, SCREEN_WIDTH - self.radius);
        }

        // Check for collision with the top or bottom boundary
        if self.position.y <= self.radius || self.position.y >= SCREEN_HEIGHT - self.radius {
            self.velocity.y = -self.velocity.y * FRICTION;
            self.position.y = self.position.y.clamp(self.radius, SCREEN_HEIGHT - self.radius);
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.freezed {
            self.update_acceleration();
            self.update_velocity(dt);
            self.update_position(dt);
        }
    }

    pub fn calculate_forces(bodies: &mut [Body]) -> Vec<Vec2> {
        let forces = bodies
            .par_iter()
            .enumerate()
            .map(|(i, body)| {
                bodies
                    .iter()
                    .enumerate()
                    .fold(Vec2::ZERO, |acc, (j, other_body)| {
                        if i != j { acc + body.calculate_force(other_body) } else { acc }
                    })
            })
            .collect::<Vec<_>>();

        forces
    }

    pub fn apply_forces(bodies: &mut [Body], forces: Vec<Vec2>, dt: f32) {
        bodies
            .iter_mut()
            .zip(forces.into_iter())
            .for_each(|(body, force)| {
                body.force = force;
                body.update(dt);
            });
    }

    pub fn apply_forces_with_substeps(
        bodies: &mut [Body],
        forces: Vec<Vec2>,
        dt: f32,
        substeps: usize
    ) {
        let dt_substep = dt / (substeps as f32);
        for _ in 0..substeps {
            bodies
                .iter_mut()
                .zip(forces.iter())
                .for_each(|(body, &force)| {
                    body.force = force;
                    body.update(dt_substep);
                });
        }
    }
}

#[macroquad::main("threebody")]
async fn main() {
    let bodies: Vec<Body> = (0..NUM_OF_BODIES).map(|_| Body::random(None)).collect();
    let bodies = Mutex::new(bodies);

    loop {
        clear_background(BLACK);

        let mut bodies = bodies.lock().unwrap();

        // Calculate all forces
        let forces = Body::calculate_forces(&mut bodies);

        // Apply all forces
        Body::apply_forces_with_substeps(&mut bodies, forces, 1.0, 1000);

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = mouse_position();
            bodies.push(Body::random(Some(vec2(mouse_position.0, mouse_position.1))));
        }

        // drag bodies with mouse
        if is_mouse_button_down(MouseButton::Right) {
            let mouse_position = mouse_position();
            for body in bodies.iter_mut() {
                if
                    body.get_distance(&Body::new(vec2(mouse_position.0, mouse_position.1))) <
                    2.0 * body.radius
                {
                    body.position = vec2(mouse_position.0, mouse_position.1);
                    body.velocity = Vec2::ZERO;
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Middle) {
            let mouse_position = mouse_position();
            for body in bodies.iter_mut() {
                if
                    body.get_distance(&Body::new(vec2(mouse_position.0, mouse_position.1))) <
                    2.0 * body.radius
                {
                    body.freezed = !body.freezed;
                }
            }
        }

        if is_key_pressed(KeyCode::Space) {
            bodies.clear();
        }

        for i in 0..bodies.len() {
            for j in i + 1..bodies.len() {
                let mut other_body = bodies[j];
                bodies[i].check_and_resolve_collision(&mut other_body);
                bodies[j] = other_body;
            }
            // bodies[i].update(0.5);
            bodies[i].check_boundary_collisions();
            draw_circle_lines(
                bodies[i].position.x,
                bodies[i].position.y,
                bodies[i].radius,
                4.0,
                RED
            );
        }

        draw_text(&format!("{}", get_fps()), 100.0, 100.0, 30.0, WHITE);
        next_frame().await;
    }
}
