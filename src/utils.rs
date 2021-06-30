use crate::ant;
use crate::configuration;
use crate::env;
use multi_dim_point::Point;
use serde_json::Value;
use std::f32::consts::PI;
use std::fs;
use std::process;

//ggez draw object from the left upper corner, and we want it to draw from the center
pub fn transform_obj(p: &Point<f32>) -> Point<f32> {
    const W: f32 = 15.0; //TO-DO - read the actual size of the resource. we assume here that all the objects are in same shape
    const H: f32 = 15.0;
    Point::new_from_vec(&vec![
        p.get_value(1) - (W / 2.0),
        p.get_value(2) - (H / 2.0),
    ])
}

//change a angle to possitive value
pub fn pos_angle(a: &f32) -> f32 {
    let new_a: f32 = if a.is_sign_negative() {
        2.0 * PI + a
    } else {
        *a
    };
    new_a
}

//euclidean distance
pub fn distance(p1: &Point<f32>, p2: &Point<f32>) -> f32 {
    p1.get_vector()
        .iter()
        .zip(p2.get_vector().iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f32>()
        .sqrt()
}

pub fn read_json(filename: &str) -> env::Env {
    fn create_ants_from_json(ant_info: &Value) -> Vec<ant::Ant> {
        let x = ant_info["x"].as_f64().unwrap_or(0.0) as f32;
        let y = ant_info["y"].as_f64().unwrap_or(0.0) as f32;
        let max_x = configuration::WIDTH;
        let max_y = configuration::HIGHT;
        let amount = ant_info["amount"].as_i64().unwrap_or(1);
        let mut ants: Vec<ant::Ant> = Vec::new();
        for _ in 0..amount {
            ants.push(ant::Ant::new(x, y, max_x, max_y));
        }
        return ants;
    }
    fn create_foods_from_json(food_info: &Value) -> Vec<Point<f32>> {
        let x = food_info["x"].as_f64().unwrap_or(0.0) as f32;
        let y = food_info["y"].as_f64().unwrap_or(0.0) as f32;
        let amount = food_info["amount"].as_i64().unwrap_or(1);
        let mut foods: Vec<Point<f32>> = Vec::new();
        for _ in 0..amount {
            foods.push(Point::new_from_vec(&vec![x, y]));
        }
        return foods;
    }
    fn create_home_from_json(home_info: &Value) -> Point<f32> {
        let x = home_info["x"].as_f64().unwrap_or(0.0) as f32;
        let y = home_info["y"].as_f64().unwrap_or(0.0) as f32;
        Point::new_from_vec(&vec![x, y])
    }

    let content = match fs::read_to_string(filename) {
        //read json file
        Ok(x) => x,
        Err(_) => {
            eprintln!("Cannot open the json file");
            process::exit(1);
        }
    };

    let json_values: Value = serde_json::from_str(&content).unwrap(); //parse the json into Value Enum

    let ants_json: &Vec<Value> = match json_values["Ants"].as_array() {
        Some(x) => x,
        _ => {
            eprintln!("Cannot find \"Ants\" in the json");
            process::exit(1);
        }
    };
    let mut ants_vec: Vec<Vec<ant::Ant>> = Vec::new();
    ants_json
        .iter()
        .for_each(|a| ants_vec.push(create_ants_from_json(a)));
    let ants_vec: Vec<ant::Ant> = ants_vec.into_iter().flatten().collect();

    let food_json: &Vec<Value> = match json_values["Food"].as_array() {
        Some(x) => x,
        _ => {
            eprintln!("Cannot find \"Food\" in the json");
            process::exit(1);
        }
    };
    let mut food_vec: Vec<Vec<Point<f32>>> = Vec::new();
    food_json
        .iter()
        .for_each(|a| food_vec.push(create_foods_from_json(a)));
    let food_vec: Vec<Point<f32>> = food_vec.into_iter().flatten().collect();

    let home_json: &Vec<Value> = match json_values["Home"].as_array() {
        Some(x) => x,
        _ => {
            eprintln!("Cannot find \"Home\" in the json");
            process::exit(1);
        }
    };

    let home: Point<f32> = create_home_from_json(&home_json[0]); //we assume that we have only 1 home! TODO - adding option to more than 1

    env::Env::new(ants_vec, food_vec, home)
}
