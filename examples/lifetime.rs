use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::math::Vec2;
use tetra::{ContextBuilder, Context, State};

struct GameContext<'c> {
    tetra: Context,
    text: &'c str,
}

impl<'c> AsRef<Context> for GameContext<'c> {
    fn as_ref(&self) -> &Context {
        &self.tetra
    }
}

impl<'c> AsMut<Context> for GameContext<'c> {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.tetra
    }
}

struct GameState {
    texture: Texture,
}

impl GameState {
    fn new<'c>(ctx: &mut GameContext<'c>) -> tetra::Result<GameState> {
        Ok(GameState {
            texture: Texture::new(ctx.as_mut(), "./examples/resources/player.png")?,
        })
    }
}

impl<'c> State<GameContext<'c>> for GameState {
    fn draw(&mut self, ctx: &mut GameContext<'c>) -> tetra::Result {
        graphics::clear(ctx.as_mut(), Color::rgb(0.769, 0.812, 0.631));

        println!("{}", ctx.text);

        self.texture.draw(
            ctx.as_mut(),
            DrawParams::new()
                .position(Vec2::new(32.0, 32.0))
                .origin(Vec2::new(8.0, 8.0))
                .scale(Vec2::new(2.0, 2.0)),
        );

        Ok(())
    }
}

fn main() -> tetra::Result {

    let text = String::from("Lifetime Test");

    let mut ctx = GameContext {
        tetra: ContextBuilder::new("Rendering a Texture", 640, 480)
        .quit_on_escape(true)
        .build()?,
        text: text.as_str(),
    };

    tetra::run(&mut ctx, GameState::new)
    
}
