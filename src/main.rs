use std::io::{stdin, stdout, Write};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    event,
};
use crate::game::{game_loop, init_game};
use crate::world::{CityID, City, Country, CountryID};
use crate::world::CountryID::USSR;
use crate::input::{get_input, PlayerInput};
mod display;
mod input;
mod game;
pub mod warheads;
mod world;
pub struct GameGlobal {
    active_countries: Vec<Country>,
}
/*
Code here is a mess due to experimentation.
will be using main to initiate display, game logic as well as 
initialize countries and cities.
 */
fn main() {
    let mut game:GameGlobal = init_game(); //initialize game countries.
    game_loop(&mut game); //begins game loop
}