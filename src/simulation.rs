use crate::Config;
use crate::World;
use crate::models::Genome;
use crate::models::Agent;


pub struct Simulation {
    initial_agents: usize,
    initial_energy: f32,
    max_energy: f32,
    base_cost: f32,
    max_tick: u32,
    stats_interval: u32,
}

impl Simulation {
    pub fn new(config: &Config) -> Self {
        Simulation {
            initial_agents: config.initial_agents,
            initial_energy: config.initial_energy,
            max_energy: config.max_energy,
            base_cost: config.base_cost,
            max_tick: config.max_tick,
            stats_interval: config.stats_interval,
        }
    }
    
    pub fn run(&self, config: &Config) {
        // Initialize the world
        let mut world = World::new(config);
        
        // Populate initial food
        world.populate_initial_food(config.initial_food);

        // Initialize agents
        for id in 0..self.initial_agents as u64 {
            let genome = Genome::new(
                config.move_speed_config.default,
                config.turn_speed_config.default,
                config.vision_range_config.default,
                config.arm_length_config.default,
            );
            
            let agent = Agent::new(
                id,
                config.world_width,
                config.world_height,
                self.initial_energy,
                self.max_energy,
                config.max_age,
                0,
                genome,
            );
        }

        
        // Simulation loop
        for tick in 0..self.max_tick {
            // Spawn food each tick
            world.spawn_food();
            
            // Additional simulation logic can be added here
            
            // Print stats at intervals
            if tick % self.stats_interval == 0 {
                println!("Tick: {}, Food count: {}", tick, world.foods.len());
            }
        }
    }

    
}