use fastnoise_lite::FastNoiseLite;
use gothspace::camera::Camera;
use gothspace::fragment::planets::{
    create_disco_planet, create_face_planet, create_gas_giant, create_green_planet,
    create_ocean_planet, create_snow_planet, create_sun,
};
use gothspace::fragment::ship::{create_ship, translation_from_camera};
use gothspace::fragment::skybox::create_skybox;
use gothspace::render::render;
use gothspace::texture::{GameTextures, Texture};
use gothspace::vertex::shader::{
    create_projection_matrix, create_view_matrix, create_viewport_matrix, Uniforms,
};
use gothspace::{framebuffer, EntityModel};
use gothspace::{Message, Model};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use mouse_rs::types::Point;
use mouse_rs::Mouse;
use nalgebra_glm::{vec3, Vec3};
use std::collections::VecDeque;
use std::f32::consts::PI;
use std::time::{Duration, Instant};

const ZOOM_SPEED: f32 = 1.0;
const ROTATION_SPEED: f32 = PI / 60.0;
const PLAYER_SPEED: f32 = 0.2;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 800;
    let framebuffer_height =
        (window_height as f32 / window_width as f32) * framebuffer_width as f32;
    let framebuffer_height = framebuffer_height as usize;

    println!("Framebuffer: ({framebuffer_width}, {framebuffer_height})");

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    // framebuffer.set_background_color(0x333355);

    let space_texture = Texture::new("assets/textures/space.png");
    framebuffer.set_background_from_texture(space_texture);

    let window_options = WindowOptions {
        // resize: true,
        // scale: minifb::Scale::FitScreen,
        ..WindowOptions::default()
    };

    let title_prefix = "3D Rendering";
    let mut window =
        Window::new(title_prefix, window_width, window_height, window_options).unwrap();
    window.set_key_repeat_delay(0.01);
    window.set_cursor_visibility(true);

    let (wx, wy) = window.get_position();
    let mouse = Mouse::new();
    let mut previous_mouse_pos = center_of_screen(
        wx as i32,
        wy as i32,
        window_width as i32,
        window_height as i32,
    );
    mouse
        .move_to(previous_mouse_pos.x, previous_mouse_pos.y)
        .unwrap();

    let window_maxs = Point {
        x: wx as i32 + window_width as i32,
        y: wy as i32 + window_height as i32,
    };

    let window_mins = Point {
        x: wx as i32,
        y: wy as i32,
    };

    let target_framerate = 60;
    let frame_delay = Duration::from_millis(1000 / target_framerate);

    let mut data = init(
        (window_width, window_height),
        (framebuffer_width, framebuffer_height),
    );
    let mut noise = FastNoiseLite::with_seed(1506);
    noise.set_frequency(Some(0.004));
    render(&mut framebuffer, &data, &mut noise);

    let mut splash_timer = 0;
    let splash_delay = 300;

    let mode_cooldown = 5;
    let mut mode_cooldown_timer = 0;

    let last_recorded_frames_max_count = 60;
    let mut last_recorded_frames = VecDeque::with_capacity(last_recorded_frames_max_count);
    let mut time = 0.0;
    while window.is_open() {
        let mut should_update = false;
        let start = Instant::now();
        mode_cooldown_timer = (mode_cooldown_timer - 1).max(0);
        splash_timer = (splash_timer + 1).min(splash_delay + 1);

        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        let mut messages: Vec<Message> = window
            .get_keys_pressed(KeyRepeat::Yes)
            .into_iter()
            .filter_map(|key| match key {
                Key::W => Some(Message::Advance(PLAYER_SPEED)),
                Key::S => Some(Message::Advance(-PLAYER_SPEED)),

                Key::Key1 => Some(Message::ChangePlanet(create_disco_planet())),
                Key::Key2 => Some(Message::ChangePlanet(create_ocean_planet())),
                Key::Key3 => Some(Message::ChangePlanet(create_gas_giant())),
                Key::Key4 => Some(Message::ChangePlanet(create_face_planet())),
                Key::Key5 => Some(Message::ChangePlanet(create_snow_planet())),
                Key::Key6 => Some(Message::ChangePlanet(create_sun())),
                Key::Key7 => Some(Message::ChangePlanet(create_green_planet())),

                // Key::Space => match (mode_cooldown_timer, &data.status) {
                //     (0, GameStatus::MainMenu) => {
                //         mode_cooldown_timer = mode_cooldown;
                //         Some(Message::StartGame)
                //     }
                //     _ => None,
                // },
                // Key::R => match (mode_cooldown_timer, &data.status) {
                //     (0, GameStatus::YouLost) | (0, GameStatus::YouWon) => {
                //         mode_cooldown_timer = mode_cooldown;
                //         Some(Message::RestartGame)
                //     }
                //     _ => None,
                // },
                _ => None,
            })
            .collect();
        should_update = true;
        messages.push(Message::UpdateTime(time));

        let Point { x, y } = mouse.get_position().unwrap();
        messages.push(Message::RotateCamera(
            (previous_mouse_pos.x - x) as f32 * ROTATION_SPEED,
            (previous_mouse_pos.y - y) as f32 * ROTATION_SPEED,
        ));
        previous_mouse_pos = Point { x, y };

        if previous_mouse_pos.x < window_mins.x || previous_mouse_pos.x > window_maxs.x {
            previous_mouse_pos = center_of_screen(
                wx as i32,
                wy as i32,
                window_width as i32,
                window_height as i32,
            );
            mouse
                .move_to(previous_mouse_pos.x, previous_mouse_pos.y)
                .unwrap();
        }

        if previous_mouse_pos.y < window_mins.y || previous_mouse_pos.y > window_maxs.y {
            previous_mouse_pos = center_of_screen(
                wx as i32,
                wy as i32,
                window_width as i32,
                window_height as i32,
            );
            mouse
                .move_to(previous_mouse_pos.x, previous_mouse_pos.y)
                .unwrap();
        }

        for msg in messages {
            data = update(data, msg);
        }

        if data.camera.has_changed() || should_update {
            framebuffer.clear();
            render(&mut framebuffer, &data, &mut noise);
        }
        data.camera.reset_change();

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .expect("Couldn't update the framebuffer!");
        let end = Instant::now();
        if last_recorded_frames.len() == last_recorded_frames_max_count {
            last_recorded_frames.pop_front();
        }
        let render_millis = (end - start).as_millis();
        last_recorded_frames.push_back(render_millis);
        time += render_millis as f32;

        let avg_millis: f32 = last_recorded_frames.iter().map(|&u| u as f32).sum::<f32>()
            / last_recorded_frames_max_count as f32;
        let avg_frames = 1000.0 / avg_millis;
        window.set_title(format!("{} - {:.2} fps", title_prefix, avg_frames).as_ref());
        std::thread::sleep(frame_delay);
    }
}

/// Init the default state
fn init(window_dimensions: (usize, usize), framebuffer_dimensions: (usize, usize)) -> Model {
    let (framebuffer_width, framebuffer_height) = framebuffer_dimensions;
    let (window_width, window_height) = window_dimensions;

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let ship = create_ship(&camera);
    let planet = create_green_planet();
    let skybox = create_skybox();
    let entities = vec![skybox, planet];
    // let entities = vec![];

    let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
    println!("View Matrix: {:#?}", view_matrix);
    let projection_matrix = create_projection_matrix(window_width as f32, window_height as f32);
    println!("Projection Matrix: {:#?}", projection_matrix);
    let viewport_matrix =
        create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
    println!("Viewport matrix: {:#?}", viewport_matrix);

    let textures = GameTextures::new("assets/textures/");
    Model {
        textures,
        entities,
        ship,
        uniforms: Uniforms {
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time: 0.0,
        },
        camera,
    }
}

fn update(data: Model, msg: Message) -> Model {
    match msg {
        Message::RotateCamera(delta_yaw, delta_pitch) => {
            let Model {
                mut camera,
                uniforms,
                mut ship,
                ..
            } = data;

            let dir = vec3(delta_yaw, delta_pitch, 1.0).normalize();
            camera.move_center(dir, ROTATION_SPEED);

            let uniforms = Uniforms {
                view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
                ..uniforms
            };

            let rotation_y = ship.model.rotation.y - delta_yaw * ROTATION_SPEED / 2.0;
            let rotation_x = ship.model.rotation.x - delta_pitch * ROTATION_SPEED / 2.0;
            ship.modify_model(EntityModel {
                rotation: vec3(rotation_x, rotation_y, ship.model.rotation.z),
                translation: translation_from_camera(&camera),
                ..ship.model
            });

            Model {
                ship,
                uniforms,
                camera,
                ..data
            }
        }

        Message::Advance(delta) => {
            let Model {
                mut camera,
                uniforms,
                mut ship,
                ..
            } = data;

            camera.advance_camera(delta);
            let uniforms = Uniforms {
                view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
                ..uniforms
            };

            ship.modify_model(EntityModel {
                translation: translation_from_camera(&camera),
                ..ship.model
            });

            Model {
                uniforms,
                ship,
                camera,
                ..data
            }
        }

        Message::UpdateTime(time) => {
            let Model { uniforms, .. } = data;

            let uniforms = Uniforms { time, ..uniforms };

            Model { uniforms, ..data }
        }
        Message::ChangePlanet(entity) => {
            let entities = vec![entity];

            Model { entities, ..data }
        }
    }
}

fn center_of_screen(wx: i32, wy: i32, window_width: i32, window_height: i32) -> Point {
    Point {
        x: wx + window_width / 2,
        y: wy + window_height / 2,
    }
}

fn vec_from_angles(alpha: f32, beta: f32) -> Vec3 {
    let (alphasin, alphacos) = alpha.sin_cos();
    let (betasin, betacos) = beta.sin_cos();

    let x = alphacos * betacos;
    let z = alphasin * betacos;
    let y = betasin;

    vec3(x, y, z)
}
