use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};

use super::tile::{Tile, standard_tile_set};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Wall {
    tiles: Vec<Tile>,
    drawn: Vec<Tile>,
    initial_count: usize,
}

impl Wall {
    pub fn new(seed: u64) -> Self {
        let mut tiles = standard_tile_set();
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        tiles.shuffle(&mut rng);
        let initial_count = tiles.len();
        Self {
            tiles,
            drawn: Vec::new(),
            initial_count,
        }
    }

    pub fn draw(&mut self, count: usize) -> Vec<Tile> {
        let actual = count.min(self.tiles.len());
        let drawn_tiles: Vec<Tile> = self.tiles.drain(..actual).collect();
        self.drawn.extend(drawn_tiles.iter().cloned());
        drawn_tiles
    }

    pub fn remaining(&self) -> usize {
        self.tiles.len()
    }

    pub fn drawn_count(&self) -> usize {
        self.drawn.len()
    }

    pub fn initial_count(&self) -> usize {
        self.initial_count
    }

    /// Remove specific tiles by ID (for tile destruction/manipulation)
    pub fn remove_by_ids(&mut self, ids: &[u32]) -> Vec<Tile> {
        let id_set: std::collections::HashSet<u32> = ids.iter().copied().collect();
        let mut removed = Vec::new();
        self.tiles.retain(|t| {
            if id_set.contains(&t.id) {
                removed.push(*t);
                false
            } else {
                true
            }
        });
        removed
    }

    /// Add tiles back to the wall (for certain gadgets/effects)
    pub fn add_tiles(&mut self, mut tiles: Vec<Tile>) {
        self.tiles.append(&mut tiles);
    }

    /// Get reference to remaining tiles (for peeking/previewing)
    pub fn peek_remaining(&self) -> &[Tile] {
        &self.tiles
    }

    /// Get reference to already drawn tiles
    pub fn peek_drawn(&self) -> &[Tile] {
        &self.drawn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wall_creation() {
        let wall = Wall::new(42);
        assert_eq!(wall.remaining(), 136);
        assert_eq!(wall.drawn_count(), 0);
    }

    #[test]
    fn test_wall_draw() {
        let mut wall = Wall::new(42);
        let tiles = wall.draw(14);
        assert_eq!(tiles.len(), 14);
        assert_eq!(wall.remaining(), 122);
        assert_eq!(wall.drawn_count(), 14);
    }

    #[test]
    fn test_wall_draw_more_than_remaining() {
        let mut wall = Wall::new(42);
        let tiles = wall.draw(200);
        assert_eq!(tiles.len(), 136);
        assert_eq!(wall.remaining(), 0);
    }

    #[test]
    fn test_deterministic_shuffle() {
        let mut wall1 = Wall::new(12345);
        let mut wall2 = Wall::new(12345);
        let tiles1 = wall1.draw(10);
        let tiles2 = wall2.draw(10);
        assert_eq!(tiles1, tiles2);
    }

    #[test]
    fn test_different_seeds_different_order() {
        let mut wall1 = Wall::new(12345);
        let mut wall2 = Wall::new(54321);
        let tiles1 = wall1.draw(10);
        let tiles2 = wall2.draw(10);
        assert_ne!(tiles1, tiles2);
    }
}
