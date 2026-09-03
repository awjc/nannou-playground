use nannou::prelude::*;

/// A minimal 3D engine abstraction for projecting 3D objects into Nannou's 2D space.
struct MiniEngine {
    camera_transform: Mat4,
    window_size: Vec2,
}

impl MiniEngine {
    fn new(app: &App, eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let rect = app.window_rect();
        let aspect = rect.w() / rect.h();
        let view_mat = Mat4::look_at_rh(eye, target, up);
        let proj_mat = Mat4::perspective_rh(std::f32::consts::PI / 4.0, aspect, 0.1, 2000.0);
        
        Self {
            camera_transform: proj_mat * view_mat,
            window_size: vec2(rect.w(), rect.h()),
        }
    }

    /// Renders a collection of triangles as 2D polygons.
    fn draw_mesh(&self, draw: &Draw, vertices: &[Vec3], indices: &[usize], transform: Mat4, color: Srgb<u8>) {
        // Pre-transform and project vertices
        let projected_vertices: Vec<Vec2> = vertices.iter()
            .map(|&v| {
                let transformed = transform * vec4(v.x, v.y, v.z, 1.0);
                let p_cam = self.camera_transform * transformed;
                vec2(p_cam.x / p_cam.w, p_cam.y / p_cam.w) * self.window_size / 2.0
            })
            .collect();

        for chunk in indices.chunks(3) {
            if chunk.len() == 3 {
                let p1 = projected_vertices[chunk[0]];
                let p2 = projected_vertices[chunk[1]];
                let p3 = projected_vertices[chunk[2]];
                
                draw.polygon()
                    .points([p1, p2, p3])
                    .color(color);
            }
        }
    }
}

/// A simple 3D object representation.
struct MeshObject {
    vertices: Vec<Vec3>,
    indices: Vec<usize>,
    position: Vec3,
    rotation: Vec3, // Euler angles
    color: Srgb<u8>,
}

impl MeshObject {
    fn new_cube(size: f32, color: Srgb<u8>) -> Self {
        let s = size / 2.0;
        let vertices = vec![
            vec3(-s, -s, -s), vec3(s, -s, -s), vec3(s, s, -s), vec3(-s, s, -s),
            vec3(-s, -s, s), vec3(s, -s, s), vec3(s, s, s), vec3(-s, s, s),
        ];
        let indices = vec![
            0, 1, 2, 0, 2, 3, // Front
            4, 5, 6, 4, 6, 7, // Back
            0, 4, 7, 0, 7, 3, // Left
            1, 5, 6, 1, 6, 2, // Right
            3, 2, 6, 3, 6, 7, // Top
            0, 1, 5, 0, 5, 4, // Bottom
        ];

        Self {
            vertices,
            indices,
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            color,
        }
    }

    fn get_transform(&self) -> Mat4 {
        Mat4::from_translation(self.position) *
        Mat4::from_rotation_x(self.rotation.x) *
        Mat4::from_rotation_y(self.rotation.y) *
        Mat4::from_rotation_z(self.rotation.z)
    }

    fn draw(&self, draw: &Draw, engine: &MiniEngine) {
        engine.draw_mesh(
            draw, 
            &self.vertices, 
            &self.indices, 
            self.get_transform(), 
            self.color
        );
    }
}

struct Model {
    engine: MiniEngine,
    cube: MeshObject,
    rotation: f32,
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

    let engine = MiniEngine::new(
        app, 
        vec3(400.0, 400.0, 400.0), // Eye
        vec3(0.0, 0.0, 0.0),       // Target
        vec3(0.0, 1.0, 0.0)        // Up
    );

    let cube = MeshObject::new_cube(150.0, STEELBLUE);

    Model {
        engine,
        cube,
        rotation: 0.0,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let dt = update.since_last.as_secs_f32();
    model.rotation += dt;
    model.cube.rotation.y = model.rotation;
    model.cube.rotation.x = model.rotation * 0.5;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.cube.draw(&draw, &model.engine);

    draw.to_frame(app, &frame).unwrap();
}
