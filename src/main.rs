use nannou::prelude::*;
use std::collections::HashMap;

struct Planet {
    position: Vector2,
    size: Vector2,
    speed: Vector2,
    mass: f32,
    color: Rgb<u8>
}

struct Model {
    gravity: f32,
    gravitational_const: f32,
    planets: HashMap<String, Planet>
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    let mut planets = HashMap::new();
    planets.insert("earth".to_string(), Planet {
        position: vec2(0.0, 0.0),
        size: vec2(50.0, 50.0),
        speed: vec2(5.0, 5.0),
        mass: 100.0,
        color: GREEN
    });

    planets.insert("moon".to_string(), Planet {
        position: vec2(100.0, 0.0),
        size: vec2(10.0, 10.0),
        speed: vec2(-5.0, 5.0),
        mass: 100.0,
        color: WHITE
    });

    Model {
        gravity: 9.81,
        gravitational_const: 6.673 * 10.0.powi(-11),
        planets: planets
    }
}

// Update runs 60fps default
fn update(_app: &App, model: &mut Model, _update: Update) {
    for planet_info in model.planets.iter_mut() {
        let (_, planet) = planet_info;
        planet.position += planet.speed / 60.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for planet_info in model.planets.iter() {
        let (name, planet) = planet_info;
        draw.ellipse().xy(planet.position).wh(planet.size).color(planet.color);
    }

    draw.to_frame(app, &frame).unwrap();
}
