//! Simulation trait implementation for Conway's Game of Life

use wasm_bindgen::prelude::wasm_bindgen;

use crate::Simulation;

/// Conway's Game of Life
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Conway {
    width: usize,
    height: usize,
    area: Vec<bool>,
    steps: usize,
}

#[wasm_bindgen]
impl Conway {
    /// Creates a new simulation with set dimensions
    pub fn with_dims(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            area: vec![false; width * height],
            steps: 0,
        }
    }

    pub fn clear(&mut self) {
        self.area = vec![false; self.width * self.height]
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.area = vec![false; width * height];
    }

    pub fn step(&mut self) {
        let mut next_area = self.area.clone();

        for idx in 0..self.area.len() {
            let x = idx % self.width;
            let y = idx / self.width;

            let mut living_neighbors = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let nx = x as isize + dx;
                    let ny = y as isize + dy;

                    if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                        let neighbor_idx = (ny as usize) * self.width + (nx as usize);
                        if self.area[neighbor_idx] {
                            living_neighbors += 1;
                        }
                    }
                }
            }

            next_area[idx] = match (self.area[idx], living_neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }

        self.area = next_area;
        self.steps += 1;
    }

    pub fn set(&mut self, idx: usize) {
        self.area[idx] = !self.area[idx]
    }

    pub fn render(&self) -> Vec<usize> {
        self.area
            .iter()
            .map(|val| if *val { 1 } else { 0 })
            .collect()
    }

    pub fn steps(&self) -> usize {
        self.steps
    }
}

impl Simulation for Conway {
    fn step(&mut self) {
        Conway::step(self);
    }

    fn render(&self) -> Vec<usize> {
        Conway::render(self)
    }

    fn steps(&self) -> usize {
        Conway::steps(self)
    }
}
