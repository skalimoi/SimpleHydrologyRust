use nalgebra::Vector2;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::map::*;
use crate::water::*;

pub type Vec2 = Vector2<f64>;

pub struct World {
    pub seed: u64,
    pub map: Map,

    pub lrate: f64,
    pub discharge_thresh: f64,
    pub maxdiff: f64,
    pub settling: f64,

    rng: ChaCha8Rng,
}

impl World {
    pub fn new(heightmap: Vec<u16>, width: usize, height: usize, seed: u64) -> Self {
        Self {
            seed,
            map: Map::new(width, height, heightmap),
            lrate: 0.1,
            discharge_thresh: 0.0,
            maxdiff: 0.01,
            settling: 0.8,
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }
    pub fn erode(&mut self, cycles: usize) {
        self.map.heightmap.iter_mut().for_each(|cell| {
            cell.discharge_track = 0.0;
            cell.momentum_track = Vec2::zeros();
        });
        for _ in 0..cycles {
            let pos = Vec2::new(self.rng.gen_range(0..self.map.width) as f64, self.rng.gen_range(0..self.map.height) as f64);
            if self.map.height(pos) < 0.1 {
                continue;
            }
            let mut drop = Drop::new(pos);
            while drop.decend(self) {}
        }
        self.map.heightmap.iter_mut().for_each(|cell| {
            cell.discharge = (1.0 - self.lrate) * cell.discharge + self.lrate * cell.discharge_track;
            cell.momentum = (1.0 - self.lrate) * cell.momentum + self.lrate * cell.momentum_track;
        });
    }
    pub fn cascade(&mut self, prev_pos: Vec2) {
        let mut neighbors = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                let offset = Vec2::new(x as f64, y as f64);
                let npos = prev_pos + offset;
                if self.map.oob(npos) || npos == prev_pos {
                    continue;
                }
                neighbors.push((npos, self.map.height(npos), offset.magnitude()))
            }
        }
        neighbors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        for neighbor in neighbors {
            let (npos, height, distance) = neighbor;

            let diff = self.map.height(prev_pos) - height;
            if diff == 0.0 {
                continue;
            }

            let excess = if height > 0.1 {
                diff.abs() - distance * self.maxdiff
            } else {
                diff.abs()
            };

            if excess <= 0.0 {
                continue;
            }

            let transfer = self.settling * excess / 2.0;

            if diff > 0.0 {
                self.map.get_mut(prev_pos).unwrap().height -= transfer;
                self.map.get_mut(npos).unwrap().height += transfer;
            } else {
                self.map.get_mut(prev_pos).unwrap().height += transfer;
                self.map.get_mut(npos).unwrap().height -= transfer;
            }
        }
    }
}
