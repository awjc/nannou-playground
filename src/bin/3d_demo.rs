use nannou::prelude::*;

/// `MiniEngine` encapsulates the 3D mathematics required to project 
/// 3D objects into Nannou's 2D primitive drawing API.
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

    fn draw_mesh(&self, draw: &Draw, vertices: &[Vec3], indices: &[usize], transform: Mat4, color: Srgb<u8>) {
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
                draw.polygon().points([p1, p2, p3]).color(color);
            }
        }
    }
}

/// A trait representing any object that can be updated and rendered in our 3D engine.
trait SceneObject {
    fn update(&mut self, dt: f32);
    fn draw(&self, draw: &Draw, engine: &MiniEngine);
}

/// `MeshObject` is the base implementation for 3D entities composed of a mesh.
struct MeshObject {
    vertices: Vec<Vec3>,
    indices: Vec<usize>,
    position: Vec3,
    rotation: Vec3,
    color: Srgb<u8>,
}

impl MeshObject {
    fn new(vertices: Vec<Vec3>, indices: Vec<usize>, color: Srgb<u8>) -> Self {
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
}

/// A specific implementation of a Cube.
struct Cube {
    inner: MeshObject,
}

impl Cube {
    fn new(position: Vec3, size: f32, color: Srgb<u8>) -> Self {
        let s = size / 2.0;
        let vertices = vec![
            vec3(-s, -s, -s), vec3(s, -s, -s), vec3(s, s, -s), vec3(-s, s, -s),
            vec3(-s, -s, s), vec3(s, -s, s), vec3(s, s, s), vec3(-s, s, s),
        ];
        let indices = vec![
            0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 0, 4, 7, 0, 7, 3,
            1, 5, 6, 1, 6, 2, 3, 2, 6, 3, 6, 7, 0, 1, 5, 0, 5, 4,
        ];

        let mut inner = MeshObject::new(vertices, indices, color);
        inner.position = position;
        Self { inner }
    }
}

impl SceneObject for Cube {
    fn update(&mut self, dt: f32) {
        self.inner.rotation.y += dt;
        self.inner.rotation.x += dt * 0.5;
    }

    fn draw(&self, draw: &Draw, engine: &MiniEngine) {
        engine.draw_mesh(
            draw, 
            &self.inner.vertices, 
            &self.inner.indices, 
            self.inner.get_transform(), 
            self.inner.color
        );
    }
}

struct Model {
    engine: MiniEngine,
    objs_in_scene: Vec<Box<dyn SceneObject>>,
}

fn main() {
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(1024, 768).view(view).build().unwrap();

    let engine = MiniEngine::new(
        app, 
        vec3(400.0, 400.0, 400.0), 
        vec3(0.0, 0.0, 0.0),       
        vec3(0.0, 1.0, 0.0)        
    );

    let mut objs_in_scene: Vec<Box<dyn SceneObject>> = Vec::new();
    
    objs_in_scene.push(Box::new(Cube::new(vec3(0.0, 0.0, 0.0), 150.0, STEELBLUE)));
    objs_in_scene.push(Box::new(Cube::new(vec3(200.0, 100.0, -50.0), 50.0, ORANGE)));
    objs_in_scene.push(Box::new(Cube::new(vec3(-250.0, -150.0, 100.0), 80.0, GREEN)));

    Model {
        engine,
        objs_in_scene,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let dt = update.since_last.as_secs_f32();
    for obj in &mut model.objs_in_scene {
        obj.update(dt);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for obj in &model.objs_in_scene {
        obj.draw(&draw, &model.engine);
    }

    draw.to_frame(app, &frame).unwrap();
}
