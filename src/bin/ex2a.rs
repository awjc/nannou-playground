use nannou::prelude::*;
use noise::{NoiseFn, Perlin};

// Moving dot field example

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    points: Vec<Vec3>,
    noise: Perlin,
}

fn model(app: &App) -> Model {
    app.new_window().size(600, 600).view(view).build().unwrap();
    let mut p = vec![];
    for x in -15..15 {
        for y in -15..15 {
            p.push(vec3(x as f32, y as f32, 0.0));
        }
    }
    let noise = Perlin::new(1);
    Model {
        points: p,
        noise: noise,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = (app.elapsed_frames() as f32) * 0.03;
    let mut pn = vec![];

    for p in &model.points {
        let r = model
            .noise
            .get([p.x as f64 / 10.0, p.y as f64 / 10.0, t as f64]);
        pn.push(vec3(p.x, p.y, r as f32));
    }
    model.points = pn;
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    for point in &model.points {
        let v = vec2(point.x, point.y);
        let d = if v.length() > 0.0 { v.normalize() } else { Vec2::ZERO };
        let r = point.z * 6.0 + 6.0;
        let p = vec2(point.x, point.y) * 15.0 + d * point.z * 15.0;
        draw.ellipse()
            .x_y(p.x, p.y)
            .w_h(r, r)
            .color(BLACK)
            .stroke(hsla(1.0 - point.z as f32 / 2.0 + 0.5, 1.0, 0.5, 1.0))
            .stroke_weight(1.0);
    }
    draw.to_frame(app, &frame).unwrap();

    // if frame.nth() < 2000 {
    //     let file_path = captured_frame_path(app, &frame);
    //     app.main_window().capture_frame(file_path);
    // }
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("frames")
        .join(format!("{:04}", frame.nth()))
        .with_extension("png")
}
