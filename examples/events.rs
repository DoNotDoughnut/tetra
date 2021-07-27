use tetra::graphics::text::{Font, Text};
use tetra::graphics::{self, Color};
use tetra::math::Vec2;
use tetra::{ContextBuilder, DefaultContext, Event, State};

struct GameState {
    text: Text,
}

impl GameState {
    fn new(ctx: &mut DefaultContext) -> tetra::Result<GameState> {
        let text = Text::new(
            "Look at your console to see what events are being fired!",
            Font::vector(ctx, "./examples/resources/DejaVuSansMono.ttf", 16.0)?,
        );

        Ok(GameState { text })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut DefaultContext) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.text.draw(ctx, Vec2::new(16.0, 16.0));

        Ok(())
    }

    fn event(&mut self, _: &mut DefaultContext, event: Event) -> tetra::Result {
        println!("{:?}", event);
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Events", 1280, 720)
        .resizable(true)
        .build()?
        .run(GameState::new)
}
