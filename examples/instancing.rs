use firecore_tetra::graphics::mesh::{BorderRadii, GeometryBuilder, Mesh, ShapeStyle};
use firecore_tetra::graphics::{self, Color, Rectangle, Shader};
use firecore_tetra::math::Vec2;
use firecore_tetra::{DefaultContext, ContextBuilder, State};

struct GameState {
    mesh: Mesh,
}

impl GameState {
    fn new(ctx: &mut DefaultContext) -> firecore_tetra::Result<GameState> {
        let mesh = GeometryBuilder::new()
            .rounded_rectangle(
                ShapeStyle::Stroke(2.0),
                Rectangle::new(0.0, 0.0, 16.0, 16.0),
                BorderRadii::new(4.0),
            )?
            .build_mesh(ctx)?;

        let mut offsets = Vec::with_capacity(256);

        for y in 0..16 {
            for x in 0..16 {
                offsets.push(Vec2::new(x as f32 * 32.0, y as f32 * 32.0));
            }
        }

        let shader = Shader::from_vertex_file(ctx, "./examples/resources/instanced.vert")?;
        shader.set_uniform(ctx, "u_offsets", offsets.as_slice());

        graphics::set_shader(ctx, &shader);

        Ok(GameState { mesh })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut DefaultContext) -> firecore_tetra::Result {
        graphics::clear(ctx, Color::rgb(0.094, 0.11, 0.16));

        self.mesh.draw_instanced(ctx, 256, Vec2::new(16.0, 16.0));

        Ok(())
    }
}

fn main() -> firecore_tetra::Result {
    ContextBuilder::new("Instanced Mesh Rendering", 1280, 720)
        .build()?
        .run(GameState::new)
}
