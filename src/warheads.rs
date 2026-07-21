pub struct Warhead {
    pub typename: String,
    pub strengthKt: i32,
}

// 1 megaton = 4-5 mile destructive radius
impl Warhead { // I almost forgot each country was supposed to have a vector of warheads.
               // Warheads will be targetable just like cities. When you blow up a countries silos 
               // it decreases their warhead amount
    pub fn new(&mut self, weapon_name: String, strengthKt: i32) {
        self.typename = weapon_name;
        self.strengthKt = strengthKt;
    }
    pub fn get_warhead_destruction_radius(&self) -> i32 {
        let destruction_radius:i32 = self.strengthKt * 5;
        return destruction_radius;
    }
    pub fn get_death_toll(population_density: i32, weapon: &Self) -> i32 {
        // Estimated deaths = Lethal area x population density
        let death_toll = weapon.strengthKt * population_density;
        return death_toll;
    }
    pub fn get_info(&self) -> Vec<String> {
        let mut info: Vec<String> = Vec::new();
        info.push(self.typename.clone());
        info.push(self.strengthKt.to_string());
        return info;
    }

}