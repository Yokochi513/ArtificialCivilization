use rand::RngExt;

use crate::models::Genome;

#[derive(Debug)]
pub struct Agent {
    id: u64,
    position: (f32, f32),
    heading: f32,
    energy: f32,
    max_energy: f32,
    age: u32,
    max_age: u32,
    generation: u32,
    genome: Genome,
    // Additional fields can be added here
}

impl Agent {
    pub fn new(id: u64, weight: f32, height: f32, energy: f32, max_energy: f32, max_age: u32, generation: u32, genome: Genome) -> Self {
        let mut rng = rand::rng();
        let position = (rng.random_range(0.0..weight), rng.random_range(0.0..height)); // Assuming world dimensions
        let heading = rng.random_range(0.0..std::f32::consts::TAU); // Random heading in radians
        Agent {
            id,
            position,
            heading,
            energy,
            max_energy,
            age: 0,
            max_age,
            generation,
            genome,
        }
    }

    // Additional methods can be added here
}