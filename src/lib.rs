//! Simulation Trait Definitions for WASM

pub mod ant;
pub mod conway;

/// A trait defining what a simulation must represent to be renderable
pub trait Simulation {
    /// Step the world forward
    fn step(&mut self);
    /// Render the world
    fn render(&self) -> Vec<usize>;
    /// Returns how many steps have gone by
    fn steps(&self) -> usize;
}
