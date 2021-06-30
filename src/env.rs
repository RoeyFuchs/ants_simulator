use super::ant::Ant;
use super::configuration::{HIGHT, WIDTH};
use super::utils::{distance, pos_angle};
use multi_dim_point::Point;
use std::f32::consts::PI;

const TTL: u16 = 350; // how many steps the signs will stay in the environment
const RADIUS: f32 = 15.0; // how far the ant can grab a food or see a sign

pub struct Env {
    ants: Vec<Ant>,
    food: Vec<Point<f32>>,
    home: Point<f32>,
    food_signs: Vec<Vec<Vec<(Point<f32>, f32, u16)>>>, // matrix of tuple of location, heading and TTL
    radius: f32,
    max_ttl: u16, // when food signs will disappear
}

impl Env {
    pub fn new(ants: Vec<Ant>, food: Vec<Point<f32>>, home: Point<f32>) -> Env {
        let food_signs: Vec<Vec<Vec<(Point<f32>, f32, u16)>>> =
            vec![vec![Vec::new(); WIDTH as usize]; HIGHT as usize];
        Env {
            ants,
            food,
            home,
            food_signs: food_signs,
            radius: RADIUS,
            max_ttl: TTL,
        }
    }
    pub fn get_ants(&self) -> &Vec<Ant> {
        &self.ants
    }
    pub fn get_food(&self) -> &Vec<Point<f32>> {
        &self.food
    }
    pub fn get_home(&self) -> &Point<f32> {
        &self.home
    }

    fn get_relevant_signs<'a>(
        signs: &'a Vec<Vec<Vec<(Point<f32>, f32, u16)>>>,
        radius: usize,
        loc: &Point<f32>,
    ) -> Vec<&'a (Point<f32>, f32, u16)> {
        let loc_x = *loc.get_value(1) as isize;
        let loc_y = *loc.get_value(2) as isize;
        let relevant_x: Vec<usize> = (loc_x - radius as isize..loc_x + radius as isize)
            .filter(|x| *x < WIDTH as isize && *x >= 0)
            .map(|x| x as usize)
            .collect();
        let relevant_y: Vec<usize> = (loc_y - radius as isize..loc_y + radius as isize)
            .filter(|y| *y < HIGHT as isize && *y >= 0)
            .map(|y| y as usize)
            .collect();

        let mut relevant_points: Vec<&(Point<f32>, f32, u16)> = Vec::new();
        for y in relevant_y.iter() {
            for x in relevant_x.iter() {
                signs[*y][*x].iter().for_each(|p| relevant_points.push(&p));
            }
        }
        relevant_points
    }

    //update the TTL (-1), and remove any sign with TTL = 0
    fn clean_signs(&mut self) {
        for r in self.food_signs.iter_mut() {
            for c in r.iter_mut() {
                c.iter_mut().for_each(|x| x.2 = x.2 - 1);
                c.retain(|x| x.2 > 0);
            }
        }
    }

    pub fn do_step(&mut self) {
        for ant in self.ants.iter_mut() {
            let ant_loc: Point<f32> = ant.get_location();
            let ant_heading = ant.get_heading();
            if ant.is_carry_food() {
                self.food_signs[*ant_loc.get_value(2) as usize][*ant_loc.get_value(1) as usize]
                    .push((ant_loc.clone(), ant_heading - PI, self.max_ttl));
                if distance(&ant_loc, &self.home) <= self.radius {
                    // if we are at home
                    ant.set_carry_food(false);
                } else {
                    // find the direction to home and step towards
                    let y_diff = self.home.get_value(2) - ant_loc.get_value(2);
                    let x_diff = self.home.get_value(1) - ant_loc.get_value(1);
                    let angle_to_home = y_diff.atan2(x_diff);
                    ant.step_heading(pos_angle(&angle_to_home));
                }
            } else {
                //if the ant hasn't food
                let mut eated = None;
                for (pos, food) in self.food.iter_mut().enumerate() {
                    if distance(&ant_loc, &food) <= self.radius {
                        ant.set_carry_food(true);
                        eated = Some(pos);
                        break;
                    }
                }

                match eated {
                    Some(x) => {
                        self.food.remove(x);
                        break;
                    }
                    _ => (),
                };

                // searching for a food sign. will happen only when didn't find a food
                let mut found_food_sign = false;
                for food_sign in
                    Env::get_relevant_signs(&self.food_signs, self.radius as usize, &ant_loc)
                {
                    if distance(&ant_loc, &food_sign.0) <= self.radius {
                        ant.step_heading(food_sign.1);
                        found_food_sign = true;
                        break;
                    }
                }
                if !found_food_sign {
                    ant.do_blind_step();
                }
            }
        }
        self.clean_signs();
    }
}
