#![allow(warnings, unused)]
mod systems {
    pub mod Boid;
    pub mod BoidSystem;
}
use bevy::prelude::*;
use bevy::core::FixedTimestep;
use rand::RngCore;
use systems::Boid;
use bevy::tasks::prelude::*;
use systems::BoidSystem;

#[derive(Component)]
pub struct GameState {
    world: BoidSystem::Boid_World,
}

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
pub const WORLD_WIDTH: f32 = 1920.0;
pub const WORLD_HEIGHT: f32 = 1080.0;
pub const BOID_AMOUNT:i32 = 1250;
#[derive(Component)]
pub struct Bird {
    pub id: i32,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_screen_diags::ScreenDiagsPlugin)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(UiCameraBundle::default());
        })
        .insert_resource(GameState {
            world: BoidSystem::Boid_World::new(WORLD_WIDTH, WORLD_HEIGHT, BOID_AMOUNT),
        })
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
            .with_system(update)
        )
        .run();
}

//Create setup system
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, game_state: ResMut<GameState>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let boids = game_state.world.get_boids();
    for i in 0..boids.len() {
        let boid = boids[i];
        let point = boid.get_point();
        let mut transform = Transform::from_translation(point);
        transform.scale = Vec3::new(0.25, 0.25, 1.);
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("graphics/Boid.png"),
                transform,
                ..Default::default()
            })
            .insert(Bird { id: boid.id });
    }
}

fn update(
    mut commands: Commands,
    mut q: Query<(&Bird, &mut Transform)>,
    mut game_state: ResMut<GameState>,
    pool: Res<ComputeTaskPool>
) {
    game_state.world.step();
    let boids = game_state.world.get_boids();
    q.for_each_mut(|(bird,mut transform)|{
        let boid = boids[bird.id as usize];
        let point = boid.get_point();
        transform.translation=Vec3::new(point.x,point.y,0.);
        transform.rotation=(Quat::from_rotation_z((boid.angle-(90. as f32).to_radians())));
    });

}

use std::f32::consts::*;

trait random_inside_unit_circle{
    fn random_inside_unit_circle(radius:f32) -> Vec3;
    fn clamp_length(self, max:f32);
}

impl random_inside_unit_circle for Vec3{
    fn random_inside_unit_circle(radius:f32) -> Vec3{
        let mut rng = rand::thread_rng();
        //Generate random point inside unit circle with radius
        let mut vec=Vec3::new(0.,0.,0.);
        //Get random polar coordinate
        let theta=(rng.next_u32() as f32)*2.*PI;
        let radius_sqrt=radius.sqrt();
        let random_polar_x=radius_sqrt*theta.cos();
        let random_polar_y=radius_sqrt*theta.sin();
        vec.x=random_polar_x;
        vec.y=random_polar_y;
        vec
    }
    //Clamp length of vector to max
    fn clamp_length(mut self,max: f32){
        let length=self.length();
        
        let n = (self.x*self.x + self.y*self.y).sqrt();
        let f=n.min(max)/n;
        self.x*=f;
        self.y*=f;
    }
}