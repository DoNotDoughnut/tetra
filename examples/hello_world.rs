use tetra::graphics::{self, Color};
use tetra::{DefaultContext, ContextBuilder, State};

struct GameState;

impl State for GameState {
    fn draw(&mut self, ctx: &mut DefaultContext) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Hello, world!", 1280, 720)
        .build()?
        .run(|_| Ok(GameState))
}
