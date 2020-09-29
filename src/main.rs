use nannou::prelude::*;
use std::collections::HashMap;

// Distance units are in kilometers and time is measured in seconds
struct Planet {
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
    planets: HashMap<String, Planet>,
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let mut planets: HashMap<String, Planet> = HashMap::new();
    planets.insert(
        "earth".to_string(),
        Planet {
            position: vec2(0.0, 0.0),
            size: vec2(6.3, 6.3),
            speed: vec2(0.0, 0.0),
            mass: 5.97 * 10.0.powi(24),
            is_active: true,
            color: GREEN,
        },
    );

    planets.insert(
        "moon".to_string(),
        Planet {
            position: vec2(384400.0, 0.0),
            size: vec2(1.7, 1.7),
            speed: vec2(0.0, 1.022),
            mass: 7.35 * 10.0.powi(22),
            is_active: true,
            color: WHITE,
        },
    );

    planets.insert(
        "fake_moon".to_string(),
        Planet {
            position: vec2(110.0, 0.0),
            size: vec2(3.0, 3.0),
            speed: vec2(0.0, 0.1),
            mass: 3.0 * 10.0.powi(18),
            is_active: true,
            color: PURPLE,
        },
    );

    Model {
        // Gravitational Constant to the -20 for km rather than m
        gravitational_const: 6.674 * 10.0.powi(-20),
        time_scale: 1,
        update_rate: 60,
        planets,
    }
}

// Update runs 60fps default
fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut influences: HashMap<String, Vector2> = HashMap::new();

    for info in model.planets.iter() {
        let (name, planet) = info;
        for info_2 in model.planets.iter() {
            let (name_2, planet_2) = info_2;
            if (name != name_2) && planet.is_active {
                let distance: f32 = ((planet_2.position.x - planet.position.x).powi(2)
                    + (planet_2.position.y - planet.position.y).powi(2))
                .sqrt();

                let gravitational_force: f32 =
                    model.gravitational_const * planet.mass * planet_2.mass / distance.powi(2);

                let acceleration: Vector2 = vec2(
                    planet_2.position.x - planet.position.x,
                    planet_2.position.y - planet.position.y,
                ) / distance
                    * (gravitational_force / planet.mass);

                influences.insert(name.clone(), acceleration);
            }
        }
    }

    for info in model.planets.iter_mut() {
        let (name, planet) = info;
        if planet.is_active {
            let progress_per_second: f32 = model.time_scale as f32 / model.update_rate as f32;
            planet.speed += *influences.get(name).unwrap() * progress_per_second;
            planet.position += planet.speed * progress_per_second;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for planet_info in model.planets.iter() {
        let (_, planet) = planet_info;
        draw.ellipse()
            .xy(planet.position)
            .wh(planet.size)
            .color(planet.color);
    }

    draw.to_frame(app, &frame).unwrap();
}
