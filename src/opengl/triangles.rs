#[derive(Clone, Copy)]
pub struct MapVertex {

    pub position: [f32; 3],
    pub surface_uv: [f32; 2],

}

implement_vertex! { MapVertex, position, surface_uv}

impl MapVertex {

    pub fn create_square(
        neg_x_neg_y: [f32; 3],
        pos_x_neg_y: [f32; 3],
        neg_x_pos_y: [f32; 3],
        pos_x_pos_y: [f32; 3],
    ) -> [MapVertex; 6] {


        [
            MapVertex {
                position: neg_x_neg_y,
                surface_uv: [0.0, 0.0],
            },
            MapVertex {
                position: pos_x_neg_y,
                surface_uv: [1.0, 0.0],
            },
            MapVertex {
                position: pos_x_pos_y,
                surface_uv: [1.0, 1.0],
            },
            MapVertex {
                position: neg_x_neg_y,
                surface_uv: [0.0, 0.0],
            },
            MapVertex {
                position: pos_x_pos_y,
                surface_uv: [1.0, 1.0],
            },
            MapVertex {
                position: neg_x_pos_y,
                surface_uv: [0.0, 1.0],
            },
        ]
    }

}
