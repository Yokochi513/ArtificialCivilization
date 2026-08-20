#[derive(Debug)]
pub struct Config {
    // simulation.rs
    pub initial_agents: usize,
    pub initial_energy: f32,
    pub max_age: u32,
    pub max_energy: f32,
    pub base_cost: f32,
    pub max_tick: u32,
    pub stats_interval: u32,

    // world.rs
    pub world_width: f32,
    pub world_height: f32,
    pub initial_food: usize,
    pub food_spawn_per_tick: usize,
    pub food_capacity: usize,
    pub food_energy: f32,

    // genome.rs
    pub move_speed_config: GenomeConfig,
    pub turn_speed_config: GenomeConfig,
    pub vision_range_config: GenomeConfig,
    pub arm_length_config: GenomeConfig,
}

#[derive(Debug)]
pub  struct GenomeConfig {
    pub default: f32,
    pub min: f32,
    pub max: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            initial_agents: 50,
            initial_energy: 100.0,
            max_age: 1000,
            max_energy: 200.0,
            base_cost: 0.5,
            max_tick: 10000,
            stats_interval: 100,

            world_width: 1000.0,
            world_height: 800.0,
            initial_food: 1600,
            food_spawn_per_tick: 50,
            food_capacity: 4000,
            food_energy: 30.0,

            move_speed_config: GenomeConfig { default: 1.0, min: 0.1, max: 3.0 },
            turn_speed_config: GenomeConfig { default: 0.5, min: 0.05, max: 3.14 },
            vision_range_config: GenomeConfig { default: 5.0, min: 1.0, max: 20.0 },
            arm_length_config: GenomeConfig { default: 0.5, min: 0.1, max: 2.0 },
        }
    }
}