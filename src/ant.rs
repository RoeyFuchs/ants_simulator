use multi_dim_point::Point;
use rand::distributions::Standard;
use rand::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Ant {
    location: Point<f32>,
    heading: f32,
    velocity: f32,
    carry_food: bool,
    max_x: f32,
    max_y: f32,
    steps: usize,
    rng_gen: ThreadRng,
}

const RANDOM_CHANCE: f32 = 0.05; // random chance to change direction independently
const CONST_STEP: usize = 25; //how many steps the ant will ignore the direction update
const MIN_VELOCITY: f32 = 1.1; // change it to any number you wish that bigger than 0.

impl Ant {
    pub fn new(x: f32, y: f32, max_x: f32, max_y: f32) -> Ant {
        let mut rng = rand::thread_rng();
        let head_base: f32 = StdRng::from_entropy().sample(Standard);
        return Ant {
            location: Point::new_from_vec(&vec![x, y]),
            heading: head_base * 2.0 * PI,
            carry_food: false,
            velocity: rng.gen::<f32>() + MIN_VELOCITY,
            max_x: max_x - 1.0,
            max_y: max_y - 1.0,
            steps: 0,
            rng_gen: rng,
        };
    }

    pub fn do_blind_step(&mut self) {
        // go forward without thinking
        let random: f32 = self.rng_gen.gen::<f32>();
        if random < RANDOM_CHANCE {
            self.set_heading(self.get_heading() + random);
        }

        //we check if the ant arrive the screen end
        let mut new_x = self.get_x() + self.get_heading().cos() * self.get_velocity();
        if new_x < 0.0 {
            new_x = 0.0;
            self.set_heading_other_side();
        }
        if new_x > self.max_x {
            new_x = self.max_x;
            self.set_heading_other_side();
        }

        let mut new_y = self.get_y() + self.get_heading().sin() * self.get_velocity();
        if new_y < 0.0 {
            new_y = 0.0;
            self.set_heading_other_side();
        }
        if new_y > self.max_y {
            new_y = self.max_y;
            self.set_heading_other_side();
        }

        self.steps += 1; // count steps, so we can use CONST_STEP

        self.set_loc(Point::new_from_vec(&vec![new_x, new_y]));
    }

    fn set_heading_other_side(&mut self) {
        self.set_heading(self.get_heading() + (PI / 1.0));
    }

    pub fn step_heading(&mut self, direction: f32) {
        if (direction != self.heading) && (self.steps % CONST_STEP == 0) {
            self.set_heading(direction); // I prefer that ant first change their direction, and only in the next step will move
        } else {
            self.do_blind_step();
        }
    }
    fn set_heading(&mut self, direction: f32) {
        self.heading = direction % (2.0 * PI);
    }
    fn set_loc(&mut self, loc: Point<f32>) {
        self.location = loc;
    }

    pub fn get_y(&self) -> &f32 {
        return self.location.get_value(2);
    }
    pub fn get_heading(&self) -> f32 {
        return self.heading.clone();
    }
    pub fn get_x(&self) -> &f32 {
        return self.location.get_value(1);
    }
    pub fn is_carry_food(&self) -> bool {
        return self.carry_food.clone();
    }
    pub fn get_velocity(&self) -> &f32 {
        return &self.velocity;
    }
    pub fn get_location(&self) -> Point<f32> {
        self.location.clone()
    }
    pub fn set_carry_food(&mut self, is_carry: bool) {
        self.steps = 0;
        self.carry_food = is_carry;
    }
}

#[cfg(test)]
mod tests {
    use super::Ant;
    #[test]
    fn constructor_x() {
        let a = Ant::new(1.5, 2.5, 45.0);
        assert_eq!(*a.get_x(), 1.5);
    }
    #[test]
    fn constructor_y() {
        let a = Ant::new(1.5, 2.5, 45.0);
        assert_eq!(*a.get_y(), 2.5);
    }
    #[test]
    fn constructor_heading() {
        let a = Ant::new(1.5, 2.5, 45.0);
        assert_eq!(*a.get_heading(), 45.0);
    }
    #[test]
    fn constructor_initilize_without_food() {
        let a = Ant::new(1.5, 2.5, 45.0);
        assert_eq!(*a.is_carry_food(), false);
    }
}
