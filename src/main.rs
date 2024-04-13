use macroquad::prelude::*;

const G: f32 = 1.0;
const NUM_OF_BODIES: usize = 3;
const BODY_RADIUS: f32 = 10.0;

#[derive(Clone, Copy, PartialEq, Debug)]
struct Body {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    force: Vec2,
    mass: f32,
}

impl Body {
    fn new(initial_pos: Vec2) -> Self {
        Body {
            position: initial_pos,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            force: Vec2::ZERO,
            mass: 1000.0,
        }
    }

    fn get_distance(&self, other_body: &Self) -> f32 {
        let x = other_body.position.x - self.position.x;
        let y = other_body.position.y - self.position.y;

        (x.powi(2) + y.powi(2)).sqrt()
    }

    fn calculate_force(&self, other_body: &Self) -> Vec2 {
        let distance = self.get_distance(other_body);
        if distance < 2.0 * BODY_RADIUS {
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
    }

    pub fn update_position(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }

    pub fn check_and_resolve_collision(&mut self, other_body: &mut Body) {
        if self.get_distance(other_body) < 2.0 * BODY_RADIUS {
            // Simple elastic collision response
            let temp_velocity = self.velocity;
            self.velocity = other_body.velocity;
            other_body.velocity = temp_velocity;
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.update_acceleration();
        self.update_velocity(dt);
        self.update_position(dt);
    }
}

#[macroquad::main("civilization")]
async fn main() {
    let mut bodies: Vec<Body> = Vec::with_capacity(NUM_OF_BODIES);
    for i in 0..NUM_OF_BODIES {
        bodies.push(Body::new(vec2((i as f32) * 100.0 + 10.0, (i as f32) * 100.0 + 10.0)));
    }

    loop {
        clear_background(BLACK);

        for i in 0..bodies.len() {
            for j in 0..bodies.len() {
                if i != j {
                    let other_body = bodies[j];
                    bodies[i].update_force(&other_body);
                }
            }
        }

        for i in 0..bodies.len() {
            for j in i + 1..bodies.len() {
                let mut other_body = bodies[j];
                bodies[i].check_and_resolve_collision(&mut other_body);
                bodies[j] = other_body;
            }
            bodies[i].update(0.1);
            draw_circle(bodies[i].position.x, bodies[i].position.y, BODY_RADIUS, RED);
        }

        next_frame().await;
    }
}
