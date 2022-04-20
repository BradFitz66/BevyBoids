use super::Boid::*;
use bevy::{prelude::*, math::Vec3A};
use std::collections::HashMap;
use rand::Rng;

#[derive(Clone)]
pub struct Boid_World {
    width: f32,
    height: f32,
    boids: Vec<Boid>,
}
impl Boid_World {
    pub fn new(width: f32, height: f32, boid_amount: i32) -> Self {
        let mut boid_vec = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..boid_amount {
            boid_vec.push(Boid::new(
                rng.gen_range(-width / 2.0..width / 2.0),
                rng.gen_range(-height / 2.0..height / 2.0),
                0.,
                i,
            ));
        }

        Boid_World {
            width,
            height,
            boids: boid_vec,
        }
    }

    pub fn get_boids(&self) -> Vec<Boid> {
        self.boids.clone()
    }
    //TODO: Add some sort of sparse spatial partioning system to make this faster
    /*
        Spatial grid system plan:
        - Create an empty hash table of types IVec3, Vec<i32>
        - When a boid is added, add it's ID to the cell it's in or create a new one and add it to that
        - Each cell has a list of boid IDs (can't store boids themselves since they use Vec3s which can't use Hash or Eq due to f32/f64)
        - When a boid is removed, remove it's ID from the cell it's in
        - When a boid is updated, remove it's ID from the cell it's in and add it to the new cell if it's changed cells

        Issues experienced with previous attempts:
        - Very difficult to debug with Bevy due to it being a pain to draw primitive shapes like squares to represent. The lyon library isn't that great at this, either
        - Bug with boids slowly going to the middle of the world and staying there.

        Best choice is to find a generic (i.e. isn't dependent on any specific framework) sparse grid system.
    */
    pub fn get_boid_neighbors(&self, b: &Boid) -> Vec<Boid> {
        let grid_x = (b.get_point().x / 8.).floor();
        let grid_y = (b.get_point().y / 8.).floor();
        let mut neighbors = Vec::new();
        //Iterate through all boids and use grid coordinates to filter
        neighbors.extend(self.boids.iter().filter(|&boid| {
            //Check if distance is below a certain threshold
            let dist = boid.get_point().distance(b.get_point());

            return dist <= 50.;
        }));
        neighbors
    }

    pub fn step(&mut self) {
        for i in 0..self.boids.len() {
            let neighbors = self.get_boid_neighbors(&self.boids[i]);
            self.boids[i].step(&neighbors);
        }
    }
}
