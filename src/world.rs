use rand::RngExt;
use crate::Config;
use crate::models::Food;


#[derive(Debug)]
pub struct World {
    width: f32,
    height: f32,
    food_capacity: usize,
    food_spawn_per_tick: usize,
    food_energy: f32,
    pub foods: Vec<Food>,
    next_food_id: u64,
    // Additional fields can be added here
}

impl World {
    pub fn new(config: &Config) -> Self {
        World {
            width: config.world_width,
            height: config.world_height,
            food_capacity: config.food_capacity,
            food_spawn_per_tick: config.food_spawn_per_tick,
            food_energy: config.food_energy,
            foods: Vec::new(),
            next_food_id: 0,
        }
    }

    pub fn populate_initial_food(&mut self, initial_food: usize) {
        let mut rng = rand::rng();
        for _ in 0..initial_food {
            let food = Food::new(self.next_food_id, self.width, self.height, self.food_energy);
            self.foods.push(food);
            self.next_food_id += 1;
        }
    }

    pub fn spawn_food(&mut self) {
        for _ in 0..self.food_spawn_per_tick {
            if self.foods.len() < self.food_capacity {
                let food = Food::new(self.next_food_id, self.width, self.height, self.food_energy);
                self.foods.push(food);
                self.next_food_id += 1;
            }
        }
    }
    // Additional methods can be added here
}