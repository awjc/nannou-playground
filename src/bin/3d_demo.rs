use nannou::prelude::*;

/// `MiniEngine` encapsulates the 3D mathematics required to project 
/// 3D objects into Nannou's 2D primitive drawing API.
struct MiniEngine {
    /// The combined View-Projection matrix (Projection * View)
    camera_transform: Mat4,
    /// Current window dimensions for scaling NDC to screen space
    window_size: Vec2,
    /// A global light direction for simple Lambertian shading
    light_dir: Vec3,
}

impl MiniEngine {
    /// Creates a new engine instance by defining a camera's properties.
    fn new(app: &App, eye: Vec3, target: Vec3, up: Vec3, light_dir: Vec3) -> Self {
        let rect = app.window_rect();
        let aspect = rect.w() / rect.h();
        let view_mat = Mat4::look_at_rh(eye, target, up);
        let proj_mat = Mat4::perspective_rh(std::f32::consts::PI / 4.0, aspect, 0.1, 2000.0);
        
        Self {
            camera_transform: proj_mat * view_mat,
            window_size: vec2(rect.w(), rect.h()),
            light_dir: light_dir.normalize(),
        }
    }

    /// `draw_mesh` is the core rendering function. It takes 3D geometry, 
    /// applies the object's local transform, then the camera's transform,
    /// and finally projects the result into 2D screen space.
    /// It also applies simple per-face Lambertian lighting.
    fn draw_mesh(&self, draw: &Draw, vertices: &[Vec3], indices: &[usize], transform: Mat4, base_color: LinSrgba) {
        // Step 1: Project all 3D vertices into 2D screen coordinates
        let projected_vertices: Vec<Vec2> = vertices.iter()
            .map(|&v| {
                let transformed = transform * vec4(v.x, v.y, v.z, 1.0);
                let p_cam = self.camera_transform * transformed;
                vec2(p_cam.x / p_cam.w, p_cam.y / p_cam.w) * self.window_size / 2.0
            })
            .collect();

        // We also need the transformed 3D points for normal calculation
        let transformed_vertices: Vec<Vec3> = vertices.iter()
            .map(|&v| {
                let t = transform * vec4(v.x, v.y, v.z, 1.0);
                vec3(t.x, t.y, t.z)
            })
            .collect();

        // Step 2: Render the triangles using the projected 2D points
        for chunk in indices.chunks(3) {
            if chunk.len() == 3 {
                let i1 = chunk[0];
                let i2 = chunk[1];
                let i3 = chunk[2];

                // Calculate face normal for lighting
                let v1 = transformed_vertices[i1];
                let v2 = transformed_vertices[i2];
                let v3 = transformed_vertices[i3];
                let normal = (v2 - v1).cross(v3 - v1).normalize();

                // Lambertian diffuse lighting: dot(normal, light_dir)
                let intensity = normal.dot(self.light_dir).max(0.0);
                let ambient = 0.2;
                let diffuse = 0.8;
                let lit_color = base_color * (ambient + intensity * diffuse);

                let p1 = projected_vertices[i1];
                let p2 = projected_vertices[i2];
                let p3 = projected_vertices[i3];
                
                draw.polygon()
                    .points([p1, p2, p3])
                    .color(lit_color);
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
    color: LinSrgba,
}

impl MeshObject {
    fn new(vertices: Vec<Vec3>, indices: Vec<usize>, color: LinSrgba) -> Self {
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
    fn new(position: Vec3, size: f32, color: LinSrgba) -> Self {
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
    app.new_window()
        .size(1024, 768)
        .view(view)
        .build()
        .unwrap();

    let engine = MiniEngine::new(
        app, 
        vec3(400.0, 400.0, 400.0), 
        vec3(0.0, 0.0, 0.0),       
        vec3(0.0, 1.0, 0.0),
        vec3(0.5, 1.0, 0.3) // Light direction
    );

    let mut objs_in_scene: Vec<Box<dyn SceneObject>> = Vec::new();
    
    objs_in_scene.push(Box::new(Cube::new(vec3(0.0, 0.0, 0.0), 150.0, LinSrgba::new(0.27, 0.51, 0.71, 1.0))));
    objs_in_scene.push(Box::new(Cube::new(vec3(200.0, 100.0, -50.0), 50.0, LinSrgba::new(1.0, 0.65, 0.0, 1.0))));
    objs_in_scene.push(Box::new(Cube::new(vec3(-250.0, -150.0, 100.0), 80.0, LinSrgba::new(0.0, 0.5, 0.0, 1.0))));

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
