use nannou::prelude::*;
use std::collections::HashMap;

// Distance units are in kilometers and time is measured in seconds
struct Planetoid {
    position: Vector2,
    size: Vector2,
    speed: Vector2,
    mass: f32,
    is_active: bool,
    color: Rgb<u8>,
}

struct Model {
    gravitational_const: f32,
    time_scale: u8,
    update_rate: u8,
    planetoids: HashMap<String, Planetoid>,
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let mut planetoids: HashMap<String, Planetoid> = HashMap::new();
    planetoids.insert(
        "DAY-GO-BAH".to_string(),
        Planetoid {
            position: vec2(0.0, 0.0),
            size: vec2(20.0, 20.0),
            speed: vec2(0.0, 0.0),
            mass: 6.0 * 10.0.powi(24),
            is_active: true,
            color: GREEN,
        },
    );

    planetoids.insert(
        "Tattoo ween".to_string(),
        Planetoid {
            position: vec2(-450.0, 0.0),
            size: vec2(10.0, 10.0),
            speed: vec2(0.0, -25.0),
            mass: 3.0 * 10.0.powi(22),
            is_active: true,
            color: ORANGE,
        },
    );

    planetoids.insert(
        "Tattoo weenie".to_string(),
        Planetoid {
            position: vec2(-425.0, 0.0),
            size: vec2(2.0, 2.0),
            speed: vec2(1.0, -20.0),
            mass: 3.0 * 10.0.powi(17),
            is_active: true,
            color: ORANGE,
        },
    );

    Model {
        // Gravitational Constant to the -20 for km rather than m
        gravitational_const: 6.674 * 10.0.powi(-20),
        time_scale: 1,
        update_rate: 60,
        planetoids,
    }
}

// Update runs 60fps default
fn update(_app: &App, model: &mut Model, _update: Update) {
    let influences = get_gravitational_influences(&model);

    for info in model.planetoids.iter_mut() {
        let (name, planetoid) = info;
        if planetoid.is_active {
            let progress_per_second: f32 = model.time_scale as f32 / model.update_rate as f32;
            planetoid.speed += *influences.get(name).unwrap() * progress_per_second;
            planetoid.position += planetoid.speed * progress_per_second;
        }
    }
}

/// Returns a Hash Map the gravitational influence for each planet. (Influence measured
/// by acceleration [km/s^2])
///
/// # Arguments
///
/// * `model` - A nannou model that has information about the planets
///
fn get_gravitational_influences(model: &Model) -> HashMap<String, Vector2> {
    let mut influences: HashMap<String, Vector2> = HashMap::new();

    for info in model.planetoids.iter() {
        let (name, planetoid) = info;
        for info_2 in model.planetoids.iter() {
            let (name_2, planetoid_2) = info_2;
            if (name != name_2) && planetoid.is_active {
                let distance: f32 = ((planetoid_2.position.x - planetoid.position.x).powi(2)
                    + (planetoid_2.position.y - planetoid.position.y).powi(2))
                .sqrt();

                let gravitational_force: f32 =
                    model.gravitational_const * planetoid.mass * planetoid_2.mass / distance.powi(2);

                let acceleration: Vector2 = vec2(
                    planetoid_2.position.x - planetoid.position.x,
                    planetoid_2.position.y - planetoid.position.y,
                ).normalize() * (gravitational_force / planetoid.mass);

                if influences.contains_key(name) {
                    let mut influence = influences.get_mut(name).unwrap();
                    influence.x += acceleration.x;
                    influence.y += acceleration.y;

                } else {
                    influences.insert(name.clone(), acceleration);
                }
            }
        }
    }

    influences
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for planet_info in model.planetoids.iter() {
        let (_, planetoid) = planet_info;
        draw.ellipse()
            .xy(planetoid.position)
            .wh(planetoid.size)
            .color(planetoid.color);
    }

    draw.to_frame(app, &frame).unwrap();
}
