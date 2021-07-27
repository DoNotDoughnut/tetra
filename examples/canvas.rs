use tetra::graphics::text::{Font, Text};
use tetra::graphics::{self, Canvas, Color, DrawParams, Shader, Texture};
use tetra::math::Vec2;
use tetra::{ContextBuilder, DefaultContext, State};

struct GameState {
    canvas: Canvas,

    texture: Texture,
    shader: Shader,
    text: Text,

    timer: f32,
    red: f32,
    green: f32,
    blue: f32,
}

impl GameState {
    fn new(ctx: &mut DefaultContext) -> tetra::Result<GameState> {
        Ok(GameState {
            canvas: Canvas::new(ctx, 1280, 720)?,
            texture: Texture::new(ctx, "./examples/resources/player.png")?,
            shader: Shader::from_fragment_file(ctx, "./examples/resources/disco.frag")?,
            text: Text::new(
                "",
                Font::vector(ctx, "./examples/resources/DejaVuSansMono.ttf", 32.0)?,
            ),

            timer: 0.0,
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        })
    }
}

impl State for GameState {
    fn update(&mut self, _ctx: &mut DefaultContext) -> tetra::Result {
        self.timer += 1.0;

        self.red = ((self.timer / 10.0).sin() + 1.0) / 2.0;
        self.green = ((self.timer / 100.0).sin() + 1.0) / 2.0;
        self.blue = ((self.timer / 1000.0).sin() + 1.0) / 2.0;

        self.text.set_content(format!(
            "Red: {:.2}\nGreen: {:.2}\nBlue: {:.2}",
            self.red, self.green, self.blue
        ));

        Ok(())
    }

    fn draw(&mut self, ctx: &mut DefaultContext) -> tetra::Result {
        graphics::set_canvas(ctx, &self.canvas);

        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.texture.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(640.0, 360.0))
                .origin(Vec2::new(8.0, 8.0))
                .scale(Vec2::new(24.0, 24.0)),
        );

        graphics::reset_canvas(ctx);

        graphics::set_shader(ctx, &self.shader);

        self.shader.set_uniform(ctx, "u_red", self.red);
        self.shader.set_uniform(ctx, "u_green", self.green);
        self.shader.set_uniform(ctx, "u_blue", self.blue);

        self.canvas.draw(ctx, Vec2::zero());

        graphics::reset_shader(ctx);

        self.text.draw(ctx, Vec2::new(16.0, 16.0));

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Canvas Rendering", 1280, 720)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
