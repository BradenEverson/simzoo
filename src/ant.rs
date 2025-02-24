//! Implementation of Langton's Ant

pub struct LangtonAnt {
    width: usize,
    height: usize,
    ant: (usize, usize),
    grid: Vec<AntState>,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum AntState {
    White = 0,
    Black = 1,
    Ant = 2,
}

impl LangtonAnt {
    pub fn with_dims(width: usize, height: usize) -> Self {
        let ant = (width / 2, height / 2);
        let mut grid = vec![AntState::White; width * height];

        grid[ant.0 + ant.1 * width] = AntState::Ant;

        Self {
            width,
            height,
            ant,
            grid,
        }
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
        self.ant = (width / 2, height / 2);

        self.grid = vec![AntState::White; width * height];
        self.grid[self.ant.0 + self.ant.1 * width] = AntState::Ant;
    }
}
