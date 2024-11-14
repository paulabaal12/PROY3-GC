use nalgebra_glm::{Vec3, Vec4, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;
use rand::Rng;

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use triangle::triangle;
use shaders::{vertex_shader, fragment_shader};
use fastnoise_lite::{FastNoiseLite, NoiseType};

#[derive(Clone, Copy)]
pub enum CelestialBody {
    Sun,
    RockyPlanet,
    GasGiant,
    CloudyPlanet,
    RingedPlanet,
    IcePlanet,
    ColorPlanet,
    Moon,
    OceanPlanet,    
    NaturePlanet,   
    AuroraPlanet, 
}

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite,
    current_body: CelestialBody,  
}

fn create_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = fragment_shader(&fragment, &uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn handle_input(window: &Window, camera: &mut Camera) {
    let movement_speed = 1.0;
    let rotation_speed = PI/50.0;
    let zoom_speed = 0.1;
   
    if window.is_key_down(Key::Left) {
        camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
        camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W) {
        camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S) {
        camera.orbit(0.0, rotation_speed);
    }

    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A) {
        movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D) {
        movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q) {
        movement.y += movement_speed;
    }
    if window.is_key_down(Key::E) {
        movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
        camera.move_center(movement);
    }

    if window.is_key_down(Key::Up) {
        camera.zoom(zoom_speed);
    }
    if window.is_key_down(Key::Down) {
        camera.zoom(-zoom_speed);
    }
}

struct Moon {
    position: Vec3,
    rotation: Vec3,
    scale: f32,
    orbit_radius: f32,
    orbit_speed: f32,
    orbit_angle: f32,
    parent_position: Vec3,
}

impl Moon {
    fn new(orbit_radius: f32, orbit_speed: f32) -> Self {
        Moon {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: 0.8,
            orbit_radius,
            orbit_speed,
            orbit_angle: 0.0,
            parent_position: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    fn update(&mut self, parent_pos: Vec3) {
        self.rotation.y += 0.01;
        self.orbit_angle += self.orbit_speed;
        self.parent_position = parent_pos;
        
        let relative_x = self.orbit_angle.cos() * self.orbit_radius;
        let relative_z = self.orbit_angle.sin() * self.orbit_radius;
        
        self.position = Vec3::new(
            parent_pos.x + relative_x,
            parent_pos.y,
            parent_pos.z + relative_z
        );
    }
}

struct Planet {
    position: Vec3,
    rotation: Vec3,
    scale: f32,
    body_type: CelestialBody,
    orbit_radius: f32,
    orbit_speed: f32,
    orbit_angle: f32,
    original_scale: f32,
}

impl Planet {
    fn new(orbit_radius: f32, body_type: CelestialBody, orbit_speed: f32) -> Self {
        let scale = match body_type {
            CelestialBody::Sun => 4.0,         // Aumenta el tamaño del Sol
            CelestialBody::GasGiant => 3.0,    // Aumenta para planetas gaseosos
            CelestialBody::RingedPlanet => 2.5, // Aumenta para planetas con anillos
            CelestialBody::IcePlanet => 2.0,   // Aumenta para planetas de hielo
            CelestialBody::RockyPlanet => 1.5, // Ajuste para planetas rocosos
            CelestialBody::OceanPlanet => 1.7, // Ajuste para planetas oceánicos
            CelestialBody::CloudyPlanet => 2.8, // Aumenta el tamaño para planetas con atmósferas densas (e.g., Tierra)
            _ => 1.2,                          // Tamaño base para otros cuerpos celestes
        };
        

        Planet {
            position: Vec3::new(orbit_radius, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale,
            original_scale: scale,
            body_type,
            orbit_radius,
            orbit_speed,
            orbit_angle: 0.0,
        }
    }

    fn update(&mut self) {
        self.rotation.y += 0.01;
        self.orbit_angle += self.orbit_speed;
        self.position.x = self.orbit_angle.cos() * self.orbit_radius;
        self.position.z = self.orbit_angle.sin() * self.orbit_radius;
    }
}

fn draw_orbit(framebuffer: &mut Framebuffer, radius: f32, uniforms: &Uniforms, depth: f32) {
    let segments = 100;
    let mut last_point = None;
    
    for i in 0..=segments {
        let angle = (i as f32 / segments as f32) * 2.0 * PI;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        let point = Vec3::new(x, 0.0, z);
        
        let world_pos = uniforms.view_matrix * Vec4::new(point.x, point.y, point.z, 1.0);
        let mut transformed = uniforms.projection_matrix * world_pos;
        transformed /= transformed.w;
        
        let screen_x = ((transformed.x + 1.0) * framebuffer.width as f32 / 2.0) as usize;
        let screen_y = ((1.0 - transformed.y) * framebuffer.height as f32 / 2.0) as usize;
        
        if let Some((last_x, last_y)) = last_point {
            draw_line(framebuffer, last_x, last_y, screen_x, screen_y, 0.99999);
        }
        
        last_point = Some((screen_x, screen_y));
    }
}

fn draw_line(framebuffer: &mut Framebuffer, x0: usize, y0: usize, x1: usize, y1: usize, depth: f32) {
    let mut x0 = x0 as isize;
    let mut y0 = y0 as isize;
    let x1 = x1 as isize;
    let y1 = y1 as isize;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let mut err = dx + dy;
    let mut e2;

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    loop {
        framebuffer.point(x0 as usize, y0 as usize, depth);

        if x0 == x1 && y0 == y1 { break; }
        e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn main() {
    let window_width = 1600;
    let window_height = 900;
    let framebuffer_width = 1600;
    let framebuffer_height = 900;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Sistema Solar",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(200, 100);
    framebuffer.set_background_color(0x000015);

    let mut camera = Camera::new(
        Vec3::new(0.0, 15.0, 30.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );

    let obj = Obj::load("assets/sphere.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array();
    
    let mut planets = vec![
        Planet::new(0.0, CelestialBody::Sun, 0.0),        
        Planet::new(5.0, CelestialBody::RockyPlanet, 0.03), 
        Planet::new(7.0, CelestialBody::ColorPlanet, 0.025), 
        Planet::new(9.0, CelestialBody::CloudyPlanet, 0.02), 
        Planet::new(11.0, CelestialBody::RockyPlanet, 0.018), 
        Planet::new(14.0, CelestialBody::GasGiant, 0.012),    
        Planet::new(18.0, CelestialBody::RingedPlanet, 0.009),
        Planet::new(21.0, CelestialBody::IcePlanet, 0.007),    
        Planet::new(24.0, CelestialBody::NaturePlanet, 0.005),    
        Planet::new(26.0, CelestialBody::AuroraPlanet, 0.015),    
        Planet::new(30.0, CelestialBody::OceanPlanet, 0.010),  
    ];
    let mut moon = Moon::new(1.5, 0.05);
    let skybox = Skybox::new(4000, 100.0); // 1000 estrellas, radio 100
    let mut time = 0u32;
    let mut selected_planet: Option<usize> = None;
    let zoom_scale = 3.0; 
    let moon_zoom_scale = 2.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, key) in [Key::Key1, Key::Key2, Key::Key3, Key::Key4, Key::Key5, 
                         Key::Key6, Key::Key7, Key::Key8, Key::Key9]
                         .iter()
                         .enumerate() {
            if window.is_key_pressed(*key, minifb::KeyRepeat::No) {
                if Some(i) == selected_planet {
                    selected_planet = None;
                    planets[i].scale = planets[i].original_scale;
                    moon.scale = 1.2;            
                    moon.orbit_radius = 1.5;      
                } else {
                  
                    if let Some(prev) = selected_planet {
                        planets[prev].scale = planets[prev].original_scale;
                    }
                  
                    selected_planet = Some(i);
                    planets[i].scale = planets[i].original_scale * zoom_scale;
    
               
                    if matches!(planets[i].body_type, CelestialBody::CloudyPlanet) {
                        moon.scale = 1.2 * moon_zoom_scale;       
                        moon.orbit_radius = 1.5 * moon_zoom_scale; 
                    } else {
                        moon.scale = 1.2;             
                        moon.orbit_radius = 1.5;      
                    }
                }
            }
        }
    
        time += 1;
        handle_input(&window, &mut camera);
        framebuffer.clear();
        
    
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
            skybox.render(&mut framebuffer, &Uniforms {
                model_matrix: Mat4::identity(),
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
                noise: create_noise(),
                current_body: CelestialBody::Sun, 
            });

        for planet in &planets {
            if planet.orbit_radius > 0.0 {
                let uniforms = Uniforms {
                    model_matrix: Mat4::identity(),
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time,
                    noise: create_noise(),
                    current_body: planet.body_type,
                };
                
                framebuffer.set_current_color(0x404040);
                draw_orbit(&mut framebuffer, planet.orbit_radius, &uniforms, 0.99999);
            }
        }

        let mut earth_position = Vec3::new(0.0, 0.0, 0.0);
        for planet in planets.iter_mut() {
            planet.update();
            
            if matches!(planet.body_type, CelestialBody::CloudyPlanet) {
                earth_position = planet.position;
            }
            
            let model_matrix = create_model_matrix(
                planet.position,
                planet.scale,
                planet.rotation
            );
            
            let uniforms = Uniforms {
                model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
                noise: create_noise(),
                current_body: planet.body_type,
            };
    
            render(&mut framebuffer, &uniforms, &vertex_arrays);
        }

        moon.update(earth_position);
        
        let moon_model_matrix = create_model_matrix(
            moon.position,
            moon.scale,
            moon.rotation
        );
        
        let moon_uniforms = Uniforms {
            model_matrix: moon_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: create_noise(),
            current_body: CelestialBody::Moon,
        };

        framebuffer.set_current_color(0x303030);
        draw_orbit(&mut framebuffer, moon.orbit_radius, &moon_uniforms, 0.99999);
        render(&mut framebuffer, &moon_uniforms, &vertex_arrays);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
    
        std::thread::sleep(frame_delay);
    }
}    


pub struct Star {
    position: Vec3,
    brightness: f32,
    size: f32,
}

pub struct Skybox {
    stars: Vec<Star>,
    radius: f32,
}

impl Skybox {
    pub fn new(num_stars: usize, radius: f32) -> Self {
        let mut rng = rand::thread_rng();
        let mut stars = Vec::with_capacity(num_stars);

        for _ in 0..num_stars {

            let theta = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
            let phi = rng.gen_range(0.0..std::f32::consts::PI);
            
   
            let x = radius * phi.sin() * theta.cos();
            let y = radius * phi.sin() * theta.sin();
            let z = radius * phi.cos();

            stars.push(Star {
                position: Vec3::new(x, y, z),
                brightness: rng.gen_range(0.5..1.0),
                size: rng.gen_range(1.0..2.0),
            });
        }

        Skybox {
            stars,
            radius,
        }
    }

    pub fn render(&self, framebuffer: &mut Framebuffer, uniforms: &Uniforms) {
        for star in &self.stars {
     
            let world_pos = uniforms.view_matrix * nalgebra_glm::Vec4::new(
                star.position.x,
                star.position.y,
                star.position.z,
                1.0
            );
            
            let mut transformed = uniforms.projection_matrix * world_pos;
            transformed /= transformed.w;

       
            if transformed.z < 1.0 {
                let screen_x = ((transformed.x + 1.0) * framebuffer.width as f32 / 2.0) as usize;
                let screen_y = ((1.0 - transformed.y) * framebuffer.height as f32 / 2.0) as usize;

           
                let intensity = (star.brightness * 255.0) as u32;
                let color = (intensity << 16) | (intensity << 8) | intensity;

                if screen_x < framebuffer.width && screen_y < framebuffer.height {
                    framebuffer.set_current_color(color);
                    

                    let size = star.size as usize;
                    for dy in 0..size {
                        for dx in 0..size {
                            let px = screen_x.saturating_add(dx).saturating_sub(size/2);
                            let py = screen_y.saturating_add(dy).saturating_sub(size/2);
                            if px < framebuffer.width && py < framebuffer.height {
                                framebuffer.point(px, py, 0.9999); 
                            }
                        }
                    }
                }
            }
        }
    }
}