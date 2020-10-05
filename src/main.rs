// Use this to hide CMD in production
// #![windows_subsystem = "windows"]

use nannou::prelude::*;
use std::collections::HashMap;

// Distance units are in kilometers and time is measured in seconds
struct Planetoid {
    position: Vector2,
    radius: f32,
    speed: Vector2,
    mass: f32,
    is_active: bool,
    color: Rgb<u8>,
}

enum CreationState {
    Radius,
    Speed,
    Nil,
}

struct Model {
    _window: WindowId,
    creation_state: CreationState,
    gravitational_const: f32,
    time_scale: u8,
    update_rate: u8,
    planetoids: Vec<Planetoid>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let mut planetoids = vec![];

    planetoids.push(Planetoid {
        position: vec2(0.0, 0.0),
        radius: 10.0,
        speed: vec2(0.0, 0.0),
        mass: 6.0 * 10.0.powi(24),
        is_active: true,
        color: GREEN,
    });

    planetoids.push(Planetoid {
        position: vec2(-450.0, 0.0),
        radius: 5.0,
        speed: vec2(0.0, 25.0),
        mass: 3.0 * 10.0.powi(22),
        is_active: true,
        color: ORANGE,
    });

    planetoids.push(Planetoid {
        position: vec2(-400.0, 0.0),
        radius: 8.0,
        speed: vec2(-1.0, 20.0),
        mass: 3.0 * 10.0.powi(19),
        is_active: true,
        color: ORANGE,
    });

    let _window = app
        .new_window()
        .size(1280, 720)
        .title("Rusty Planets")
        .event(event)
        .view(view)
        .build()
        .unwrap();

    Model {
        _window,
        creation_state: CreationState::Nil,
        // Gravitational Constant to the -20 for km rather than m
        gravitational_const: 6.674 * 10.0.powi(-20),
        time_scale: 1,
        update_rate: 60,
        planetoids,
    }
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(MouseButton::Left) => {
            // Begin Creation Process
            match model.creation_state {
                CreationState::Nil => creation_radius(app, model),
                CreationState::Radius => creation_speed(app, model),
                CreationState::Speed => creation_finish(app, model),
            }
        }
        MouseReleased(MouseButton::Left) => {
            // Launch planet or determine size (Both?)
        }
        _ => (),
    }
}

fn creation_radius(app: &App, model: &mut Model) {
    let mouse_position = app.mouse.position();
    model.creation_state = CreationState::Radius;

    println!("Radius @ {} {}", mouse_position.x, mouse_position.y);
}

fn creation_speed(app: &App, model: &mut Model) {
    let mouse_position = app.mouse.position();
    model.creation_state = CreationState::Speed;

    println!("Speed @ {} {}", mouse_position.x, mouse_position.y);
}

fn creation_finish(app: &App, model: &mut Model) {
    let mouse_position = app.mouse.position();
    model.creation_state = CreationState::Nil;

    println!("Finish @ {} {}", mouse_position.x, mouse_position.y);
}

// Update runs 60fps default
fn update(_app: &App, model: &mut Model, _update: Update) {
    let progress_per_update = model.time_scale as f32 / model.update_rate as f32;
    handle_collisions(model);
    calculate_gravitational_influences(model, progress_per_update);

    match model.creation_state {
        // Update Model in some way when creating a planet
        // Note: Fairly certain last element of the planetoids vector will
        // always reference the planetoid in creation
        CreationState::Nil => (),
        CreationState::Radius => (),
        CreationState::Speed => (),
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
        let mass_ratio = mass / planetoid.mass;
        planetoid.mass += mass;
        planetoid.radius += radius;
        planetoid.speed.x += speed.x * mass_ratio;
        planetoid.speed.y += speed.y * mass_ratio;
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
    let mut influences: HashMap<usize, Vector2> = HashMap::new();

    for i in 0..model.planetoids.len() {
        let planetoid = &model.planetoids[i];
        for j in 0..model.planetoids.len() {
            if i != j && planetoid.is_active {
                let planetoid_2 = &model.planetoids[j];
                let distance = ((planetoid_2.position.x - planetoid.position.x).powi(2)
                    + (planetoid_2.position.y - planetoid.position.y).powi(2))
                .sqrt();

                let gravitational_force =
                    model.gravitational_const * planetoid.mass * planetoid_2.mass
                        / distance.powi(2);

                let acceleration = vec2(
                    planetoid_2.position.x - planetoid.position.x,
                    planetoid_2.position.y - planetoid.position.y,
                )
                .normalize()
                    * (gravitational_force / planetoid.mass);

                let mut influence = influences.entry(i).or_default();
                influence.x += acceleration.x;
                influence.y += acceleration.y;
            }
        }
    }

    for (i, influence) in influences.iter() {
        model.planetoids[*i].speed += *influence * progress_per_update;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    match model.creation_state {
        CreationState::Speed => {
            // Draw an arrow or something
        }
        _ => (),
    }

    for planetoid in model.planetoids.iter() {
        draw.ellipse()
            .xy(planetoid.position)
            .radius(planetoid.radius)
            .color(planetoid.color);
    }

    draw.to_frame(app, &frame).unwrap();
}
