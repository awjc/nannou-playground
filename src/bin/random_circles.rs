use nannou::prelude::*;
use rand::Rng;
use rand::SeedableRng;
use std::time::Instant;

const INITIAL_WINDOW_SIZE: Vec2 = Vec2::new( 1024., 768. );

const NUM_CIRCLES: u16 = 50;
const SPEED_LIMIT: f32 = 500.; // pixels/sec

fn main() {
    nannou::app(model).update(update).run();
}

struct Circle {
    pos: Vec2,
    vel: Vec2,
    radius: f32
}

struct Model {
    _window: Entity,
    window_size: Vec2,
    rng: rand::rngs::StdRng,
    circles: Vec<Circle>,
    last_update: Instant,
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
    let window_size = INITIAL_WINDOW_SIZE;
    let _window = app
        .new_window()
        .size(window_size.x as u32, window_size.y as u32)
        .resized(resized)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build();
    let mut rng = rand::rngs::StdRng::from_entropy();

    let circles = generate_circles(&mut rng, &window_size, NUM_CIRCLES);

    Model {
        _window,
        window_size,
        rng,
        circles,
        last_update: Instant::now(),
    }
}

fn generate_circles(rng: &mut rand::rngs::StdRng, window_size: &Vec2, num_circles: u16) -> Vec<Circle> {
    let mut circles = vec![];
    for _n in 1..=num_circles {
        let width_range = window_size.x / 2. * 0.4;
        let height_range = window_size.y / 2. * 0.4;
        let pos = Vec2::new(
            rng.gen_range(-width_range..width_range),
            rng.gen_range(-height_range..height_range),
        );
        let speed = rng.gen_range(-1.0..1.0) * SPEED_LIMIT;  // pixels/sec
        let vel_dir = rng.gen_range(0.0..1.0) * 2. * PI;
        let vel = Vec2::new(
            speed * vel_dir.cos(),
            speed * vel_dir.sin(),
        );
        let radius = rng.gen_range(10..30) as f32;
        circles.push(Circle { pos, vel, radius });
    }
    circles
}

fn update(_app: &App, model: &mut Model) {
    let now = Instant::now();
    let delta = now.duration_since(model.last_update).as_secs_f32();
    model.last_update = now;

    for circle in &mut model.circles {
        circle.pos += circle.vel * delta;
    }

    handle_wall_bounce(model);
}

fn handle_wall_bounce(model: &mut Model) {
    let half_width = model.window_size.x / 2.;
    let half_height = model.window_size.y / 2.;
    for circle in &mut model.circles {
        if circle.pos.x - circle.radius < -half_width {
            circle.vel.x = -circle.vel.x;
            circle.pos.x = -half_width + circle.radius;
        } else if circle.pos.y - circle.radius < -half_height {
            circle.vel.y = -circle.vel.y;
            circle.pos.y = -half_height + circle.radius;
        } else if circle.pos.x + circle.radius > half_width {
            circle.vel.x = -circle.vel.x;
            circle.pos.x = half_width - circle.radius;
        } else if circle.pos.y + circle.radius > half_height {
            circle.vel.y = -circle.vel.y;
            circle.pos.y = half_height - circle.radius;
        }
    }
}

fn view(app: &App, model: &Model) {
    let draw = app.draw();
    let bg_color = Srgba::new(0.2, 0.1, 0.4, 1.0);
    draw.background().color(bg_color);

    let circle_col = Srgba::new(0.2, 0.9, 0.4, 0.8);
    for circle in &model.circles {
        draw_circle(&draw, &circle.pos, circle.radius, circle_col);
    }
}

fn draw_circle(draw: &Draw, loc: &Vec2, radius: f32, col: Srgba) {
    draw.ellipse()
        .x_y(loc.x, loc.y)
        .w_h(radius * 2., radius * 2.)
        .color(col)
        .stroke_weight(1.0);
}
