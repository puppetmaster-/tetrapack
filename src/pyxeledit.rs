use log::{info, debug};
use serde::Deserialize;

impl PyxelTilemap {
    pub fn new(data: &str) -> PyxelTilemap{
        info!("create Tilemap from PyxelEdit (json).");
        let mut pyxeltilemap: PyxelTilemap = serde_json::from_str(data).unwrap();
        remodel(&mut pyxeltilemap);
        pyxeltilemap
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
