use crate::world::*;

pub struct Drop {
    age: i32,
    pos: Vec2,
    speed: Vec2,

    volume: f64,
    sediment: f64,

    max_age: i32,
    min_vol: f64,
    evap_rate: f64,
    deposition_rate: f64,
    entrainment: f64,
    gravity: f64,
    momentum_transfer: f64,
}

impl Drop {
    pub fn new(pos: Vec2) -> Self {
        Self {
            age: 0,
            pos,
            speed: Vec2::new(0.0, 0.0),
            volume: 1.0,
            sediment: 0.0,

            max_age: 500,
            min_vol: 0.01,
            evap_rate: 0.001,
            deposition_rate: 0.1,
            entrainment: 10.0,
            gravity: 1.0,
            momentum_transfer: 1.0,
        }
    }
    pub fn decend(&mut self, world: &mut World) -> bool {
        let prev_pos = self.pos;
        let normal_vector = world.map.normal(prev_pos);
        let Some(cell) = world.map.get_mut(prev_pos) else {return false};

        if self.age > self.max_age || self.volume < self.min_vol {
            cell.height += self.sediment;
            return false;
        };

        let eff_d = (self.deposition_rate * (1.0 - cell.rootdensity)).max(0.0);

        self.speed += self.gravity * Vec2::new(normal_vector.x, normal_vector.z) / self.volume;
        if cell.momentum.magnitude() > 0.0 && self.speed.magnitude() > 0.0 {
            self.speed += self.momentum_transfer * cell.momentum.normalize().dot(&self.speed.normalize())
                / (self.volume + cell.discharge)
                * cell.momentum;
        };

        if self.speed.magnitude() > 0.0 {
            self.speed = 2.0f64.sqrt() * self.speed.normalize();
        };

        self.pos += self.speed;

        cell.discharge_track += self.volume;
        cell.momentum_track += self.volume * self.speed;

        let Some(cell) = world.map.get(prev_pos) else {return false};

        let h2 = if world.map.oob(self.pos) {
            cell.height - 0.002
        } else {
            world.map.height(self.pos)
        };

        let c_eq = ((1.0 + self.entrainment * world.map.discharge(prev_pos)) * (cell.height - h2)).max(0.0);
        let cdiff = c_eq - self.sediment;

        self.sediment += eff_d * cdiff;
        let Some(cell) = world.map.get_mut(prev_pos) else {return false};
        cell.height -= eff_d * cdiff;

        self.sediment /= 1.0 - self.evap_rate;
        self.volume *= 1.0 - self.evap_rate;

        if world.map.oob(self.pos) {
            self.volume = 0.0;
            return false;
        };

        world.cascade(self.pos);
        self.age += 1;
        true
    }
}
