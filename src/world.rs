use clipline::Clip;
use colorgrad::Gradient;

use crate::{
    branch::Branch,
    tree::{self, Tree},
    types::Vector,
};

pub struct World {
    width: f64,
    height: f64,
    trees: Vec<Tree>,
    clip: Clip<u32>,
}

const TIME_SCALE: f64 = 2.0;

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width as f64,
            height: height as f64,
            clip: Clip::<u32>::new((0, 0), (width - 1, height - 1)).unwrap(),
            trees: vec![
                tree::test1(Vector::new(width as f64 / 3.0, 0.0)),
                tree::test2(Vector::new(width as f64 * 2.0 / 3.0, 0.0)),
            ],
        }
    }

    pub fn update(&mut self, time_delta: f64) {
        for tree in self.trees.iter_mut() {
            tree.update(time_delta * TIME_SCALE);
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        // clear frame
        for byte in frame.iter_mut() {
            *byte = 0;
        }

        for tree in self.trees.iter() {
            self.draw_tree(&tree, &tree.trunc, tree.position, frame);
        }
    }

    fn draw_tree(&self, tree: &Tree, branch: &Branch, position: Vector, frame: &mut [u8]) {
        let next_position = position + branch.vector;
        let height = self.height as u32;

        let start = (
            position.x.floor() as u32,
            height.checked_sub(position.y.floor() as u32).unwrap_or(0),
        );
        let end = (
            next_position.x.floor() as u32,
            height
                .checked_sub(next_position.y.floor() as u32)
                .unwrap_or(0),
        );

        let color = tree
            .colors
            .at(branch.depth as f32 / branch.limits.depth as f32)
            .to_rgba8();

        if let Some(line) = self.clip.any_octant(start, end) {
            line.for_each(|xy| {
                let idx = (xy.1 * self.width as u32 + xy.0) as usize * 4;
                frame[idx..(idx + 4)].copy_from_slice(&color);
            });
        }

        branch.children.iter().for_each(|branch| {
            self.draw_tree(tree, branch, next_position, frame);
        });
    }
}
