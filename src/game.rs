/*
Create mechanic for targeting country using user input
Create other game logic like calculating fatalities
Take input from input.rs functions to process into said input
related functions.
In the future this file will contain all the logic necessary for the game loop.
*/
use crate::GameGlobal;
use crate::input::{get_input, PlayerInput};
use crate::world::{Country, CountryID};
use crate::world::CountryID::USSR;

pub fn game_loop(game: &mut GameGlobal) {
    let mut player: PlayerInput;
    player = get_input(&game);
    game.active_countries[0].cities[player.target.1].check_targeted(&player);
    //USSR_Country.cities[player.target.1].check_targeted(&player);

}
pub fn init_game() -> GameGlobal {
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