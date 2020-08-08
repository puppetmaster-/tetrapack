use crate::TetraVec2;
use std::collections::HashMap;
use tetra::graphics::Rectangle;
use log::{info,debug};
use serde::{Deserialize};

#[allow(dead_code)]
impl PyxelTilemap {
    pub fn new(data: &str) -> PyxelTilemap{
        info!("create Tilemap from PyxelEdit (json).");
        let mut pyxeltilemap: PyxelTilemap = serde_json::from_str(data).unwrap();
        remodel(&mut pyxeltilemap);
        pyxeltilemap
    }
    pub fn get_id_at_position(&self, layer: Layers, position: TetraVec2) -> Option<i32>{
        let x = position.x as i64 / self.tile_width;
        let y = position.y as i64 / self.tile_height;
        let i = (x*y) as usize;
        match layer.tiles.get(i){
            Some(tile) => {
                let id = tile.id as i32;
                std::thread::spawn(move || drop(layer));
                Some(id)
            },
            _ => None
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename = "RootInterface")]
pub struct PyxelTilemap {
    pub tileshigh: i64,
    pub tileswide: i64,
    #[serde(rename = "tileheight")]
    pub tile_height: i64,
    #[serde(rename = "tilewidth")]
    pub tile_width: i64,
    pub layers: Vec<Layers>,
}

#[derive(Debug, Deserialize)]
pub struct Layers {
    pub number: i64,
    pub tiles: Vec<Tile>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Tiles")]
pub struct Tile {
    #[serde(rename = "tile")]
    pub id: i32,
    pub x: i64,
    pub y: i64,
    #[serde(rename = "flipX")]
    flip_x: bool,
    index: i64,
    #[serde(rename = "rot")]
    rotation_id: i8,
    #[serde(default="default_f32")]
    pub position_x: f32,
    #[serde(default="default_f32")]
    pub position_y: f32,
    #[serde(default="default_f32")]
    pub rotation: f32,
    #[serde(default="default_scale")]
    pub scale: (f32,f32),
}

#[allow(clippy::approx_constant)]
fn pyxel_rotation(rotation: i8) ->f32{
    let rot = rotation;
    let mut return_value = 0.0;
    if rot == 1{
        return_value = 1.57;
    } else if rot == 2{
        return_value = 3.14;
    } else if rot == 3 {
        return_value = 4.71;
    }
    return_value
}

fn default_f32() -> f32{
    0.0
}

fn default_scale() -> (f32,f32){
    (0.0,0.0)
}

#[allow(dead_code)]
fn remodel(tilemap: &mut PyxelTilemap){
    debug!("tilemap has {} layers",tilemap.layers.len());
    for (i,layer) in tilemap.layers.iter_mut().enumerate() {
        layer.tiles.retain(|t| t.id != -1);
        debug!("layer {} has {} tile",i,layer.tiles.len());
        for tile in layer.tiles.iter_mut(){
            let mut scale = (1.0,1.0);
            let mut shift_x = 0;
            let mut shift_y = 0;
            let tile_width = tilemap.tile_width;
            let tile_height = tilemap.tile_height;

            if tile.flip_x {
                scale = (-1.0,1.0);
                if tile.rotation_id == 0 {
                    shift_x = tile_width;
                }else if tile.rotation_id == 1 {
                    shift_x = tile_width;
                    shift_y = tile_height;
                }else if tile.rotation_id == 2{
                    shift_y = tile_height;
                }
            } else if tile.rotation_id == 1{
                shift_x = tile_width;
            } else if tile.rotation_id == 2{
                shift_x = tile_width;
                shift_y = tile_height;
            } else if tile.rotation_id == 3{
                shift_y = tile_height;
            }
            tile.position_x = (tile.x * tilemap.tile_width + shift_x) as f32;
            tile.position_y = (tile.y * tilemap.tile_height + shift_y) as f32;
            tile.rotation = pyxel_rotation(tile.rotation_id);
            tile.scale = scale;
        }
    }
}

#[allow(dead_code)]
pub fn get_tile_rectangles(texture_height: i32, texture_width: i32, tile_width: i64, tile_height: i64) ->HashMap<i32, Rectangle>{
    let mut id = 0;
    let mut tile_rectangles: HashMap<i32, Rectangle> = HashMap::new();
    let x = i64::from(texture_width) / tile_width;
    let y = i64::from(texture_height) / tile_height;
    for i in 0..x{
        for j in 0..y{
            let rec = Rectangle::new((j*tile_width) as f32,(i*tile_height) as f32, tile_width as f32, tile_height as f32); //switch x and y axis
            tile_rectangles.insert(id,rec);
            id +=1;
        }
    }
    tile_rectangles
}