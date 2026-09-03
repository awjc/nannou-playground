use nannou::prelude::*;

struct Model {
    rotation: f32,
    sphere_pos: Vec3,
    sphere_vel: Vec3,
}

fn main() {
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1024, 768)
        .view(view)
        .build()
        .unwrap();

    Model {
        rotation: 0.0,
        sphere_pos: vec3(0.0, 0.0, 0.0),
        sphere_vel: vec3(150.0, 100.0, 80.0),
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let dt = update.since_last.as_secs_f32();
    model.rotation += dt;
    model.sphere_pos += model.sphere_vel * dt;

    if model.sphere_pos.x.abs() > 200.0 { model.sphere_vel.x *= -1.0; }
    if model.sphere_pos.y.abs() > 200.0 { model.sphere_vel.y *= -1.0; }
    if model.sphere_pos.z.abs() > 200.0 { model.sphere_vel.z *= -1.0; }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let eye = vec3(400.0, 400.0, 400.0);
    let target = vec3(0.0, 0.0, 0.0);
    let up = vec3(0.0, 1.0, 0.0);
    let view_mat = Mat4::look_at_rh(eye, target, up);
    
    let rect = app.window_rect();
    let aspect = rect.w() / rect.h();
    let proj_mat = Mat4::perspective_rh(std::f32::consts::PI / 4.0, aspect, 0.1, 2000.0);
    let camera_transform = proj_mat * view_mat;

    // Draw a rotating cube using a mesh of lines
    let s = 100.0;
    let cube_rot = Mat4::from_rotation_y(model.rotation) * Mat4::from_rotation_x(model.rotation * 0.5);
    
    let vertices = [
        vec3(-s, -s, -s), vec3(s, -s, -s), vec3(s, s, -s), vec3(-s, s, -s),
        vec3(-s, -s, s), vec3(s, -s, s), vec3(s, s, s), vec3(-s, s, s),
    ];
    let edges = [
        (0, 1), (1, 2), (2, 3), (3, 0),
        (4, 5), (5, 6), (6, 7), (7, 4),
        (0, 4), (1, 5), (2, 6), (3, 7),
    ];

    for (i, j) in edges {
        let p1_4 = cube_rot * vec4(vertices[i].x, vertices[i].y, vertices[i].z, 1.0);
        let p2_4 = cube_rot * vec4(vertices[j].x, vertices[j].y, vertices[j].z, 1.0);
        
        // Transform points to NDC
        let p1_ndc = camera_transform * p1_4;
        let p2_ndc = camera_transform * p2_4;

        // Convert NDC to screen space manually to use draw.line()
        // NDC is [-1, 1]. We need to scale it to the window size.
        let p1_screen = vec2(p1_ndc.x / p1_ndc.w, p1_ndc.y / p1_ndc.w) * rect.w() / 2.0;
        let p2_screen = vec2(p2_ndc.x / p2_ndc.w, p2_ndc.y / p2_ndc.w) * rect.w() / 2.0;

        draw.line()
            .start(p1_screen)
            .end(p2_screen)
            .weight(2.0)
            .color(STEELBLUE);
    }

    // Draw moving sphere using a point cloud of small circles
    let sphere_radius = 40.0;
    for i in 0..12 {
        let phi = std::f32::consts::PI * (i as f32 / 12.0);
        for j in 0..12 {
            let theta = 2.0 * std::f32::consts::PI * (j as f32 / 12.0);
            let p = vec3(
                sphere_radius * phi.sin() * theta.cos(),
                sphere_radius * phi.sin() * theta.sin(),
                sphere_radius * phi.cos(),
            );
            let world_p = model.sphere_pos + p;
            
            let p_4 = camera_transform * vec4(world_p.x, world_p.y, world_p.z, 1.0);
            let p_screen = vec2(p_4.x / p_4.w, p_4.y / p_4.w) * rect.w() / 2.0;

            draw.ellipse()
                .xy(p_screen)
                .radius(3.0)
                .color(GOLD);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
