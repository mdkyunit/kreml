use std::io;
use crate::world::{city_id, City};
use crate::world::city_id::{moscow, nullcity};
use crate::GameGlobal;
use crate::warheads::Warhead;

pub struct playerinput {
    pub target: (city_id, usize),
    pub weapon: Warhead,
    pub player_nation: city_id,
}
pub fn get_input(game: &GameGlobal) -> playerinput { // i hate this fucking function so much
    // command format "ACTION TARGET AMOUNT" ie
    // "LAUNCH LOS_ANGELES 20"
    let mut input = String::new();
    let mut player_target:city_id;
    //warhead hardcoded as 20kt generic bomb for now
    let debug_nuke:Warhead = Warhead {
        typename: "Debug Nuke".to_string(),
        strengthKt:20,
    };
    let mut player = playerinput {
        target: (nullcity, 0),
        player_nation:nullcity,
        weapon:debug_nuke,
    };
    loop {
        io::stdin().read_line(&mut input);
        let input_terms: Vec<&str> = input.split_whitespace().collect();
        if input_terms.len() > 3 || input_terms.len() < 3 {
            println!("Incorrect amount of arguments");
            continue;
        } else {
            for item in input_terms.iter() {
                println!("{}", item);
            }
            player.target = city_id_fromname(&input_terms[1].to_string(), &game);
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
fn city_id_fromname(input: &String, game: &GameGlobal) ->  (city_id, usize) {// so much pain to get this fuckass funtion to work.
    let user_inp: String = input.clone();
    for i in 0..game.active_countries[0].cities.len() {                                 //takes the length of the array of generated cities from its parent country
        if game.active_countries[0].cities[i].get_name().to_lowercase().as_str() == user_inp {//then compares the name of each city to the users input. Then returns the cities ID and index on the parent countries vector
            let city_target = game.active_countries[0].cities[i].get_id();
            return city_target;
        } else {
            continue;
        }
    }
    (nullcity, 10) //Nullcity is the default valid if the input is invalid or otherwise unable to be determined.
                   //This helps prevent the program from going batshit crazy
}