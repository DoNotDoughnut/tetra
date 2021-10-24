use std::fs;

use firecore_tetra::graphics::text::{Font, Text};
use firecore_tetra::graphics::{self, Color};
use firecore_tetra::math::Vec2;
use firecore_tetra::{DefaultContext, ContextBuilder, Event, State, TetraError};

struct GameState {
    file: Text,
}

impl GameState {
    fn new(ctx: &mut DefaultContext) -> firecore_tetra::Result<GameState> {
        Ok(GameState {
            file: Text::new(
                "Drop a file onto this window to view the contents.",
                Font::vector(ctx, "./examples/resources/DejaVuSansMono.ttf", 16.0)?,
            ),
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut DefaultContext) -> firecore_tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.file.draw(ctx, Vec2::new(16.0, 16.0));

        Ok(())
    }

    fn event(&mut self, _: &mut DefaultContext, event: Event) -> firecore_tetra::Result {
        if let Event::FileDropped { path } = event {
            let new_content = fs::read_to_string(&path)
                .map_err(|e| TetraError::FailedToLoadAsset { reason: e, path })?;

            self.file.set_content(new_content);
        }

        Ok(())
    }
}

fn main() -> firecore_tetra::Result {
    ContextBuilder::new("File Dropping", 1280, 720)
        .build()?
        .run(GameState::new)
}
