use fastnoise_lite::FastNoiseLite;
use gothspace::camera::Camera;
use gothspace::color::Color;
use gothspace::fragment::ellipsis::next_point_in_ellipsis;
use gothspace::fragment::planets::{
    create_disco_planet, create_face_planet, create_gas_giant, create_green_planet,
    create_ocean_planet, create_snow_planet, create_sun,
};
use gothspace::fragment::ship::{
    create_ship, create_ship_from, ORIGINAL_ROTATION,
};
use gothspace::light::Light;
use gothspace::render::render;
use gothspace::skybox::Skybox;
use gothspace::texture::GameTextures;
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

const ZOOM_SPEED: f32 = 0.1;
const ROTATION_SPEED: f32 = PI * 1e-3;
const SHIP_ROTATION_SPEED: f32 = PI * 5e-2;
const PLAYER_ACCELERATION: f32 = 1e-3;
const MAX_PLAYER_SPEED: f32 = 0.3;
const CAM_POS_DELTA_TO_SHIP: Vec3 = Vec3::new(0.0, 1.0, 10.0);
const CAM_CENTER_DELTA_TO_SHIP: Vec3 = Vec3::new(0.0, 1.5, 0.0);

fn main() {
    let window_width = 1080;
    let window_height = 720;

    let framebuffer_width = 1500;
    let framebuffer_height =
        (window_height as f32 / window_width as f32) * framebuffer_width as f32;
    let framebuffer_height = framebuffer_height as usize;

    println!("Framebuffer: ({framebuffer_width}, {framebuffer_height})");

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    // framebuffer.set_background_color(0x333355);

    let window_options = WindowOptions {
        resize: true,
        scale: minifb::Scale::FitScreen,
        ..WindowOptions::default()
    };

    let title_prefix = "3D Rendering";
    let mut window =
        Window::new(title_prefix, window_width, window_height, window_options).unwrap();
    let mut window_size = window.get_size();
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
                Key::W => Some(Message::Accelerate(PLAYER_ACCELERATION)),
                Key::S => Some(Message::Accelerate(-PLAYER_ACCELERATION)),

                Key::Up => Some(Message::RotateShip(
                    vec3(1.0, 0.0, 0.0) * SHIP_ROTATION_SPEED,
                )),
                Key::Down => Some(Message::RotateShip(
                    vec3(-1.0, 0.0, 0.0) * SHIP_ROTATION_SPEED,
                )),
                Key::Left => Some(Message::RotateShip(
                    vec3(0.0, 1.0, 0.0) * SHIP_ROTATION_SPEED,
                )),
                Key::Right => Some(Message::RotateShip(
                    vec3(0.0, -1.0, 0.0) * SHIP_ROTATION_SPEED,
                )),

                Key::Tab => {
                    if mode_cooldown_timer == 0 {
                        mode_cooldown_timer = mode_cooldown;
                        Some(Message::AlternateView)
                    } else {
                        None
                    }
                }

                Key::Space => Some(Message::StopShip),

                _ => None,
            })
            .collect();
        should_update = true;
        messages.push(Message::UpdateTime(time));
        mode_cooldown_timer = (mode_cooldown_timer - 1).max(0);

        let Point { x, y } = mouse.get_position().unwrap();
        messages.push(Message::RotateCamera(
            (x - previous_mouse_pos.x) as f32 * ROTATION_SPEED,
            (previous_mouse_pos.y - y) as f32 * ROTATION_SPEED,
        ));
        previous_mouse_pos = Point { x, y };

        if let Some(delta) = window.get_scroll_wheel().map(|(_, y)| y) {
            messages.push(Message::ZoomCamera(delta * ZOOM_SPEED));
        }

        let current_window_size = window.get_size();
        if current_window_size != window_size {
            window_size = current_window_size;
            messages.push(Message::ResizeWindow(window_size));
        }

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

    let starting_ship_position = vec3(0.0, -2.0, 35.0);
    let ship = create_ship(starting_ship_position);
    let camera = Camera::new(
        starting_ship_position + CAM_POS_DELTA_TO_SHIP,
        starting_ship_position + CAM_CENTER_DELTA_TO_SHIP,
        Vec3::new(0.0, 1.0, 0.0),
        2.0,
    );
    let sun = create_sun(vec3(0.0, 0.0, 0.0));
    let green_planet = create_green_planet();
    let disco_planet = create_disco_planet();
    let gas_planet = create_gas_giant();
    let face_planet = create_face_planet();
    let snow_planet = create_snow_planet();
    let ocean_planet = create_ocean_planet();
    let entities = vec![
        sun,
        green_planet,
        disco_planet,
        gas_planet,
        face_planet,
        snow_planet,
        ocean_planet,
    ];

    let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
    println!("View Matrix: {:#?}", view_matrix);
    let projection_matrix = create_projection_matrix(window_width as f32, window_height as f32);
    println!("Projection Matrix: {:#?}", projection_matrix);
    let viewport_matrix =
        create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
    println!("Viewport matrix: {:#?}", viewport_matrix);

    let lights = vec![Light {
        position: Vec3::zeros(),
        color: Color::white(),
        intensity: 1.0,
    }];

    let skybox = Skybox::new(5000, 50.0);
    let textures = GameTextures::new("assets/textures/");

    Model {
        view_type: gothspace::ViewType::FirstPerson,
        textures,
        entities,
        previous_fpv_state: (create_ship_from(&ship), camera),
        ship,
        uniforms: Uniforms {
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time: 0.0,
        },
        camera,
        lights,
        skybox,
    }
}

fn update(data: Model, msg: Message) -> Model {
    match msg {
        Message::RotateCamera(delta_yaw, delta_pitch) => {
            let Model {
                mut camera,
                uniforms,
                ..
            } = data;

            camera.orbit(delta_yaw, delta_pitch);

            let uniforms = Uniforms {
                view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
                ..uniforms
            };

            Model {
                uniforms,
                camera,
                ..data
            }
        }

        Message::Accelerate(delta) => {
            let Model {
                camera,
                uniforms,
                mut ship,
                ..
            } = data;

            let ship_rotation = ship.entity.model.rotation - ORIGINAL_ROTATION;
            let ship_direction = vec3(
                -ship_rotation.y.sin(),
                -ship_rotation.x.sin(),
                -ship_rotation.z.cos(),
            )
            .normalize();
            // println!("{ship_rotation:?} -> {ship_direction:?}");
            // let ship_direction =
            //     (ship.entity.model.rotation.cross(&vec3(1.0, 0.0, 0.0))).normalize();
            ship.acceleration += ship_direction * delta;
            ship.velocity += ship.acceleration;
            if ship.velocity.magnitude() > MAX_PLAYER_SPEED {
                ship.velocity -= ship.acceleration;
            }

            Model {
                uniforms,
                ship,
                camera,
                ..data
            }
        }

        Message::UpdateTime(time) => {
            let Model {
                uniforms,
                mut entities,
                mut ship,
                mut camera,
                ..
            } = data;
            let uniforms = Uniforms { time, ..uniforms };

            for entity in &mut entities {
                if let Some(ref info) = entity.ellipsis {
                    let new_position = next_point_in_ellipsis(time * info.velocity, info);
                    entity.modify_model(EntityModel {
                        translation: new_position,
                        ..entity.model
                    });
                }
            }

            let previous_position = ship.entity.model.translation;
            ship.velocity += ship.acceleration;
            if ship.velocity.magnitude() > MAX_PLAYER_SPEED {
                ship.velocity -= ship.acceleration;
            }
            let translation = previous_position + ship.velocity;
            ship.entity.modify_model(EntityModel {
                translation,
                ..ship.entity.model
            });

            camera.modify_center_and_eye(translation, camera.eye + ship.velocity);
            let uniforms = Uniforms {
                view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
                ..uniforms
            };

            Model {
                uniforms,
                entities,
                camera,
                ship,
                ..data
            }
        }

        Message::ZoomCamera(delta) => {
            let Model { mut camera, .. } = data;
            camera.zoom(delta);
            Model { camera, ..data }
        }

        Message::ResizeWindow(new_size) => {
            let Model { uniforms, .. } = data;

            let projection_matrix = create_projection_matrix(new_size.0 as f32, new_size.1 as f32);
            let uniforms = Uniforms {
                projection_matrix,
                ..uniforms
            };

            Model { uniforms, ..data }
        }

        Message::AlternateView => {
            let Model {
                ship,
                mut camera,
                view_type,
                previous_fpv_state,
                ..
            } = data;

            match view_type {
                gothspace::ViewType::BirdEye => {
                    let ship = create_ship_from(&previous_fpv_state.0);
                    let camera = previous_fpv_state.1;

                    Model {
                        ship,
                        camera,
                        previous_fpv_state,
                        view_type: gothspace::ViewType::FirstPerson,
                        ..data
                    }
                }
                gothspace::ViewType::FirstPerson => {
                    // Saving view state to know when to return...
                    let previous_fpv_state = (create_ship_from(&ship), camera);

                    camera.eye = vec3(0.0, 130.0, 0.0);
                    camera.center = Vec3::zeros();

                    Model {
                        ship,
                        camera,
                        previous_fpv_state,
                        view_type: gothspace::ViewType::BirdEye,
                        ..data
                    }
                }
            }
        }

        Message::StopShip => {
            let Model { mut ship, .. } = data;

            ship.velocity = Vec3::zeros();
            ship.acceleration = Vec3::zeros();

            Model { ship, ..data }
        }

        Message::RotateShip(rotation) => {
            let Model { mut ship, .. } = data;

            ship.entity.modify_model(EntityModel {
                rotation: ship.entity.model.rotation + rotation,
                ..ship.entity.model
            });

            Model { ship, ..data }
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
