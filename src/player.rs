use crate::game_object::{GameObject, GameObjectPosition};
use crate::projectile::Projectile;
use crate::utils::clamp;
use ggez::graphics::{Canvas, Image};
use ggez::input::keyboard::KeyCode;
use ggez::{mint::Point2, Context};

const MAX_VELOCITY: f32 = 700.0;

pub(crate) struct Player {
    pub game_object: GameObject,
    pub position: Point2<f32>,
    velocity_x: f32,
    pub projectiles: Vec<Projectile>,
}

impl Player {
    pub fn new(ctx: &Context, position: Point2<f32>) -> Player {
        // get size of canvas size
        let (_width, _height) = ctx.gfx.drawable_size();
        let texture = Image::from_path(ctx, "/space_ship_dark_fit_sm.png")
            .expect("failed to get asset for player");
        Player {
            game_object: GameObject::new(ctx, texture),
            position: Point2 {
                x: position.x,
                y: position.y,
            },
            velocity_x: 0.0,
            projectiles: Vec::new(),
        }
    }

    pub fn update(&mut self, ctx: &Context, dt: f32) {
        let (width, _height) = ctx.gfx.drawable_size();
        self.position.x += self.velocity_x * dt;
        self.position.x = clamp(self.position.x, 0.0, width - 54.0);
        self.projectiles.retain(|projectile: &Projectile| projectile.is_active);
        for (_index, projectile) in self.projectiles.iter_mut().enumerate() {
                projectile.update(ctx, dt)
        }
    }

    pub fn draw(&mut self, ctx: &Context, canvas: &mut Canvas) {
        self.game_object.move_to_position(self.position, canvas);
        
        for (_index, projectile) in self.projectiles.iter_mut().enumerate() {
            if projectile.is_active {
                projectile.draw(ctx, canvas)
            }
        }
    }

    pub fn key_down_event(&mut self, ctx: &Context, input: ggez::input::keyboard::KeyInput) {
        
        match input.keycode {
            Some(KeyCode::Right) => self.velocity_x = MAX_VELOCITY,
            Some(KeyCode::Left) => self.velocity_x = -MAX_VELOCITY,
            Some(KeyCode::Space) => self.projectiles.push(Projectile::new(
                ctx,
                Point2 {
                    x: self.position.x + self.game_object.width / 2.0,
                    y: self.position.y,
                },
            )),
            _ => (),
        }
    }

    pub fn key_up_event(&mut self, input: ggez::input::keyboard::KeyInput) {
        match input.keycode {
            Some(KeyCode::Right) | Some(KeyCode::Left) => self.velocity_x = 0.0,
            _ => (),
        }
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
