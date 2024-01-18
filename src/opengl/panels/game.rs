use std::time::Duration;
use glium::{Depth, DepthTest, Display, DrawParameters, Program, Surface, VertexBuffer};
use winit::event::KeyboardInput;
use war_economy_core::Game;
use crate::opengl::algorithms::{Camera, KeyControls};
use crate::opengl::object_conversion::map::map_tiles_to_vertexes;
use crate::opengl::panels::Panel;
use crate::opengl::triangles::Vertex3d;
use crate::opengl::{FRAGMENT_SHADER, VERTEX_SHADER};
use crate::ResultStringify;
use crate::units::{Angle, Matrix4x4};

pub struct GamePanel {

    terrain_program: Program,
    map_vertex_buffer: VertexBuffer<Vertex3d>,

    keyboard: KeyControls,
    camera: Camera,

    game: Game,

}

impl GamePanel {

    pub fn new(display: &Display) -> Result<Self, String> {
        let game = Game::new();
        let mut camera = Camera::new(Duration::from_secs_f32(1.0 / 60.0));
        camera.position = [0.0, -5.0, -5.0];
        camera.rotation.x = Angle::from_degrees(15.0);

        Ok(Self {
            terrain_program: Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).stringify_error()?,
            map_vertex_buffer: VertexBuffer::new(display, &map_tiles_to_vertexes(game.map.get_terrain())).stringify_error()?,

            keyboard: KeyControls::new(),
            camera,

            game,
        })
    }

}

impl Panel for GamePanel {
    fn keyboard_event(&mut self, keyboard_input: KeyboardInput, is_synthetic: bool) -> Result<(), String> {

        self.keyboard.process_input(keyboard_input, is_synthetic);

        Ok(())
    }

    fn redraw(&mut self, display: &Display, last_frame_duration: Duration) -> Result<(), String> {
        self.camera.tick(last_frame_duration, &self.keyboard);

        let mut target = display.draw();

        let projection_matrix = Matrix4x4::projection_matrix(
            target.get_dimensions(),
            Angle::from_degrees(75.0),
            Angle::from_degrees(75.0),
            0.001,
            1000.0,
        );

        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let draw_parameters = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        target.draw(
            &self.map_vertex_buffer,
            &indices,
            &self.terrain_program,
            &uniform!(
                            projection: projection_matrix.to_arrays(),
                            rotation: self.camera.rotation.rotation_matrix().to_arrays(),
                            camera_position: self.camera.position,
                        ),
            &draw_parameters,
        ).stringify_error()?;

        target.finish().expect("Didn't not Swedish.");
        Ok(())
    }

}
