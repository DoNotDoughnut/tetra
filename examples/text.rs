use firecore_tetra::graphics::text::{Font, Text};
use firecore_tetra::graphics::{self, Color};
use firecore_tetra::math::Vec2;
use firecore_tetra::{DefaultContext, ContextBuilder, State};

const TEXT_OFFSET: Vec2<f32> = Vec2::new(16.0, 16.0);

struct GameState {
    vector_text: Text,
    bitmap_text: Text,
}

impl GameState {
    fn new(ctx: &mut DefaultContext) -> firecore_tetra::Result<GameState> {
        let vector_text = Text::new(
            "Hello, world!\n\nThis is some text being rendered from a TTF font.",
            Font::vector(ctx, "./examples/resources/DejaVuSansMono.ttf", 16.0)?,
        );

        let bitmap_text = Text::new(
            "Hello, world!\n\nThis is some text being rendered from a bitmap font.",
            Font::bmfont(ctx, "./examples/resources/DejaVuSansMono.fnt")?,
        );

        Ok(GameState {
            vector_text,
            bitmap_text,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut DefaultContext) -> firecore_tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.vector_text.draw(ctx, TEXT_OFFSET);
        self.bitmap_text
            .draw(ctx, TEXT_OFFSET + Vec2::new(0.0, 128.0));

        Ok(())
    }
}

fn main() -> firecore_tetra::Result {
    ContextBuilder::new("Rendering Text", 1280, 720)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
