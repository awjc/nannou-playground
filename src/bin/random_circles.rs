use nannou::prelude::*;
use rand::Rng;
use rand::SeedableRng;
use std::time::Instant;

fn main() {
    nannou::app(model).update(update).run();
}

struct Circle {
    pos: Vec2,
    vel: Vec2,
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
            model.circles = generate_circles(&mut model.rng, 10);
        }
        _ => {}
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build();
    let mut rng = rand::rngs::StdRng::from_entropy();

    let circles = generate_circles(&mut rng, 10);

    Model {
        _window,
        rng,
        circles,
        last_update: Instant::now(),
    }
}

fn generate_circles(rng: &mut rand::rngs::StdRng, num_circles: i16) -> Vec<Circle> {
    let mut circles = vec![];
    for _n in 1..=num_circles {
        let pos = Vec2::new(
            rng.gen_range(-200.0..200.0), //
            rng.gen_range(-200.0..200.0), //
        );
        let vel = Vec2::new(
            rng.gen_range(-1.0..1.0) * 30., // pixels/sec
            rng.gen_range(-1.0..1.0) * 30., // pixels/sec
        );
        circles.push(Circle { pos, vel });
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
}

fn view(app: &App, model: &Model) {
    let draw = app.draw();
    let bg_color = Srgba::new(0.2, 0.1, 0.4, 1.0);
    let obj_color = Srgba::new(0.2, 0.7, 0.9, 0.5);
    draw.background().color(bg_color);
    draw.ellipse().color(obj_color);

    let col = Srgba::new(0.2, 0.9, 0.4, 0.8);
    let rad = 25.;

    for circle in &model.circles {
        draw_circle(&draw, &circle.pos, rad, col);
    }
}

fn draw_circle(draw: &Draw, loc: &Vec2, rad: f32, col: Srgba) {
    draw.ellipse()
        .x_y(loc.x, loc.y)
        .w_h(rad * 2., rad * 2.)
        .color(col)
        .stroke_weight(1.0);
}
