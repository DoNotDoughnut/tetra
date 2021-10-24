use firecore_tetra::graphics::text::{Font, Text};
use firecore_tetra::graphics::{self, Color};
use firecore_tetra::math::Vec2;
use firecore_tetra::{ContextBuilder, DefaultContext, Event, State};

struct GameState {
    text: Text,
}

impl GameState {
    fn new(ctx: &mut DefaultContext) -> firecore_tetra::Result<GameState> {
        let text = Text::new(
            "Look at your console to see what events are being fired!",
            Font::vector(ctx, "./examples/resources/DejaVuSansMono.ttf", 16.0)?,
        );

        Ok(GameState { text })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut DefaultContext) -> firecore_tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.text.draw(ctx, Vec2::new(16.0, 16.0));

        Ok(())
    }

    fn event(&mut self, _: &mut DefaultContext, event: Event) -> firecore_tetra::Result {
        println!("{:?}", event);
        Ok(())
    }
}

fn main() -> firecore_tetra::Result {
    ContextBuilder::new("Events", 1280, 720)
        .resizable(true)
        .build()?
        .run(GameState::new)
}
