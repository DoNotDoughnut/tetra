use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::{
    self, Color, DrawParams, Rectangle, StencilAction, StencilState, StencilTest, Texture,
};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

struct GameState {
    circle_mesh: Mesh,
    rectangle_mesh: Mesh,
    texture: Texture,
}

impl GameState {
    pub fn new(ctx: &mut Context<()>) -> tetra::Result<GameState> {
        Ok(Self {
            circle_mesh: Mesh::circle(ctx, ShapeStyle::Fill, Vec2::new(400.0, 300.0), 150.0)?,
            rectangle_mesh: Mesh::rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(0.0, 0.0, 800.0, 600.0),
            )?,
            texture: Texture::new(ctx, "./examples/resources/wabbit_alpha.png")?,
        })
    }
}

impl State<()> for GameState {
    fn draw(&mut self, ctx: &mut Context<()>) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);

        // configure the graphics state for writing to the stencil buffer
        graphics::set_stencil_state(ctx, StencilState::write(StencilAction::Replace, 1));

        // disable writing to the visible pixels
        graphics::set_color_mask(ctx, false, false, false, false);

        // clear the stencil buffer to remove the data from the last frame
        graphics::clear_stencil(ctx, 0);

        // write a circle to the stencil buffer
        self.circle_mesh.draw(ctx, Vec2::zero());

        // enable stencil testing
        graphics::set_stencil_state(ctx, StencilState::read(StencilTest::EqualTo, 1));

        // re-enable writing to the visible pixels
        graphics::set_color_mask(ctx, true, true, true, true);

        // draw a white background and image
        self.rectangle_mesh.draw(ctx, Vec2::zero());

        self.texture.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(400.0, 300.0))
                .scale(Vec2::broadcast(10.0))
                .origin(Vec2::new(
                    self.texture.width() as f32 / 2.0,
                    self.texture.height() as f32 / 2.0,
                )),
        );

        // reset the stencil state
        graphics::set_stencil_state(ctx, StencilState::disabled());

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Stencils", 800, 600)
        .stencil_buffer(true)
        .build(|_| Ok(()))?
        .run(GameState::new)
}
