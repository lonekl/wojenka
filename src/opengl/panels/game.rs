use std::time::Duration;
use glium::{Depth, DepthTest, Display, DrawParameters, Program, Surface, VertexBuffer};
use winit::event::KeyboardInput;
use war_economy_core::Game;
use crate::opengl::algorithms::{Camera, KeyControls};
use crate::opengl::object_conversion::map::map_tiles_to_vertexes;
use crate::opengl::panels::{Panel};
use crate::opengl::triangles::MapVertex;
use crate::opengl::error::{InterfaceError, ToInterfaceError};
use crate::units::{Angle, Matrix4x4};

pub struct GamePanel {

    terrain_program: Program,
    map_vertex_buffer: VertexBuffer<MapVertex>,

    keyboard: KeyControls,
    camera: Camera,

    game: Game,

}

impl GamePanel {

    pub fn new(display: &Display) -> Result<Self, InterfaceError> {
        let game = Game::new();
        let mut camera = Camera::new(Duration::from_secs_f32(1.0 / 60.0));
        camera.position = [0.0, -5.0, -5.0];
        camera.rotation.x = Angle::from_degrees(15.0);

        Ok(Self {
            terrain_program: Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).to_interface_error()?,
            map_vertex_buffer: VertexBuffer::new(display, &map_tiles_to_vertexes(game.map.get_terrain())).to_interface_error()?,

            keyboard: KeyControls::new(),
            camera,

            game,
        })
    }

}

impl Panel for GamePanel {
    fn keyboard_event(&mut self, keyboard_input: KeyboardInput, is_synthetic: bool) -> Result<(), InterfaceError> {

        self.keyboard.process_input(keyboard_input, is_synthetic);

        Ok(())
    }

    fn redraw(&mut self, display: &Display, last_frame_duration: Duration) -> Result<(), InterfaceError> {
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
        ).to_interface_error()?;

        target.finish().to_interface_error()
    }

}

const VERTEX_SHADER: &'static str = r#"
#version 150

in vec3 position;
in vec2 surface_uv;

uniform mat4 projection;
uniform mat3 rotation;
uniform vec3 camera_position;

out vec2 v_surface_uv;

void main() {

    gl_Position = projection * vec4(rotation * (position - camera_position), 1.0);

    v_surface_uv = surface_uv;

}

"#;

const FRAGMENT_SHADER: &'static str = r#"
#version 150

in vec2 v_surface_uv;

out vec4 color;

void main() {

    color = vec4(v_surface_uv.x, 1.0, v_surface_uv.y, 1.0);

}

"#;