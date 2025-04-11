use crate::{branch::Branch, limits::Limits, types::Vector};
use colorgrad::LinearGradient;

#[derive(Debug)]
pub struct Tree {
    pub position: Vector,
    pub trunc: Branch,
    pub energy: f64,
    pub colors: LinearGradient,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            position: Vector::new(0.0, 0.0),
            trunc: Branch::default(),
            energy: 50_000.0,
            colors: colorgrad::GradientBuilder::new()
                .html_colors(&["#918464", "#03c454"])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        }
    }
}

impl Tree {
    pub fn update(&mut self, time_delta: f64) {
        let energy = self.use_energy(time_delta);
        self.trunc.update(time_delta, energy);
    }

    fn use_energy(&mut self, time_delta: f64) -> f64 {
        let mut energy_to_use = time_delta * self.trunc.energy_need;

        if self.energy < energy_to_use {
            energy_to_use = self.energy;
            self.energy = 0.0;
        } else {
            self.energy -= energy_to_use;
        }

        energy_to_use
    }
}

pub fn test1(position: Vector) -> Tree {
    Tree {
        position,
        energy: 100_000.0,
        colors: colorgrad::GradientBuilder::new()
            .html_colors(&["#918464", "#03c454", "#ededed"])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
        trunc: Branch {
            vector: Vector::new(-0.1, 1.0),
            limits: Limits {
                angle: -0.6..0.6, // radians
                length: 50.0,
                child_count: 2,
                depth: 13,
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn test2(position: Vector) -> Tree {
    Tree {
        position,
        energy: 120_000.0,
        colors: colorgrad::GradientBuilder::new()
            .html_colors(&["#918464", "#6b7347", "#03c454", "#c961bf"])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
        trunc: Branch {
            vector: Vector::new(0.1, 1.0),
            limits: Limits {
                angle: -0.4..0.4, // radians
                length: 70.0,
                child_count: 2,
                depth: 13,
            },
            ..Default::default()
        },
        ..Default::default()
    }
}
