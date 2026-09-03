// Gemma 4's experiment

use nannou::prelude::*;

const NUM_PARTICLES: u32 = 1500;
const MAX_SPEED: f32 = 2.5;
const PARTICLE_SIZE: f32 = 3.0;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    color: Srgba,
}

struct Model {
    particles: Vec<Particle>,
    window_size: Vec2,
    time: f32,
}

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::RefreshSync)
        .update(update)
        .view(view)
        .run();
}

fn model(app: &App) -> Model {
    let window_size = Vec2::new(1024.0, 768.0);
    let _window = app.new_window().size(1024, 768).view(view).build();

    let mut particles = vec![];
    for _ in 0..NUM_PARTICLES {
        particles.push(Particle {
            pos: Vec2::new(
                (rand::random::<f32>() - 0.5) * window_size.x,
                (rand::random::<f32>() - 0.5) * window_size.y,
            ),
            vel: Vec2::ZERO,
            color: Srgba::new(0.3, 0.7, 1.0, 0.5),
        });
    }

    Model {
        particles,
        window_size,
        time: 0.0,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let dt = update.since_last.as_secs_f32();
    model.time += dt;

    for particle in &mut model.particles {
        // Create a smooth, swirling flow field using trigonometry
        let angle = (particle.pos.y * 0.002 + model.time * 0.3).cos() * PI
            + (particle.pos.x * 0.002 + model.time * 0.3).sin() * PI;

        let target_vel = Vec2::new(angle.cos(), angle.sin()) * MAX_SPEED;

        // Smoothly steer the particle towards the target velocity
        particle.vel = particle.vel.lerp(target_vel, dt * 1.5);
        particle.pos += particle.vel;

        // Wrap around the window edges
        let half_w = model.window_size.x / 2.0;
        let half_h = model.window_size.y / 2.0;
        if particle.pos.x > half_w {
            particle.pos.x = -half_w;
        } else if particle.pos.x < -half_w {
            particle.pos.x = half_w;
        }

        if particle.pos.y > half_h {
            particle.pos.y = -half_h;
        } else if particle.pos.y < -half_h {
            particle.pos.y = half_h;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(Srgba::new(0.02, 0.02, 0.05, 1.0));

    for particle in &model.particles {
        draw.ellipse()
            .x_y(particle.pos.x, particle.pos.y)
            .w_h(PARTICLE_SIZE, PARTICLE_SIZE)
            .color(particle.color);
    }

    draw.to_frame(app, &frame).unwrap();
}
