use std::any::Any;
use std::{env, path};

use aabb::AABB;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::input::keyboard::KeyCode;
use ggez::mint::Point2;
use ggez::{conf, input, Context, ContextBuilder, GameResult};

mod aabb;
mod alien;
mod game_object;
mod player;
mod projectile;
mod utils;
use alien::Alien;
use game_object::GameObjectPosition;
use player::Player;

struct MyGame {
    player: Player,
    aliens: Vec<Alien>,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let (width, height) = ctx.gfx.drawable_size();

        MyGame {
            player: Player::new(
                ctx,
                Point2 {
                    x: 100.0,
                    y: height - 100.0,
                },
            ),
            aliens: spawn_aliens(ctx, width, height),
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();
        self.player.update(&ctx, dt);

        self.aliens.retain(|alien: &Alien| alien.is_active);
        for alien in &mut self.aliens {
            alien.update(&ctx, dt);
        }

        for projectile in &mut self.player.projectiles {
            for alien in &mut self.aliens {
                if utils::intersects(
                    AABB {
                        x: projectile.position.x,
                        y: projectile.position.y,
                        width: projectile.game_object.width,
                        height: projectile.game_object.height,
                    },
                    AABB {
                        x: alien.position.x,
                        y: alien.position.y,
                        width: alien.game_object.width,
                        height: alien.game_object.height,
                    },
                ) {
                    alien.is_active = false;
                    projectile.is_active = false;
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        self.player.draw(ctx, &mut canvas);

        for alien in &mut self.aliens {
            alien.draw(ctx, &mut canvas);
        }
        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        self.player.key_down_event(ctx, input);
        match input.keycode {
            Some(KeyCode::Escape) => ctx.request_quit(),
            _ => (), // Do nothing
        }
        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        input: input::keyboard::KeyInput,
    ) -> Result<(), ggez::GameError> {
        self.player.key_up_event(input);
        Ok(())
    }
}

fn spawn_aliens(ctx: &Context, width: f32, _height: f32) -> Vec<Alien> {
    let mut aliens_vec = Vec::new();

    let padding: f32 = 200.0;
    let spawn_position_x_min: f32 = padding;
    let spawn_position_x_max: f32 = width - padding;
    let margin: f32 = 100.0;

    let mut draw_position_x = spawn_position_x_min;
    let mut draw_position_y = margin;

    for _i in 0..8 {
        loop {
            let mut alien = Alien::new(ctx, Point2 { x: 0.0, y: 0.0 });
            let obj_width = alien.game_object.width;
            let spacing =
                ((spawn_position_x_max - spawn_position_x_min) - (obj_width * 11.0)) / 10.0;
            alien.set_position(Point2 {
                x: draw_position_x,
                y: draw_position_y,
            });
            draw_position_x += spacing;

            if alien.get_position(GameObjectPosition::Left) > spawn_position_x_max {
                break;
            } else {
                aliens_vec.push(alien);
            }
        }
        draw_position_x = spawn_position_x_min;
        draw_position_y = draw_position_y + 50.0;
    }

    return aliens_vec;
}

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("Space Invaders", "DC")
        .add_resource_path(resource_dir)
        .window_mode(conf::WindowMode::default().fullscreen_type(conf::FullscreenType::Desktop))
        .build()
        .expect("Could not create ggez context!");
    let my_game = MyGame::new(&mut ctx);
    event::run(ctx, event_loop, my_game);
}
