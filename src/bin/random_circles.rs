use nannou::prelude::*;
use rand::Rng;
use rand::SeedableRng;
use std::time::Instant;

struct WindowSize {
    w: u32,
    h: u32,
}
impl WindowSize {
    const fn half_w(&self) -> f32 { self.w as f32 / 2.0 }
    const fn half_h(&self) -> f32 { self.h as f32 / 2.0 }
}
const WINDOW: WindowSize = WindowSize { w: 1024, h: 768 };

const NUM_CIRCLES: u16 = 20;

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
    rng: rand::rngs::StdRng,
    circles: Vec<Circle>,
    last_update: Instant,
}

fn mouse_pressed(_app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => {
            model.circles = generate_circles(&mut model.rng, NUM_CIRCLES);
        }
        _ => {}
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WINDOW.w, WINDOW.h)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build();
    let mut rng = rand::rngs::StdRng::from_entropy();

    let circles = generate_circles(&mut rng, NUM_CIRCLES);

    Model {
        _window,
        rng,
        circles,
        last_update: Instant::now(),
    }
}

fn generate_circles(rng: &mut rand::rngs::StdRng, num_circles: u16) -> Vec<Circle> {
    let mut circles = vec![];
    for _n in 1..=num_circles {
        let width_range = WINDOW.half_w() * 0.4;
        let height_range = WINDOW.half_h() * 0.4;
        let pos = Vec2::new(
            rng.gen_range(-width_range..width_range), //
            rng.gen_range(-height_range..height_range), //
        );
        let vel = Vec2::new(
            rng.gen_range(-1.0..1.0) * 80., // pixels/sec
            rng.gen_range(-1.0..1.0) * 80., // pixels/sec
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
    for circle in &mut model.circles {
        if circle.pos.x - circle.radius < -WINDOW.half_w() {
            circle.vel.x = -circle.vel.x;
            circle.pos.x = -WINDOW.half_w() + circle.radius;
        }
        if circle.pos.y - circle.radius < -WINDOW.half_h() {
            circle.vel.y = -circle.vel.y;
            circle.pos.y = -WINDOW.half_h() + circle.radius;
        }
        if circle.pos.x + circle.radius > WINDOW.half_w() {
            circle.vel.x = -circle.vel.x;
            circle.pos.x = WINDOW.half_w() - circle.radius;
        }
        if circle.pos.y + circle.radius > WINDOW.half_h() {
            circle.vel.y = -circle.vel.y;
            circle.pos.y = WINDOW.half_h() - circle.radius;
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
