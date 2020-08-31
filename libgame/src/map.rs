use bracket_pathfinding::prelude::*;
use gdnative::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Cell {
    Empty,
    Occupied(Vector2),
}

struct MapCells {
    inner: Vec<Cell>,
}

impl MapCells {
    fn with_capacity(cap: usize) -> Self {
        Self {
            inner: vec![Cell::Empty; cap],
        }
    }

    fn insert(&mut self, index: usize, coord: Vector2) {
        self.inner.insert(index, Cell::Occupied(coord));
    }

    fn get(&self, index: usize) -> Option<&Cell> {
        self.inner.get(index)
    }
}

pub struct Map {
    cells: MapCells,
    map_size: Rect2,
    tile_size: Vector2,
}

impl Map {
    pub fn new(coords: Vec<Vector2>, map_size: Rect2, tile_size: Vector2) -> Self {
        let cap = map_size.width() * map_size.height();

        let mut map = Self {
            cells: MapCells::with_capacity(cap as usize),
            map_size,
            tile_size,
        };

        coords.iter().for_each(|&coord| {
            let index = map.coords_to_index(coord);
            map.cells.insert(index, coord);
        });

        map
    }

    fn coords_to_index(&self, coords: Vector2) -> usize {
        let x = coords.x - self.map_size.min_x();
        let y = coords.y - self.map_size.min_y();
        let index = x + y * self.map_size.width();
        index as usize
    }

    fn index_to_coords(&self, index: usize) -> Option<Vector2> {
        match self.cells.get(index) {
            Some(Cell::Occupied(coords)) => Some(*coords),
            _ => None,
        }
    }

    fn local_pos_to_index(&self, pos: Vector2) -> usize {
        // Local pos to coords
        let x = (pos.x / self.tile_size.x).floor() - self.map_size.min_x();
        let y = (pos.y / self.tile_size.y).floor() - self.map_size.min_y();

        let index = x + y * self.map_size.width();
        index as usize
    }

    fn index_to_local_pos(&self, index: usize) -> Option<Vector2> {
        match self.cells.get(index) {
            Some(Cell::Occupied(coords)) => {
                let x = coords.x * self.tile_size.x + self.tile_size.x / 2.;
                let y = coords.y * self.tile_size.y + self.tile_size.y / 2.;

                Some(Vector2::new(x, y))
            }
            _ => None,
        }
    }

    /// Is the given tile a valid exit (if yes, return index)
    fn valid_exit(&self, coords: Vector2) -> Option<usize> {
        let index = self.coords_to_index(coords);

        match self.cells.get(index) {
            Some(Cell::Occupied(_)) => Some(index),
            _ => None,
        }
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();

        let location = match self.index_to_coords(idx) {
            Some(l) => l,
            None => return exits,
        };

        let cost_cardinal = 1.0;
        let cost_diagonal = 1.4;

        // Cardinal directions
        if let Some(idx) = self.valid_exit(location + Vector2::new(-1., 0.)) {
            exits.push((idx, cost_cardinal));
        }

        if let Some(idx) = self.valid_exit(location + Vector2::new(1., 0.)) {
            exits.push((idx, cost_cardinal));
        }

        if let Some(idx) = self.valid_exit(location + Vector2::new(0., -1.)) {
            exits.push((idx, cost_cardinal));
        }

        if let Some(idx) = self.valid_exit(location + Vector2::new(0., 1.)) {
            exits.push((idx, cost_cardinal));
        }

        // Diagonals
        if let Some(idx) = self.valid_exit(location + Vector2::new(-1., 1.)) {
            exits.push((idx, cost_diagonal));
        }

        if let Some(idx) = self.valid_exit(location + Vector2::new(1., 1.)) {
            exits.push((idx, cost_diagonal));
        }

        if let Some(idx) = self.valid_exit(location + Vector2::new(1., -1.)) {
            exits.push((idx, cost_diagonal));
        }

        if let Some(idx) = self.valid_exit(location + Vector2::new(-1., -1.)) {
            exits.push((idx, cost_diagonal));
        }

        exits
    }

    fn get_pathing_distance(&self, idx_1: usize, idx_2: usize) -> f32 {
        let coords_1 = self.index_to_coords(idx_1).unwrap();
        let coords_2 = self.index_to_coords(idx_2).unwrap();

        DistanceAlg::PythagorasSquared.distance2d(
            Point::new(coords_1.x as i32, coords_1.y as i32),
            Point::new(coords_2.x as i32, coords_2.y as i32),
        )
    }
}
