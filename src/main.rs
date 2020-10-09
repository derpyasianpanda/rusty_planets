// Use this to hide CMD in production
// #![windows_subsystem = "windows"]

use nannou::prelude::*;
use std::collections::HashMap;

enum CreationState {
    Radius,
    Speed,
    Nil,
}

// Distance units are in kilometers and time is measured in seconds
struct Planetoid {
    position: Vector2,
    radius: f32,
    speed: Vector2,
    mass: f32,
    is_active: bool,
    color: Rgb<u8>,
}

struct Model {
    window: WindowId,
    background_texture: wgpu::Texture,
    creation_state: CreationState,
    gravitational_const: f32,
    density: f32,
    time_scale: u8,
    update_rate: u8,
    planetoids: Vec<Planetoid>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(1280, 720)
        .title("Rusty Planets")
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("space.jpg");
    let background_texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model {
        window,
        background_texture,
        creation_state: CreationState::Nil,
        // Gravitational Constant to the -20 for km rather than m
        gravitational_const: 6.674 * 10.0.powi(-20),
        // Arbitrary Density Value to make visualization more appealing (kg/km^3)
        density: 7.0 * 10.0.powi(18),
        time_scale: 1,
        update_rate: 60,
        planetoids: vec![],
    }
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(MouseButton::Left) => {
            handle_left_click(app, model);
        }

        MousePressed(MouseButton::Right) => {
            match model.creation_state {
                // Delete a planet
                CreationState::Nil => {
                    delete_planet_at_mouse(app, model);
                }

                // Cancel the creation process
                _ => {
                    model.creation_state = CreationState::Nil;
                    model.planetoids.pop();
                }
            }
        }

        KeyPressed(Key::Up) => {
            model.density *= 2.0;
        }

        KeyPressed(Key::Down) => {
            model.density /= 2.0;
        }

        _ => (),
    }
}

fn handle_left_click(app: &App, model: &mut Model) {
    match model.creation_state {
        CreationState::Nil => {
            model.planetoids.push(Planetoid {
                position: app.mouse.position(),
                radius: 2.0,
                speed: vec2(0.0, 0.0),
                mass: 2.0 * 10.0.powi(18),
                is_active: false,
                color: GREEN,
            });
            model.creation_state = CreationState::Radius;
        }

        CreationState::Radius => {
            model.creation_state = CreationState::Speed;
        }

        CreationState::Speed => {
            model.creation_state = CreationState::Nil;
            model.planetoids.last_mut().unwrap().is_active = true;
        }
    }
}

fn delete_planet_at_mouse(app: &App, model: &mut Model) {
    let mouse_position = app.mouse.position();
    for i in 0..model.planetoids.len() {
        let planetoid = &model.planetoids[i];
        if (mouse_position - planetoid.position).magnitude() <= planetoid.radius {
            model.planetoids.remove(i);
            break;
        }
    }
}

// Update runs 60fps default
fn update(app: &App, model: &mut Model, _update: Update) {
    let progress_per_update = model.time_scale as f32 / model.update_rate as f32;
    handle_collisions(model);
    calculate_gravitational_influences(model, progress_per_update);

    match model.creation_state {
        // Update Model in some way when creating a planet
        // Note: Fairly certain last element of the planetoids vector will
        // always reference the planetoid in creation
        CreationState::Radius => {
            let planetoid = model.planetoids.last_mut().unwrap();
            planetoid.radius = (planetoid.position - app.mouse.position()).magnitude();
            planetoid.mass = (4.0 / 3.0) * PI * planetoid.radius.powi(3) * model.density;
        }

        CreationState::Speed => {
            let planetoid = model.planetoids.last_mut().unwrap();
            let direction = app.mouse.position() - planetoid.position;
            if direction.magnitude() > 6.0 {
                planetoid.speed = direction.normalize() * direction.magnitude() / 2.0;
            } else {
                planetoid.speed = Vector2::new(0.0, 0.0);
            }
        }

        _ => (),
    }

    for planetoid in model.planetoids.iter_mut() {
        if planetoid.is_active {
            planetoid.position += planetoid.speed * progress_per_update;
        }
    }
}

fn handle_collisions(model: &mut Model) {
    let mut updates = HashMap::new();
    let mut removed = vec![];

    for i in 0..model.planetoids.len() {
        let planetoid = &model.planetoids[i];
        if planetoid.is_active {
            for j in 0..model.planetoids.len() {
                let planetoid_2 = &model.planetoids[j];
                if i != j && planetoid_2.is_active {
                    let distance = (planetoid.position - planetoid_2.position).magnitude();
                    if distance <= (planetoid.radius + planetoid_2.radius) {
                        if planetoid.mass > planetoid_2.mass {
                            updates.insert(
                                i,
                                (planetoid_2.mass, planetoid_2.radius, planetoid_2.speed),
                            );
                            removed.push(j);
                        } else {
                            updates.insert(j, (planetoid.mass, planetoid.radius, planetoid.speed));
                            removed.push(i);
                        }
                    }
                }
            }
        }
    }

    for (i, (mass, radius, speed)) in updates.iter() {
        let planetoid = &mut model.planetoids[*i];
        let mass_sum = mass + planetoid.mass;
        planetoid.mass += mass;
        planetoid.radius = (radius.powi(3) + planetoid.radius.powi(3)).powf(1.0 / 3.0);
        planetoid.speed.x = ((mass * speed.x) + (planetoid.mass * planetoid.speed.x)) / mass_sum;
        planetoid.speed.y = ((mass * speed.y) + (planetoid.mass * planetoid.speed.y)) / mass_sum;
    }

    removed.sort_unstable_by(|a, b| b.cmp(a));
    while !removed.is_empty() {
        removed.pop();
        model.planetoids.remove(removed.pop().unwrap());
    }
}

/// Calculates the gravitational influence for each planet and applies it
/// Influence measured by acceleration (km/s^2)
///
/// # Arguments
///
/// * `model` - A nannou model that has information about the planets
/// * `progress_per_update` - How much progress is applied per update tick
///
fn calculate_gravitational_influences(model: &mut Model, progress_per_update: f32) {
    for i in 0..model.planetoids.len() {
        let planetoid = &model.planetoids[i];
        if planetoid.is_active {
            let position = planetoid.position;
            let mass = planetoid.mass;
            for j in 0..model.planetoids.len() {
                let planetoid_2 = &mut model.planetoids[j];
                if i != j && planetoid_2.is_active {
                    let distance = (position - planetoid_2.position).magnitude();

                    let gravitational_force =
                        model.gravitational_const * mass * planetoid_2.mass / distance.powi(2);

                    let acceleration = (position - planetoid_2.position).normalize()
                        * (gravitational_force / planetoid_2.mass);

                    planetoid_2.speed += acceleration * progress_per_update;
                }
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window(model.window).unwrap();
    draw.texture(&model.background_texture)
        .wh(window.rect().wh());

    let title = &*format!(
        "Rusty Planets | Current Density: {:+e} kg/km^3",
        model.density
    );
    window.set_title(title);

    for planetoid in model.planetoids.iter() {
        draw.ellipse()
            .xy(planetoid.position)
            .radius(planetoid.radius)
            .color(planetoid.color);
    }

    match model.creation_state {
        CreationState::Speed => {
            let planetoid = model.planetoids.last().unwrap();
            draw.arrow()
                .weight(planetoid.speed.magnitude().log(5.0))
                .start(planetoid.position)
                .end(planetoid.position + planetoid.speed);
        }
        _ => (),
    }

    draw.to_frame(app, &frame).unwrap();
}
