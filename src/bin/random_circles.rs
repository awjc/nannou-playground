use nannou::prelude::*;
use rand::Rng;
use rand::SeedableRng;
use std::time::Instant;

const INITIAL_WINDOW_W: u32 = 1024;
const INITIAL_WINDOW_H: u32 = 768;

const NUM_CIRCLES: u32 = 50;
const SPEED_LIMIT: f32 = 500.0; // pixels/sec
const GRAVITY: f32 = 2000.0;
const BOUNCE_DECAY_MIN: f32 = 0.75;
const BOUNCE_DECAY_MAX: f32 = 0.96;
const MIN_RADIUS: f32 = 10.0;
const MAX_RADIUS: f32 = 30.0;

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::RefreshSync)
        .update(update)
        .run();
}

struct Circle {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
    decay: f32,
}

struct Model {
    window_size: Vec2,
    rng: rand::rngs::StdRng,
    circles: Vec<Circle>,
    frames_this_second: u32,
    fps: u32,
    last_fps_reset: Instant,
}

fn mouse_pressed(_app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Middle => {
            model.circles = generate_circles(&mut model.rng, &model.window_size, NUM_CIRCLES);
        }
        _ => {}
    }
}

fn resized(_app: &App, model: &mut Model, new_size: Vec2) {
    model.window_size = new_size
}

fn model(app: &App) -> Model {
    let window_size = Vec2::new(INITIAL_WINDOW_W as f32, INITIAL_WINDOW_H as f32);
    let _window = app
        .new_window()
        .size(INITIAL_WINDOW_W, INITIAL_WINDOW_H)
        .resized(resized)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build();
    let mut rng = rand::rngs::StdRng::from_entropy();

    let circles = generate_circles(&mut rng, &window_size, NUM_CIRCLES);

    Model {
        window_size,
        rng,
        circles,
        frames_this_second: 0,
        fps: 0,
        last_fps_reset: Instant::now()
    }
}

fn generate_circles(rng: &mut rand::rngs::StdRng, window_size: &Vec2, num_circles: u32) -> Vec<Circle> {
    let mut circles = vec![];
    let width_range = window_size.x / 2.0 * 0.4;
    let height_range = window_size.y / 2.0 * 0.4;
    for _n in 1..=num_circles {
        let pos = Vec2::new(
            rng.gen_range(-width_range..width_range),
            rng.gen_range(-height_range..height_range),
        );
        let speed = rng.gen_range(-0.0..1.0) * SPEED_LIMIT;  // pixels/sec
        let vel_dir = rng.gen_range(0.0..1.0) * 2.0 * PI;
        let vel = Vec2::new(
            speed * vel_dir.cos(),
            speed * vel_dir.sin(),
        );
        let radius = rng.gen_range(MIN_RADIUS..MAX_RADIUS);
        // fraction of velocity kept after a bounce; bigger balls lose more per bounce. precomputed now to not have to do it every frame
        let lerp = 1.0 - ((radius - MIN_RADIUS) / (MAX_RADIUS - MIN_RADIUS)).powf(2.0);
        let decay = BOUNCE_DECAY_MIN + lerp * (BOUNCE_DECAY_MAX - BOUNCE_DECAY_MIN);
        circles.push(Circle { pos, vel, radius, decay });
    }
    circles
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let now = Instant::now();
    let delta = update.since_last.as_secs_f32();

    for circle in &mut model.circles {
        circle.pos += circle.vel * delta;
    }

    handle_wall_bounce(model);

    for circle in &mut model.circles {
        circle.vel.y -= GRAVITY * delta;
    }

    model.frames_this_second += 1;
    if now.duration_since(model.last_fps_reset).as_secs_f32() >= 1.0 {
        model.fps = model.frames_this_second;
        model.frames_this_second = 0;
        model.last_fps_reset = now;
    }
}

fn handle_wall_bounce(model: &mut Model) {
    let half_width = model.window_size.x / 2.0;
    let half_height = model.window_size.y / 2.0;
    for circle in &mut model.circles {
        // horizontal bounces do not decay
        if circle.pos.x - circle.radius < -half_width {
            circle.vel.x = -circle.vel.x;
            circle.pos.x = -half_width + circle.radius;
        } else if circle.pos.x + circle.radius > half_width {
            circle.vel.x = -circle.vel.x;
            circle.pos.x = half_width - circle.radius;
        }
        if circle.pos.y - circle.radius < -half_height {
            circle.vel.y = -circle.vel.y * circle.decay;
            circle.pos.y = -half_height + circle.radius;
        } else if circle.pos.y + circle.radius > half_height {
            circle.vel.y = -circle.vel.y * circle.decay;
            circle.pos.y = half_height - circle.radius;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let bg_color = Srgba::new(0.2, 0.1, 0.4, 1.0);
    draw.background().color(bg_color);

    let circle_col = Srgba::new(0.2, 0.9, 0.4, 0.8);
    for circle in &model.circles {
        draw_circle(&draw, &circle.pos, circle.radius, circle_col);
    }

    // FPS counter
    let fps = model.fps;
    draw.text(&format!("FPS: {fps}"))
        .x_y(-model.window_size.x / 2.0 + 40.0, model.window_size.y / 2.0 - 10.0)
        .font_size(16)
        .color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_circle(draw: &Draw, loc: &Vec2, radius: f32, col: Srgba) {
    draw.ellipse()
        .x_y(loc.x, loc.y)
        .w_h(radius * 2.0, radius * 2.0)
        .color(col)
        .stroke_weight(1.0);
}
