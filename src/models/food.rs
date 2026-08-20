use rand::RngExt;

#[derive(Debug)]
pub struct Food {
    id: u64,
    position: (f32, f32),
    energy: f32,
    // Additional fields can be added here
}

impl Food {
    pub fn new(id: u64, width: f32, height: f32, energy: f32) -> Self {
        let mut rng = rand::rng();
        let position = (rng.random_range(0.0..width), rng.random_range(0.0..height)); // Assuming world dimensions
        Food {
            id,
            position,
            energy,
        }
    }
    // Additional methods can be added here
}