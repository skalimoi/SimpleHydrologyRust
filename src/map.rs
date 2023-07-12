use errorfunctions::RealErrorFunctions;
use nalgebra::Vector3;

use crate::world::Vec2;

#[derive(Default)]
pub struct Cell {
    pub height: f64,
    pub discharge: f64,
    pub momentum: Vec2,

    pub discharge_track: f64,
    pub momentum_track: Vec2,

    pub rootdensity: f64,
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub heightmap: Vec<Cell>,
}

impl Map {
    pub fn new(width: usize, height: usize, heightmap: Vec<u16>) -> Self {
        Self {
            width,
            height,
            heightmap: heightmap
                .into_iter()
                .map(|x| Cell {
                    height: x as f64 / 255.0,
                    ..Default::default()
                })
                .collect(),
        }
    }
    pub fn get(&self, pos: Vec2) -> Option<&Cell> {
        self.heightmap.get(self.width * pos.y as usize + pos.x as usize)
    }
    pub fn get_mut(&mut self, pos: Vec2) -> Option<&mut Cell> {
        self.heightmap.get_mut(self.width * pos.y as usize + pos.x as usize)
    }
    pub fn oob(&self, pos: Vec2) -> bool {
        pos.x < 0.0 || pos.y < 0.0 || pos.x >= self.width as f64 || pos.y >= self.height as f64
    }
    pub fn height(&self, pos: Vec2) -> f64 {
        if self.oob(pos) {
            return 0.0;
        }
        self.get(pos).map(|x| x.height).unwrap_or(0.0)
    }
    pub fn discharge(&self, pos: Vec2) -> f64 {
        if self.oob(pos) {
            return 0.0;
        }
        self.get(pos).map(|x| (0.4 * x.discharge as f64).erf() as f64).unwrap_or(0.0)
    }
    pub fn normal(&self, pos: Vec2) -> Vector3<f64> {
        let mut normal = Vector3::zeros();
        let scale = Vector3::new(1.0, 80.0, 1.0);

        if !self.oob(pos + Vec2::new(1.0, 1.0)) {
            normal += scale
                .component_mul(&Vector3::new(0.0, self.height(pos + Vec2::new(0.0, 1.0)) - self.height(pos), 1.0))
                .cross(&scale.component_mul(&Vector3::new(
                    1.0,
                    self.height(pos + Vec2::new(1.0, 0.0)) - self.height(pos),
                    0.0,
                )));
        }

        if !self.oob(pos + Vec2::new(-1.0, -1.0)) {
            normal += scale
                .component_mul(&Vector3::new(0.0, self.height(pos - Vec2::new(0.0, 1.0)) - self.height(pos), -1.0))
                .cross(&scale.component_mul(&Vector3::new(
                    -1.0,
                    self.height(pos - Vec2::new(1.0, 0.0)) - self.height(pos),
                    0.0,
                )));
        }

        //Two Alternative Planes (+X -> -Y) (-X -> +Y)
        if !self.oob(pos + Vec2::new(1.0, -1.0)) {
            normal += scale
                .component_mul(&Vector3::new(1.0, self.height(pos + Vec2::new(1.0, 0.0)) - self.height(pos), 0.0))
                .cross(&scale.component_mul(&Vector3::new(
                    0.0,
                    self.height(pos - Vec2::new(0.0, 1.0)) - self.height(pos),
                    -1.0,
                )));
        }

        if !self.oob(pos + Vec2::new(-1.0, 1.0)) {
            normal += scale
                .component_mul(&Vector3::new(-1.0, self.height(pos - Vec2::new(1.0, 0.0)) - self.height(pos), 0.0))
                .cross(&scale.component_mul(&Vector3::new(
                    0.0,
                    self.height(pos + Vec2::new(0.0, 1.0)) - self.height(pos),
                    1.0,
                )));
        }
        if normal.magnitude() > 0.0 {
            normal = normal.normalize();
        }
        normal
    }
}
