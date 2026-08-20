mod config;
mod models;
mod simulation;
mod world;

pub use config::Config;
pub use world::World;
pub use simulation::Simulation;

fn main() {
    println!("Hello, world!");
    let config = Config::default();

    Simulation::new(&config).run(&config);
}
