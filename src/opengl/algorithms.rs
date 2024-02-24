use std::time::Duration;
use winit::event::{ElementState, KeyboardInput, MouseScrollDelta, VirtualKeyCode};
use crate::units::{Angle, RotationXYZ};



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
    pub min_position: [f32; 3],
    pub max_position: [f32; 3],
    pub limit_z_dependency: f32,

    pub height_move_step: f32,

    pub max_height_x_rotation: Angle,
    pub min_height_x_rotation: Angle,
    pub x_rotation_scale_power: i32,

}

impl Camera {

    pub fn new(max_map_axis: (u32, u32)) -> Self {
        let max_map_axis = (max_map_axis.0 as f32 / 2.0, max_map_axis.1 as f32 / 2.0);

        Self {
            rotation: RotationXYZ::ZERO,
            position: [0.0, 0.0, 0.0],

            position_add_target: [0.0, 0.0, 0.0],
            movement_per_second: 2.0,
            movement_softness: 17.5,
            min_position: [-max_map_axis.0, -max_map_axis.1, -75.0],
            max_position: [max_map_axis.0, max_map_axis.1, -2.0],
            limit_z_dependency: 0.475,

            height_move_step: 0.175,

            max_height_x_rotation: Angle::from_degrees(1.0),
            min_height_x_rotation: Angle::from_degrees(65.0),
            x_rotation_scale_power: 10,
        }
    }



    pub fn tick(&mut self, frame_duration: Duration, key_controls: &KeyControls) {
        let effect_rate = frame_duration.as_secs_f32();
        let movement_adder = self.movement_per_second * -self.position[2] * effect_rate;

        key_controls.camera_movement(&mut self.position_add_target, movement_adder);

        let mut motion_rate = self.movement_softness * effect_rate;
        if motion_rate > 1.0 { motion_rate = 1.0; }

        Self::motion_between_bounds(
            self.position[0],
            &mut self.position_add_target[0],
            self.min_position[0] + self.position[2] * self.limit_z_dependency,
            self.max_position[0] - self.position[2] * self.limit_z_dependency
        );
        Self::motion_between_bounds(
            self.position[1],
            &mut self.position_add_target[1],
            self.min_position[1] + self.position[2] * self.limit_z_dependency,
            self.max_position[1] - self.position[2] * self.limit_z_dependency
        );
        Self::motion_between_bounds(
            self.position[2],
            &mut self.position_add_target[2],
            self.min_position[2],
            self.max_position[2]
        );

        let motion_x = self.position_add_target[0] * motion_rate;
        let motion_y = self.position_add_target[1] * motion_rate;
        let motion_z = self.position_add_target[2] * motion_rate;

        self.position[0] += motion_x;
        self.position[1] += motion_y;
        self.position[2] += motion_z;

        self.position_add_target[0] -= motion_x;
        self.position_add_target[1] -= motion_y;
        self.position_add_target[2] -= motion_z;



        let x_rotation_full_scale = (self.max_position[2] - self.min_position[2]).powi(self.x_rotation_scale_power);
        let x_rotation_current_scale = (self.position[2] - self.min_position[2]).powi(self.x_rotation_scale_power);
        let x_rotation_level = x_rotation_current_scale / x_rotation_full_scale;
        self.rotation.x = (self.min_height_x_rotation - self.max_height_x_rotation) * x_rotation_level + self.max_height_x_rotation;

    }



    pub fn mouse_scroll(&mut self, delta: MouseScrollDelta) {

        self.position_add_target[2] -= match delta {
            MouseScrollDelta::LineDelta( _x, y ) => y,
            MouseScrollDelta::PixelDelta( axis ) => axis.y as f32,
        } * self.height_move_step * self.position[2];

    }



    fn motion_between_bounds(position: f32, motion: &mut f32, min_position: f32, max_position: f32) {
        let target = position + *motion;

        if target > max_position {
            *motion = max_position - position;
        }

        if target < min_position {
            *motion = min_position - position;
        }

    }

}
