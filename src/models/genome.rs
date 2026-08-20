#[derive(Debug)]
pub struct Genome {
    move_speed: f32,
    turn_speed: f32,
    vision_range: f32,
    arm_length: f32,
    // Additional fields can be added here
}

impl Genome {
    pub fn new(move_speed: f32, turn_speed: f32, vision_range: f32, arm_length: f32) -> Self {
        Genome {
            move_speed,
            turn_speed,
            vision_range,
            arm_length,
        }
    }

    // Additional methods can be added here
}