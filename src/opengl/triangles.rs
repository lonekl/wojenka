#[derive(Clone, Copy)]
pub struct Vertex3d {

    pub position: [f32; 3],
    pub uv: [f32; 2],

}

implement_vertex! { Vertex3d, position, uv}

impl Vertex3d {

    pub fn create_square(
        neg_x_neg_y: [f32; 3],
        pos_x_neg_y: [f32; 3],
        neg_x_pos_y: [f32; 3],
        pos_x_pos_y: [f32; 3],
    ) -> [Vertex3d; 6] {


        [
            Vertex3d {
                position: neg_x_neg_y,
                uv: [0.0, 0.0],
            },
            Vertex3d {
                position: pos_x_neg_y,
                uv: [1.0, 0.0],
            },
            Vertex3d {
                position: pos_x_pos_y,
                uv: [1.0, 1.0],
            },
            Vertex3d {
                position: neg_x_neg_y,
                uv: [0.0, 0.0],
            },
            Vertex3d {
                position: pos_x_pos_y,
                uv: [1.0, 1.0],
            },
            Vertex3d {
                position: neg_x_pos_y,
                uv: [0.0, 1.0],
            },
        ]
    }

}
