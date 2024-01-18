use std::time::Duration;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};
use crate::units::RotationXYZ;



pub struct KeyControls {

    pub controls: [(bool, VirtualKeyCode); Self::NUMBER_OF_KEYS],

}

impl KeyControls {
    pub const NUMBER_OF_KEYS: usize = 4;

    pub const GO_NORTH: usize = 0;
    pub const GO_SOUTH: usize = 1;
    pub const GO_EAST: usize =  2;
    pub const GO_WEST: usize =  3;

    pub fn new() -> Self {

        Self {
            controls: [
                (false, VirtualKeyCode::W),
                (false, VirtualKeyCode::S),
                (false, VirtualKeyCode::D),
                (false, VirtualKeyCode::A),
            ],
        }
    }

    pub fn process_input(&mut self, input: KeyboardInput, is_synthetic: bool) {
        if !is_synthetic {
            for control in &mut self.controls {
                if Some(control.1) == input.virtual_keycode {
                    control.0 = match input.state {
                        ElementState::Pressed => true,
                        ElementState::Released => false,
                    }}}}
    }


    fn camera_movement(&self, position: &mut [f32; 3], adder: f32) {

        if self.controls[Self::GO_EAST].0 { position[0] += adder; }
        if self.controls[Self::GO_WEST].0 { position[0] -= adder; }
        if self.controls[Self::GO_NORTH].0 { position[1] += adder; }
        if self.controls[Self::GO_SOUTH].0 { position[1] -= adder; }

    }

}



pub struct Camera {

    pub rotation: RotationXYZ,
    pub position: [f32; 3],
    pub position_add_target: [f32; 3],
    pub movement_per_second: f32,
    pub movement_softness: f32,

}

impl Camera {

    pub fn new(target_frame_duration: Duration) -> Self {

        Self {
            rotation: RotationXYZ::ZERO,
            position: [0.0, 0.0, 0.0],
            position_add_target: [0.0, 0.0, 0.0],
            movement_per_second: 2.0,
            movement_softness: 17.5,
        }
    }


    pub fn tick(&mut self, frame_duration: Duration, key_controls: &KeyControls) {
        let effect_rate = frame_duration.as_secs_f32();
        let movement_adder = self.movement_per_second * -self.position[2] * effect_rate;

        key_controls.camera_movement(&mut self.position_add_target, movement_adder);

        let mut motion_rate = self.movement_softness * effect_rate;
        if motion_rate > 1.0 { motion_rate = 1.0; }

        let motion_x = self.position_add_target[0] * motion_rate;
        let motion_y = self.position_add_target[1] * motion_rate;
        let motion_z = self.position_add_target[2] * motion_rate;

        self.position[0] += motion_x;
        self.position[1] += motion_y;
        self.position[2] += motion_z;

        self.position_add_target[0] -= motion_x;
        self.position_add_target[1] -= motion_y;
        self.position_add_target[2] -= motion_z;

    }

}
