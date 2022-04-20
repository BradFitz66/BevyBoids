use std::ops::Mul;

use bevy::prelude::*;

use crate::{random_inside_unit_circle, WORLD_HEIGHT, WORLD_WIDTH};

#[derive(Copy,Clone)]
pub struct Boid{
    pub point: Vec3,
    pub accel:Vec3,
    pub velocity: Vec3,
    pub angle:f32,
    pub id:i32,
}

const MAX_FORCE:f32=0.03;
const MAX_SPEED:f32=2.0;

impl Boid{
    pub fn new(x:f32,y:f32,z:f32,id:i32) -> Self{
        Boid{
            
            point: Vec3::new(x,y,z),
            velocity: Vec3::random_inside_unit_circle(1.),
            angle:0.,
            id:id,
            accel:Vec3::new(0.,0.,0.),
        }
    }

    pub fn get_point(&self) -> Vec3{
        self.point.clone()
    }

    pub fn step(&mut self, neighbors:&Vec<Boid>){
        self.border();

        self.separation(neighbors);
        self.alignment(neighbors);
        self.cohesion(neighbors);
        
        self.velocity+=self.accel;
        self.velocity=self.velocity.clamp_length(0.0, MAX_SPEED);
        self.point+=self.velocity;
        self.angle=self.velocity.y.atan2(self.velocity.x);
        self.accel=Vec3::default();
    }
    pub fn border(&mut self){
        if self.point.x > WORLD_WIDTH/2.0 {
            self.point.x = -WORLD_WIDTH/2.0;
        }
        if self.point.x < -WORLD_WIDTH/2.0{
            self.point.x = WORLD_WIDTH/2.0;
        }
        if self.point.y > WORLD_HEIGHT/2.0 {
            self.point.y = -WORLD_HEIGHT/2.0;
        }
        if self.point.y < -WORLD_HEIGHT/2.0 {
            self.point.y = WORLD_HEIGHT/2.0;
        }
    }

    pub fn addForce(&mut self, mut force:Vec3, mul:f32){
        force=force.clamp_length(0.0, MAX_FORCE);
        self.accel+=force*mul;
    }
    pub fn seek(&mut self, desired_pos:Vec3){
        let desired = desired_pos-self.point;
        desired.normalize();
        desired.mul(5.);

        let steer_force:Vec3 = desired-self.velocity;
        self.addForce(steer_force,1.2);
    }

    //Align boids with neighbors
    pub fn alignment(&mut self, neighbors:&Vec<Boid>){
        let mut velocity_sum=Vec3::new(0.,0.,0.);
        let mut neighbor_count=0;
        for neighbor in neighbors{
            if neighbor.id == self.id {
                continue;
            }
            velocity_sum=velocity_sum+neighbor.velocity;
            neighbor_count+=1;
        }
        if neighbor_count > 0 {
            velocity_sum=velocity_sum/neighbor_count as f32;
            velocity_sum=velocity_sum.normalize();
            velocity_sum.mul(5.);
            let steer_force:Vec3 = (velocity_sum-self.velocity);
            self.addForce(steer_force,1.1);
        }
    }

    //Cohesion
    pub fn cohesion(&mut self, neighbors:&Vec<Boid>){
        let mut neighbor_sum=Vec3::new(0.,0.,0.);
        let mut neighbor_count=0;
        for neighbor in neighbors{
            if neighbor.id == self.id {
                continue;
            }
            neighbor_sum=neighbor_sum+neighbor.point;
            neighbor_count+=1;
        }
        if neighbor_count > 0 {
            neighbor_sum=neighbor_sum/neighbor_count as f32;
            self.seek(neighbor_sum);
        }
    }
    //Separation
    pub fn separation(&mut self, neighbors:&Vec<Boid>){
        let mut desired_separation=100.0;
        let mut steer_force=Vec3::new(0.,0.,0.);
        let mut neighbor_count=0;
        for neighbor in neighbors{
            if neighbor.id == self.id {
                continue;
            }
            let distance=self.point.distance(neighbor.point);
            if distance < desired_separation {
                let mut diff=self.point-neighbor.point;
                diff=diff.normalize();
                diff/=distance;

                steer_force+=diff;
                neighbor_count+=1;
            }
        }
        if neighbor_count > 0 {
            steer_force/=neighbor_count as f32;
        }
        if(steer_force.length()>0.){
            steer_force=steer_force.normalize();
            steer_force.mul(MAX_SPEED);
            steer_force-=self.velocity;
            self.addForce(steer_force,1.5);
        }
    }
}
