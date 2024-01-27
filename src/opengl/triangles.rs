#[derive(Clone, Copy)]
pub struct MapVertex {

    pub position: [f32; 3],
    pub surface_uv: [f32; 2],

}

implement_vertex! { MapVertex, position, surface_uv}

impl MapVertex {

    pub fn create_map_square(
        neg_x_neg_y: [f32; 3],
        pos_x_neg_y: [f32; 3],
        neg_x_pos_y: [f32; 3],
        pos_x_pos_y: [f32; 3],

        neg_x_uv: f32,
        pos_x_uv: f32,
        neg_y_uv: f32,
        pos_y_uv: f32,
    ) -> [MapVertex; 6] {


        [
            MapVertex {
                position: neg_x_neg_y,
                surface_uv: [neg_x_uv, neg_y_uv],
            },
            MapVertex {
                position: pos_x_neg_y,
                surface_uv: [pos_x_uv, neg_y_uv],
            },
            MapVertex {
                position: pos_x_pos_y,
                surface_uv: [pos_x_uv, pos_y_uv],
            },
            MapVertex {
                position: neg_x_neg_y,
                surface_uv: [neg_x_uv, neg_y_uv],
            },
            MapVertex {
                position: pos_x_pos_y,
                surface_uv: [pos_x_uv, pos_y_uv],
            },
            MapVertex {
                position: neg_x_pos_y,
                surface_uv: [neg_x_uv, pos_y_uv],
            },
        ]
    }

}
