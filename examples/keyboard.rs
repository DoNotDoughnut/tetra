use firecore_tetra::graphics::{self, Color, DrawParams, Texture};
use firecore_tetra::input::{self, Key};
use firecore_tetra::math::Vec2;
use firecore_tetra::{ContextBuilder, DefaultContext, State};

struct GameState {
    texture: Texture,
    position: Vec2<f32>,
}

impl GameState {
    fn new(ctx: &mut DefaultContext) -> firecore_tetra::Result<GameState> {
        Ok(GameState {
            texture: Texture::new(ctx, "./examples/resources/player.png")?,
            position: Vec2::new(32.0, 32.0),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut DefaultContext) -> firecore_tetra::Result {
        if input::is_key_down(ctx, Key::A) {
            self.position.x -= 4.0;
        }

        if input::is_key_down(ctx, Key::D) {
            self.position.x += 4.0;
        }

        if input::is_key_down(ctx, Key::W) {
            self.position.y -= 4.0;
        }

        if input::is_key_down(ctx, Key::S) {
            self.position.y += 4.0;
        }

        let mut pressed = input::get_keys_pressed(ctx).peekable();
        if pressed.peek().is_some() {
            println!(
                "Keys pressed this update: {:?}",
                pressed.collect::<Vec<_>>()
            );
        }

        let mut released = input::get_keys_released(ctx).peekable();
        if released.peek().is_some() {
            println!(
                "Keys released this update: {:?}",
                released.collect::<Vec<_>>()
            );
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut DefaultContext) -> firecore_tetra::Result {
        graphics::clear(ctx, Color::rgb(0.769, 0.812, 0.631));

        self.texture.draw(
            ctx,
            DrawParams::new()
                .position(self.position)
                .origin(Vec2::new(8.0, 8.0))
                .scale(Vec2::new(2.0, 2.0)),
        );

        Ok(())
    }
}

fn main() -> firecore_tetra::Result {
    ContextBuilder::new("Keyboard Input", 640, 480)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
