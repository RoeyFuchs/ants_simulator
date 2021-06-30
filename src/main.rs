mod ant;
mod configuration;
mod env;
mod utils;

use std::process;

use ggez;
use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self};
use ggez::mint;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use std::env as std_env;

struct MainState {
    environment: env::Env,
    ant_img: graphics::Image,
    ant_carry_img: graphics::Image,
    food_img: graphics::Image,
    home_img: graphics::Image,
}
impl MainState {
    fn new(ctx: &mut Context, env: env::Env) -> GameResult<MainState> {
        let ant_img = graphics::Image::new(ctx, configuration::ANT_IMG)?;
        let ant_carry_img = graphics::Image::new(ctx, configuration::ANT_CARRY_IMG)?;
        let food_img = graphics::Image::new(ctx, configuration::FOOD_IMG)?;
        let home_img = graphics::Image::new(ctx, configuration::HOME_IMG)?;
        Ok(MainState {
            environment: env,
            ant_img: ant_img,
            ant_carry_img: ant_carry_img,
            food_img: food_img,
            home_img: home_img,
        })
    }
}
impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.environment.do_step();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, ggez::graphics::Color::WHITE); //clearing the screen. we cannot change the location of what we already drew, so we need to re-draw everything :(
        {
            for a in self.environment.get_ants() {
                let ant_loc_to_draw = &a.get_location();

                let x = ant_loc_to_draw.get_value(1).clone();
                let y = ant_loc_to_draw.get_value(2).clone();
                let heading = a.get_heading();
                let loc: mint::Point2<f32> = mint::Point2 { x, y };

                let drawparams = graphics::DrawParam::new().dest(loc).rotation(heading);
                if a.is_carry_food() {
                    graphics::draw(ctx, &self.ant_carry_img, drawparams)?;
                } else {
                    graphics::draw(ctx, &self.ant_img, drawparams)?;
                }
            }
            for food in self.environment.get_food() {
                let food_loc_to_draw = utils::transform_obj(food);
                let loc: mint::Point2<f32> = mint::Point2 {
                    x: food_loc_to_draw.get_value(1).clone(),
                    y: food_loc_to_draw.get_value(2).clone(),
                };
                let drawparams = graphics::DrawParam::new().dest(loc);
                graphics::draw(ctx, &self.food_img, drawparams)?;
            }

            let loc: mint::Point2<f32> = mint::Point2 {
                x: utils::transform_obj(self.environment.get_home())
                    .get_value(1)
                    .clone(),
                y: utils::transform_obj(self.environment.get_home())
                    .get_value(2)
                    .clone(),
            };
            let drawparams = graphics::DrawParam::new().dest(loc);
            graphics::draw(ctx, &self.home_img, drawparams)?;
        }
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}
fn main() -> GameResult {
    let resource_dir = configuration::RESOURCES_DIR;

    let cb = ContextBuilder::new("ants simulator", "Roey")
        .window_setup(
            conf::WindowSetup::default()
                .title("ants simulator")
                .vsync(false),
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(configuration::WIDTH, configuration::HIGHT)
                .resizable(true),
        )
        .add_resource_path(resource_dir);

    //handle the argument of json file
    let args: Vec<String> = std_env::args().collect();
    if args.len() < 2 {
        eprintln!("Please add path to the json file as argument");
        process::exit(1);
    }
    let env = utils::read_json(&args[1]);

    let (mut ctx, events_loop) = cb.build()?;
    let game = MainState::new(&mut ctx, env)?;
    event::run(ctx, events_loop, game);
}
