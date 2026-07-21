use std::io::{stdin, stdout, Write};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    event,
};
use crate::world::{city_id, City, Country, CountryID};
use crate::world::CountryID::USSR;
use crate::input::{get_input, playerinput};
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
    let mut game:GameGlobal = init_game();
    game_loop(&mut game);
}
fn game_loop(game: &mut GameGlobal) {
    let mut player:playerinput;
        player = get_input(&game);
        game.active_countries[0].cities[player.target.1].check_targeted(&player);
    //USSR_Country.cities[player.target.1].check_targeted(&player);

}
fn init_game() -> GameGlobal {
    let mut game:GameGlobal = GameGlobal {
        active_countries: Vec::new(),
    };
    let mut countries: Vec<Country> = Vec::new();
    let mut USSR_Country:Country = Country {
        id:USSR,
        name: "USSR".to_string(),
        population: 0,
        cities: Vec::new(),
        cities_count: 0,
    };
    USSR_Country.new(CountryID::USSR);
    countries = vec![USSR_Country];
    game.active_countries = countries;
    println!("Active city count {}", game.active_countries[0].cities.len());
    return game;
}