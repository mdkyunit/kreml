//use crate::cities::{city, city_id};

use crate::world::city_id::{buttville, kiev, leningrad, moscow, novosibirsk, nullcity};
use crate::GameGlobal;
use crate::input::playerinput;
use crate::warheads::Warhead;

pub enum CountryID {
    USSR = 0,
    USA = 1,
}
pub mod city_base_population {
    pub const MOSCOW_POPULATION: i32 = 8960000;
    pub const LENINGRAD_POPULATION: i32 = 5020000;
    pub const KIEV_POPULATION: i32 = 2580000;
    pub const NOVOSIBIRSK_POPULATION: i32 = 1400000;
}
#[derive(Clone, Copy)]
pub enum city_id {
    nullcity = 10,
    moscow = 0, // 8.96 million
    leningrad = 1, // 5.02 million
    kiev = 2, // 2.58 million
    novosibirsk = 3, // 1.4 million
    buttville = 4,
}
pub struct Country {
    pub id: CountryID,
    pub name: String,
    pub population: i32,
    pub cities: Vec<City>,
    pub cities_count: i32,
}
pub struct City {
    pub id: city_id,
    pub population: i32,
    pub name: String,
    pub is_destroyed: bool,
}

impl Country {
    pub fn new(&mut self, country_id: CountryID) {
        match country_id {
            CountryID::USSR => {
                self.cities = generate_cities_ussr();
                self.cities_count = self.cities.len() as i32;
            }
            CountryID::USA => {
               // self.cities = generate_cities_usa();
            },
        }
        self.id = country_id;
        self.name = "Default".to_string();
        for i in 0..self.cities.len() {
            self.population += self.cities[i].population;
        }
        //println!("Country made with population {}", self.population);
    }
}
impl City {
    pub fn check_targeted(&mut self ,player_input: &playerinput){
        let input = &player_input.target;
        if matches!(&self.id, input) {
            self.attacked(&player_input.weapon)
        }
    }
    pub fn attacked(&mut self, warheads: &Warhead) {
        if self.is_destroyed != true {
            print!("{} Population before attack: {}\n", self.name, self.population);
            self.population = self.population - (warheads.strengthKt * 500000);
            if self.population <= 0 {
                println!("{} was leveled!\n", self.name);
                self.is_destroyed = true;
            } else {
                println!("{} was attacked! new population: {}\n", self.name, self.population);
            }
        } else {
            println!("{} Is already leveled!", self.name);
        }
    }
    pub fn get_id(&self) -> (city_id, usize) {
        let city_id: city_id = self.id;
        let city_index: usize = city_id as usize;
        (city_id, city_index)
    }
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
}

/*pub fn generate_cities_usa() -> Vec<City> {

}*/
pub fn generate_cities_ussr() -> Vec<City> { //might make this a generic generate cities function.
                                             //or make it a method for the Country struct.
                                             //this will be the only area where cities need to be hardcoded in
                                             //procedures will automatically produce the logic needed for targeting etc.
    let mut gen_cities: Vec<City> = vec![
        City {
            id: moscow,
            population: city_base_population::MOSCOW_POPULATION,
            name: String::from("Moscow"),
            is_destroyed: false,
        },
        City {
            id: leningrad,
            population: city_base_population::LENINGRAD_POPULATION,
            name: String::from("Leningrad"),
            is_destroyed: false,
        },
        City {
            id: Kyiv,
            population: city_base_population::KYIV_POPULATION,
            name: String::from("Kyiv"),
            is_destroyed: false,
        },
        City {
            id: novosibirsk,
            population: city_base_population::NOVOSIBIRSK_POPULATION,
            name: String::from("Novosibirsk"),
            is_destroyed: false,

        },
        City {
            id: buttville,
            population:69,
            name: "Buttville".to_string(), // i know im very mature.
            is_destroyed: false,

        }, //buttville is first example of program working procedurally in order to process new cities being added.
        City {
            id: nullcity,
            population: -1,
            name: "Null City".to_string(),
            is_destroyed: false,
        },
    ];
    for item in 0..gen_cities.len() {
        println!("Generated city: {}", gen_cities[item].name); // for debug purposes
    }
    return gen_cities;
}