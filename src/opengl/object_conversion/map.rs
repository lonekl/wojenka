use war_economy_core::map::{MapShape, Tile};
use war_economy_core::map::units::TerrainHeight;
use crate::opengl::triangles::MapVertex;

pub fn map_tiles_to_vertexes(terrain: (MapShape, TerrainHeight, &Vec<Tile>)) -> Vec<MapVertex> {
    let (map_shape, tile_size, tiles) = terrain;

    let mut vertex_groups = vec![];

    match map_shape {
        MapShape::Rectangular { width, height } => {
            let width_usize = width as usize;
            //let height_usize = height as usize;
            let x_offset = width as f32 / -2.0;
            let y_offset = height as f32 / -2.0;

            let x_uv_scale = 1.0 / width as f32;
            let y_uv_scale = 1.0 / height as f32;


            for (tile_index, central_tile) in tiles.iter().enumerate() {
                let tile_y = tile_index / width_usize;
                let tile_x = tile_index % width_usize;

                let tile_display_y = tile_y as f32 + y_offset;
                let tile_display_x = tile_x as f32 + x_offset;

                let west_tile = if tile_index == 0 {central_tile} else {&tiles[tile_index - 1]};
                let east_tile = tiles.get(tile_index + 1).unwrap_or(central_tile);
                let south_tile = if tile_index <= width_usize {central_tile} else {&tiles[tile_index - width_usize]};
                let north_tile = tiles.get(tile_index + width_usize).unwrap_or(central_tile);

                let south_west_tile = if tile_index < width_usize + 1 {central_tile} else {&tiles[tile_index - width_usize - 1]};
                let south_east_tile = if tile_index <= width_usize {central_tile} else {&tiles[tile_index - width_usize + 1]};

                let north_west_tile = tiles.get(tile_index + width_usize - 1).unwrap_or(central_tile);
                let north_east_tile = tiles.get(tile_index + width_usize + 1).unwrap_or(central_tile);


                vertex_groups.push(create_tile_square(
                    x_uv_scale,
                    y_uv_scale,
                    tile_display_x,
                    tile_display_y,
                    central_tile.height.to_f32_rescaled(tile_size),
                    west_tile.height.to_f32_rescaled(tile_size),
                    east_tile.height.to_f32_rescaled(tile_size),
                    north_tile.height.to_f32_rescaled(tile_size),
                    south_tile.height.to_f32_rescaled(tile_size),
                    south_west_tile.height.to_f32_rescaled(tile_size),
                    south_east_tile.height.to_f32_rescaled(tile_size),
                    north_west_tile.height.to_f32_rescaled(tile_size),
                    north_east_tile.height.to_f32_rescaled(tile_size),
                ));

            }

        },
    }

    let mut vertexes = vec![];

    for vertex_group in vertex_groups {
        for vertex in vertex_group {
            vertexes.push(vertex);
        }
    }

    vertexes
}

fn create_tile_square(
    x_uv_scale: f32,
    y_uv_scale: f32,
    tile_x: f32,
    tile_y: f32,
    central_height: f32,
    west_height: f32,
    east_height: f32,
    north_height: f32,
    south_height: f32,
    south_west_height: f32,
    south_east_height: f32,
    north_west_height: f32,
    north_east_height: f32,
) -> [MapVertex; 24] {
    let neg_x_uv = tile_x * x_uv_scale + 0.5;
    let central_x_uv = (tile_x + 0.5) * x_uv_scale + 0.5;
    let pos_x_uv = (tile_x + 1.0) * x_uv_scale + 0.5;

    let neg_y_uv = -tile_y * y_uv_scale + 0.5;
    let central_y_uv = -(tile_y + 0.5) * y_uv_scale + 0.5;
    let pos_y_uv = -(tile_y + 1.0) * y_uv_scale + 0.5;

    let west_central_height = (west_height + central_height) / 2.0;
    let east_central_height = (east_height + central_height) / 2.0;
    let south_central_height = (south_height + central_height) / 2.0;
    let north_central_height = (north_height + central_height) / 2.0;

    let south_east_central_height = (south_east_height + south_height + east_height + central_height) / 4.0;
    let south_west_central_height = (south_west_height + south_height + west_height + central_height) / 4.0;
    let north_east_central_height = (north_east_height + north_height + east_height + central_height) / 4.0;
    let north_west_central_height = (north_west_height + north_height + west_height + central_height) / 4.0;

    let vertex_array_2d = [
        MapVertex::create_map_square(
            [tile_x,       tile_y, -south_west_central_height],
            [tile_x + 0.5, tile_y,      -south_central_height],
            [tile_x,       tile_y + 0.5, -west_central_height],
            [tile_x + 0.5, tile_y + 0.5,      -central_height],

            neg_x_uv,
            central_x_uv,
            neg_y_uv,
            central_y_uv,
        ),
        MapVertex::create_map_square(
            [tile_x + 0.5, tile_y,      -south_central_height],
            [tile_x + 1.0, tile_y, -south_east_central_height],
            [tile_x + 0.5, tile_y + 0.5,      -central_height],
            [tile_x + 1.0, tile_y + 0.5, -east_central_height],

            central_x_uv,
            pos_x_uv,
            neg_y_uv,
            central_y_uv,
        ),
        MapVertex::create_map_square(
            [tile_x,       tile_y + 0.5,       -west_central_height],
            [tile_x + 0.5, tile_y + 0.5,            -central_height],
            [tile_x,       tile_y + 1.0, -north_west_central_height],
            [tile_x + 0.5, tile_y + 1.0,      -north_central_height],

            neg_x_uv,
            central_x_uv,
            central_y_uv,
            pos_y_uv,
        ),
        MapVertex::create_map_square(
            [tile_x + 0.5, tile_y + 0.5,            -central_height],
            [tile_x + 1.0, tile_y + 0.5,       -east_central_height],
            [tile_x + 0.5, tile_y + 1.0,      -north_central_height],
            [tile_x + 1.0, tile_y + 1.0, -north_east_central_height],

            central_x_uv,
            pos_x_uv,
            central_y_uv,
            pos_y_uv,
        ),
    ];

    [
        vertex_array_2d[0][0],
        vertex_array_2d[0][1],
        vertex_array_2d[0][2],
        vertex_array_2d[0][3],
        vertex_array_2d[0][4],
        vertex_array_2d[0][5],

        vertex_array_2d[1][0],
        vertex_array_2d[1][1],
        vertex_array_2d[1][2],
        vertex_array_2d[1][3],
        vertex_array_2d[1][4],
        vertex_array_2d[1][5],

        vertex_array_2d[2][0],
        vertex_array_2d[2][1],
        vertex_array_2d[2][2],
        vertex_array_2d[2][3],
        vertex_array_2d[2][4],
        vertex_array_2d[2][5],

        vertex_array_2d[3][0],
        vertex_array_2d[3][1],
        vertex_array_2d[3][2],
        vertex_array_2d[3][3],
        vertex_array_2d[3][4],
        vertex_array_2d[3][5],
    ]
}
