use std::cell::UnsafeCell;
use std::mem::{size_of, transmute};
use std::ops::Index;
use std::sync::Arc;
use crate::Definitions;
use crate::map::tile::surface::{SurfaceType, TileSurface};
use crate::map::units::TerrainHeight;

pub mod sectors;
pub mod surface;



pub struct TileArray {

    definitions: Arc<Definitions>,
    tile_amount: u32,
    surface_byte_size: usize,
    tile_byte_size: usize,

    byte_array: Box<[u8]>,

}

impl TileArray {

    pub fn new(definitions: Arc<Definitions>, filler: TileLocal, tile_amount: u32) -> Self {
        let surface_byte_size = size_of::<TileSurface>() * definitions.surface_types.layers.len();
        let tile_byte_size = size_of::<TileSizedData>() + surface_byte_size;
        let mut byte_array = vec![0; tile_amount as usize * tile_byte_size].into_boxed_slice();
        let mut result = Self {
            definitions,
            tile_amount,
            surface_byte_size,
            tile_byte_size,

            byte_array,
        };


        result
    }



    pub fn put(&mut self, index: u32, new_tile: TileLocal) -> Result<(), ()> {
        if index >= self.tile_amount {
            return Err(())
        }

        unsafe {
            (&mut self.byte_array[self.byte_index(index)] as * mut u8 as * mut TileSizedData).write(new_tile.main);

            for (surface_index, new_surface) in new_tile.surface.iter().enumerate() {
                (&mut self.byte_array[
                    self.byte_index(index)
                        + surface_index * size_of::<TileSurface>()
                        + size_of::<TileSizedData>()
                    ] as * mut u8 as * mut TileSurface).write(new_surface.clone());
            }
        }

        Ok(())
    }



    pub fn get(&self, index: u32) -> Option<TileLink> {
        let byte_index = self.byte_index(index);


        if byte_index < self.byte_array.len() {
            let result = unsafe { TileLink {
                main: transmute(&self.byte_array[byte_index]),
                surface: transmute((&self.byte_array[size_of::<TileSizedData>() + byte_index], self.definitions.surface_types.layers.len())),
            }};

            Some(result)
        } else {
            None
        }
    }

    pub fn index(&self, index: u32) -> TileLink {

        self.get(index).expect(&format!("index {index} out of bounds, for {} length", self.tile_amount))
    }



    fn byte_index(&self, full_index: u32) -> usize {

        full_index as usize * self.tile_byte_size
    }

}

impl<'a> IntoIterator for &'a TileArray {
    type Item = TileLink<'a>;
    type IntoIter = TileArrayIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {

        TileArrayIterator {
            current_index: 0,
            tiles: self,
        }
    }
}



pub struct TileArrayIterator<'a> {

    current_index: u32,

    tiles: &'a TileArray,

}

impl<'a> Iterator for TileArrayIterator<'a> {
    type Item = TileLink<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == self.tiles.tile_amount {

            return None
        }
        let result = self.tiles.index(self.current_index);
        self.current_index += 1;

        Some(result)
    }
}



#[derive(Clone)]
pub struct TileLocal {

    pub main: TileSizedData,
    pub surface: Box<[TileSurface]>,

}

impl TileLocal {

    pub fn new(owner: usize, surface: Box<[TileSurface]>) -> Self {

        Self {
            main: TileSizedData {
                height: TerrainHeight::from_meters(10),
                owner,
            },
            surface,
        }
    }

}



#[derive(Clone, Copy)]
pub struct TileLink<'a> {

    pub main: &'a TileSizedData,
    pub surface: &'a [TileSurface],

}

impl<'a> TileLink<'a> {

    pub fn clone_data(&self) -> TileLocal {

        TileLocal {
            main: self.main.clone(),
            surface: self.surface.to_vec().into_boxed_slice(),
        }
    }

}



#[derive(Clone)]
pub struct TileSizedData {

    pub height: TerrainHeight,

    pub owner: usize,

}
