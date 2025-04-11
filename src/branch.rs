use nalgebra::Rotation2;
use rand::Rng;

use crate::{limits::Limits, types::Vector};

const GROWTH_FACTOR: f64 = 5.0;

#[derive(Debug)]
pub struct Branch {
    pub vector: Vector,
    pub children: Vec<Branch>,
    pub limits: Limits,
    pub depth: usize,
    pub energy_need: f64,
    pub random_rotation_direction: f64,
}

impl Default for Branch {
    fn default() -> Self {
        let limits = Limits::default();
        let vector = Vector::new(0.1, 1.0);
        let energy_need = limits.length / GROWTH_FACTOR;

        Self {
            vector,
            children: Vec::new(),
            limits,
            depth: 0,
            energy_need,
            random_rotation_direction: 0.01,
        }
    }
}

impl Branch {
    pub fn update(&mut self, time_delta: f64, energy: f64) {
        if self.should_branch(time_delta) {
            self.new_branch();
        }
        self.update_energy_need();

        let (growth_energy, child_energies) = self.distrubute_energy(energy);

        self.grow(time_delta, growth_energy);
        self.random_rotation();

        self.children
            .iter_mut()
            .zip(child_energies)
            .for_each(|(child, child_energy)| {
                child.update(time_delta, child_energy);
            });
    }

    fn update_energy_need(&mut self) {
        let new_self_need = (self.limits.length - self.vector.norm()).max(0.0) / GROWTH_FACTOR;

        self.energy_need = new_self_need
            + self
                .children
                .iter()
                .map(|child| child.energy_need)
                .sum::<f64>();
    }

    fn random_rotation(&mut self) {
        let mut rng = rand::rng();
        if rng.random_bool(0.1) {
            self.random_rotation_direction *= -1.0
        };

        let magnitude = (self.limits.length - self.vector.norm()).min(2.0) * 0.1;

        let rotation = Rotation2::new(magnitude * self.random_rotation_direction);
        self.vector = rotation * self.vector;
    }

    pub fn grow(&mut self, time_delta: f64, energy: f64) {
        if energy <= 0.0 {
            return;
        }

        let new_norm = self.vector.norm() + (energy * time_delta * GROWTH_FACTOR);
        let new_vector = self.vector.normalize() * new_norm;
        self.vector = new_vector;
    }

    fn distrubute_energy(&mut self, energy: f64) -> (f64, Vec<f64>) {
        let child_needs = self
            .children
            .iter()
            .map(|child| child.energy_need)
            .collect::<Vec<f64>>();

        let growth_need = self.energy_need - child_needs.iter().sum::<f64>();

        if self.energy_need <= 0.0 {
            let child_count = self.children.len().max(1) as f64;
            return (
                energy / child_count,
                vec![energy / child_count; self.children.len()],
            );
        }

        let growth_energy = energy * (growth_need / self.energy_need);
        let child_energies = child_needs
            .iter()
            .map(|need| energy * (need / self.energy_need))
            .collect::<Vec<f64>>();

        (growth_energy, child_energies)
    }

    fn new_branch(&mut self) {
        let mut rng = rand::rng();
        let random_rotation = Rotation2::new(rng.random_range(self.limits.angle.clone()));

        self.children.push(Branch {
            vector: random_rotation * self.vector.normalize(),
            limits: Limits {
                angle: self.limits.angle.clone(),
                length: self.limits.length * rng.random_range(0.8..1.0),
                child_count: self.limits.child_count,
                depth: self.limits.depth,
            },
            depth: self.depth + 1,
            ..Default::default()
        });
    }

    fn should_branch(&self, time_delta: f64) -> bool {
        if self.children.len() >= self.limits.child_count || self.depth >= self.limits.depth {
            return false;
        }

        rand::random::<f64>() < ((self.vector.norm() / self.limits.length) * time_delta)
    }
}
