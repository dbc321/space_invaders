use ggez::{
    graphics::{Canvas, Image},
    mint::Point2,
    Context,
};
use rand::Rng;

use crate::{
    aabb::AABB,
    game_object::{self, GameObject},
};

pub(crate) struct Projectile {
    pub game_object: GameObject,
    pub position: Point2<f32>,
    velocity_y: f32,
    pub is_active: bool,
}

impl Projectile {
    pub fn new(ctx: &Context, point: Point2<f32>) -> Projectile {
        let texture =
            Image::from_path(ctx, "/projectile.png").expect("failed to get asset for player");

        let mut rng = rand::thread_rng();
        let rand_projectile_offset = rng.gen_range(-10..10) as f32;
        let game_object = GameObject::new(ctx, texture);
        let width = game_object.width;
       
        Projectile {
            game_object: game_object,
            position: Point2 {
                x: point.x - width/2.0 + rand_projectile_offset,
                y: point.y,
            },
            velocity_y: 700.0,
            is_active: true,
        }
    }

    pub fn update(&mut self, ctx: &Context, dt: f32) {
        let (_width, _height) = ctx.gfx.drawable_size();
        self.position.y -= self.velocity_y * dt;
        if self.position.y < 0.0 {
            self.is_active = false
        }
    }

    pub fn draw(&mut self, _ctx: &Context, canvas: &mut Canvas) {
        match self.is_active {
            true => self.game_object.move_to_position(self.position, canvas),
            false => {}
        }
    }
}
