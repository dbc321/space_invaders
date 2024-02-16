use ggez::{graphics::{Canvas, Image}, mint::Point2, Context};

use crate::{aabb::AABB, game_object::{GameObject, GameObjectPosition}, utils::clamp};


const MAX_VELOCITY: f32 = 500.0;



pub(crate) struct Alien {
    pub game_object : GameObject,
    pub position: Point2<f32>,
    velocity_x: f32,
    pub is_active: bool

}

impl Alien {

    pub fn new(ctx: &Context, point: Point2<f32>) -> Alien{

        let (_width, _height) = ctx.gfx.drawable_size();
        let texture = Image::from_path(ctx, "/alien_1_fit_sm.png")
        .expect("failed to get asset for player");

        let game_object = GameObject::new(ctx, texture);

        Alien {
            game_object: game_object,
            position: Point2 {
                x: point.x,
                y: point.y,
            },
            velocity_x: 0.0,
            is_active: true
        }
    }

    pub fn update(&mut self, ctx: &Context, dt: f32) {
        let (width, _height) = ctx.gfx.drawable_size();
        self.position.x += self.velocity_x * dt;
        self.position.x = clamp(self.position.x, 0.0, width - (self.game_object.width/2.0));
    }

    pub fn draw(&mut self, _ctx: &Context, canvas: &mut Canvas) {
        self.game_object.move_to_position(self.position, canvas)
    }

    pub fn set_position(&mut self, position: Point2<f32>) {
        self.position = position;
    }

    pub fn get_position(&self, position: GameObjectPosition) -> f32 {
        match position {
            GameObjectPosition::Left => self.position.x,
            GameObjectPosition::Right => self.position.x + self.game_object.width,
            GameObjectPosition::Top => self.position.y - self.game_object.height,
            GameObjectPosition::Bottom => self.position.y,
        }
    }

   
}