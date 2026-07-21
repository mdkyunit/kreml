use std::io;
use crate::world::{CityID};
use crate::world::CityID::{NullCity};
use crate::GameGlobal;
use crate::warheads::Warhead;

pub struct PlayerInput {
    pub target: (CityID, usize),
    pub weapon: Warhead,
    //pub player_nation: city_id, //will be implemented when logic to choose played nation is added.
}
pub fn get_input(game: &GameGlobal) -> PlayerInput { // I hate this fucking function so much
    // command format "ACTION TARGET AMOUNT" ie
    // "LAUNCH LOS_ANGELES 20"
    // When defense is added, "Include Duds" will be a factor.
    let mut input = String::new();
    //warhead hardcoded as 20kt generic bomb for now
    let debug_nuke:Warhead = Warhead {
        typename: "Debug Nuke".to_string(),
        strength_kt:20,
    };
    let mut player = PlayerInput {
        target: (NullCity, 0),
       // player_nation:nullcity,
        weapon:debug_nuke,
    };
    loop {
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input_terms: Vec<&str> = input.split_whitespace().collect();
        if input_terms.len() > 3 || input_terms.len() < 3 {
            println!("Incorrect amount of arguments");
            continue;
        } else {
            for item in input_terms.iter() {
                println!("{}", item);
            }
            player.target = city_id_from_name(&input_terms[1].to_string(), &game);
            if player.target.1 >= game.active_countries[0].cities_count as usize {
                println!("bad input");
                input.clear();
                continue;
            } else {
                return player;
            }
        }
    }
}
fn city_id_from_name(input: &String, game: &GameGlobal) ->  (CityID, usize) {// so much pain to get this fuckass function to work.
    let user_inp: String = input.clone();
    for i in 0..game.active_countries[0].cities.len() {                                 //takes the length of the array of generated cities from its parent country
        if game.active_countries[0].cities[i].get_name().to_lowercase().as_str() == user_inp {//then compares the name of each city to the users input. Then returns the cities ID and index on the parent countries vector
            let city_target = game.active_countries[0].cities[i].get_id();
            return city_target;
        } else {
            continue;
        }
    }
    (NullCity, 10) //Null city is the default valid if the input is invalid or otherwise unable to be determined.
                   //This helps prevent the program from going batshit crazy
}