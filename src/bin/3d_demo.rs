use nannou::prelude::*;

/// `MiniEngine` encapsulates the 3D mathematics required to project 
/// 3D world coordinates onto Nannou's 2D primitive drawing API.
struct MiniEngine {
    /// The combined View-Projection matrix (Projection * View)
    camera_transform: Mat4,
    /// Current window dimensions for scaling NDC to screen space
    window_size: Vec2,
}

impl MiniEngine {
    /// Creates a new engine instance by defining a camera's properties.
    fn new(app: &App, eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let rect = app.window_rect();
        let aspect = rect.w() / rect.h();
        
        // 1. Create the View Matrix (where the camera is and where it looks)
        let view_mat = Mat4::look_at_rh(eye, target, up);
        
        // 2. Create the Projection Matrix (simulates perspective/depth)
        let proj_mat = Mat4::perspective_rh(std::f32::consts::PI / 4.0, aspect, 0.1, 2000.0);
        
        Self {
            camera_transform: proj_mat * view_mat,
            window_size: vec2(rect.w(), rect.h()),
        }
    }

    /// `draw_mesh` is the core rendering function. It takes 3D geometry, 
    /// applies the object's local transform, then the camera's transform,
    /// and finally projects the result into 2D screen space.
    fn draw_mesh(&self, draw: &Draw, vertices: &[Vec3], indices: &[usize], transform: Mat4, color: Srgb<u8>) {
        // Step 1: Project all 3D vertices into 2D screen coordinates
        let projected_vertices: Vec<Vec2> = vertices.iter()
            .map(|&v| {
                // Apply Object Transform (Scale/Rotate/Translate)
                let transformed = transform * vec4(v.x, v.y, v.z, 1.0);
                
                // Apply Camera Transform (View/Projection)
                let p_cam = self.camera_transform * transformed;
                
                // Convert from Normalized Device Coordinates (NDC) [-1, 1] 
                // to actual Window Screen Space
                vec2(p_cam.x / p_cam.w, p_cam.y / p_cam.w) * self.window_size / 2.0
            })
            .collect();

        // Step 2: Render the triangles using the projected 2D points
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

/// `MeshObject` represents a 3D entity in the world.
struct MeshObject {
    /// The raw local-space geometry
    vertices: Vec<Vec3>,
    /// How vertices are connected to form triangles
    indices: Vec<usize>,
    /// World position
    position: Vec3,
    /// Rotation angles (Euler angles: X, Y, Z)
    rotation: Vec3,
    /// Surface color
    color: Srgb<u8>,
}

impl MeshObject {
    /// Constructor for a standard cube primitive.
    fn new_cube(size: f32, color: Srgb<u8>) -> Self {
        let s = size / 2.0;
        // Define the 8 corners of the cube
        let vertices = vec![
            vec3(-s, -s, -s), vec3(s, -s, -s), vec3(s, s, -s), vec3(-s, s, -s),
            vec3(-s, -s, s), vec3(s, -s, s), vec3(s, s, s), vec3(-s, s, s),
        ];
        // Define triangles (2 per face = 12 triangles total)
        let indices = vec![
            0, 1, 2, 0, 2, 3, // Front face
            4, 5, 6, 4, 6, 7, // Back face
            0, 4, 7, 0, 7, 3, // Left face
            1, 5, 6, 1, 6, 2, // Right face
            3, 2, 6, 3, 6, 7, // Top face
            0, 1, 5, 0, 5, 4, // Bottom face
        ];

        Self {
            vertices,
            indices,
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            color,
        }
    }

    /// Calculates the 4x4 transformation matrix for this object.
    fn get_transform(&self) -> Mat4 {
        Mat4::from_translation(self.position) *
        Mat4::from_rotation_x(self.rotation.x) *
        Mat4::from_rotation_y(self.rotation.y) *
        Mat4::from_rotation_z(self.rotation.z)
    }

    /// Renders the object using the provided engine.
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
    rotation_timer: f32,
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

    // Initialize Engine with a camera looking at the origin
    let engine = MiniEngine::new(
        app, 
        vec3(400.0, 400.0, 400.0), // Camera Position (Eye)
        vec3(0.0, 0.0, 0.0),       // Target point
        vec3(0.0, 1.0, 0.0)        // Up vector
    );

    let cube = MeshObject::new_cube(150.0, STEELBLUE);

    Model {
        engine,
        cube,
        rotation_timer: 0.0,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let dt = update.since_last.as_secs_f32();
    model.rotation_timer += dt;
    
    // Animate the cube's rotation
    model.cube.rotation.y = model.rotation_timer;
    model.cube.rotation.x = model.rotation_timer * 0.5;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    // Render all objects in the world
    model.cube.draw(&draw, &model.engine);

    draw.to_frame(app, &frame).unwrap();
}
