use log::info;
use serde::Deserialize;

const FLIP_HOR_VER_DIA_14_FLAG: u32 = 14;
const FLIP_HOR_VER_12_FLAG: u32 = 12;
const FLIP_HOR_DIA_10_FLAG: u32 = 10;
const FLIP_HOR_8_FLAG: u32 = 8;
const FLIP_VER_DIA_6_FLAG: u32 = 6;
const FLIP_VER_4_FLAG: u32   = 4;
const FLIP_DIA_2_FLAG: u32   = 2;
const ALL_FLIP_FLAGS: u32 = 0x8000_0000 | 0x4000_0000 | 0x2000_0000;

impl TiledTilemap {
    pub fn new(data: &str) -> TiledTilemap{
        if data.contains("<?xml"){
            info!("create Tilemap from Tiled (tmx).");
            let mut tmx_tilemap: TmxTilemap = serde_xml_rs::from_str(data).unwrap();
            remodel_tmx(&mut tmx_tilemap)
        }else{
            info!("create Tilemap from Tiled (json).");
            let mut json_tilemap: JsonTilemap = serde_json::from_str(data).unwrap();
            remodel_json(&mut json_tilemap)
        }
    }
}

pub struct TiledTilemap{
    pub tilewidth: usize,
    pub tileheight: usize,
    pub tile_height: i64,
    pub tile_width: i64,
    pub layers: Vec<Layer>,
    //pub objects: Vec<Object>,
}
#[allow(dead_code)]
pub struct Layer{
    pub id: i32,
    pub name: String,
    width: i64,
    height: i64,
    pub tiles: Vec<Tile>,
    //properties: Vec<Property>,
}

#[derive(Debug, Deserialize)]
#[serde(rename="Map")]
pub struct TmxTilemap{
    version: String,
    tiledversion: String,
    orientation: String,
    renderorder: String,
    //infinite: i8,
    width: i32,
    height: i32,
    #[serde(rename="tilewidth")]
    pub tile_width: i64,
    #[serde(rename="tileheight")]
    pub tile_height: i64,
    #[serde(rename="tileset")]
    tilesets: Vec<Tileset>,
    #[serde(rename="layer")]
    pub layers: Vec<TmxLayer>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "RootInterface")]
pub struct JsonTilemap{
    version: f32,
    tiledversion: String,
    orientation: String,
    renderorder: String,
    //infinite: bool,
    width: i32,
    height: i32,
    #[serde(rename="tilewidth")]
    pub tile_width: i64,
    #[serde(rename="tileheight")]
    pub tile_height: i64,
    tilesets: Vec<Tileset>,
    pub layers: Vec<JsonLayer>,
}

#[derive(Debug, Deserialize)]
struct Tileset{
    firstgid: u32,
    source: String,
}

/* solution with Enum
#[derive(Debug, Deserialize)]
struct Property{
    #[serde(default="default_string")]
    name: String,
    #[serde(rename="type")]
    #[serde(default="default_string")]
    property_type: String,
    #[serde(default="default_string")]
    value: String,
}
*/

#[derive(Debug, Deserialize)]
pub struct TmxLayer{
    id: i32,
    pub name: String,
    width: i64,
    height: i64,
    data: Data,
    #[serde(default="default_vec")]
    pub tiles: Vec<Tile>,
    //#[serde(default="default_vec")]
    //properties: Vec<Property>,
}

#[derive(Debug, Deserialize)]
pub struct JsonLayer{
    id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub layer_type: String,
    #[serde(default="default_i64")]
    width: i64,
    #[serde(default="default_i64")]
    height: i64,
    #[serde(default="default_data")]
    data: Vec<u32>,
    //#[serde(default="default_vec")]
    //properties: Vec<Property>,
}

#[derive(Debug, Deserialize)]
struct Data{
    #[serde(default="default_string")]
    encoding: String,
    #[serde(rename = "$value")]
    #[serde(default="default_string")]
    tile_data: String,
}

#[derive(Debug, Deserialize)]
pub struct Tile {
    #[serde(default="default_u32")]
    pub id: u32,
    #[serde(default="default_i64")]
    pub x: i64,
    #[serde(default="default_i64")]
    pub y: i64,
    #[serde(default="default_f32")]
    pub position_x: f32,
    #[serde(default="default_f32")]
    pub position_y: f32,
    #[serde(default="default_f32")]
    pub rotation: f32,
    #[serde(default="default_scale")]
    pub scale: (f32,f32),
}

fn default_i64() -> i64{
    0
}

fn default_u32() -> u32{
    0
}

fn default_f32() -> f32{
    0.0
}

fn default_vec<T>() -> Vec<T>{
    let test: Vec<T> = Vec::new();
    test
}

fn default_scale() -> (f32,f32){
    (0.0,0.0)
}

fn default_string() -> String{
    String::new()
}

fn default_data() -> Vec<u32>{
   Vec::new()
}

fn remodel_tmx(tilemap: &mut TmxTilemap) -> TiledTilemap{
    let mut layers: Vec<Layer> = vec![];
    let tile_height = tilemap.tile_height;
    let tile_width = tilemap.tile_width;

    for l in tilemap.layers.iter(){
        let data = l.data.tile_data.replace("\r\n","");
        let nums = data.split(',').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let firstgid = tilemap.tilesets[0].firstgid;
        layers.push(Layer{
            id: l.id,
            name: l.name.clone(),
            width: l.width,
            height: l.height,
            tiles: creates_tiles(&nums,l.width,tile_width, tile_height, firstgid),
            //properties: vec![]
        });
    }
    TiledTilemap{
        tileheight: tilemap.height as usize,
        tilewidth: tilemap.width as usize,
        tile_height,
        tile_width,
        layers,
    }
}

fn remodel_json(tilemap: &mut JsonTilemap) -> TiledTilemap{
    let mut layers: Vec<Layer> = vec![];
    let tile_height = tilemap.tile_height;
    let tile_width = tilemap.tile_width;
    for l in tilemap.layers.iter(){
        if l.layer_type.contains("tilelayer"){
            let firstgid = tilemap.tilesets[0].firstgid;
            layers.push(Layer {
                id: l.id,
                name: l.name.clone(),
                width: l.width,
                height: l.height,
                tiles: creates_tiles(&l.data,l.width,tile_width, tile_height, firstgid),
                //properties: vec![]
            });
        }
    }

    TiledTilemap{
        tileheight: tilemap.height as usize,
        tilewidth: tilemap.width as usize,
        tile_height,
        tile_width,
        layers
    }
}

#[allow(clippy::approx_constant)]
fn creates_tiles(nums: &[u32],width: i64,tile_width: i64, tile_height: i64, firstgid: u32) -> Vec<Tile>{
    let mut tiles = vec![];
    let mut y = -1;
    for (i,id) in nums.iter().enumerate(){
        let x = i as i64 % width;
        if x == 0{
            y +=1;
        }
        if *id != 0{
            let id = *id as u32;
            let mut scale_x = 1.0;
            let mut scale_y = 1.0;
            let mut rotation = 0.0;
            let mut shift_x = 0;
            let mut shift_y = 0;
            let flags = (id & ALL_FLIP_FLAGS) >> 28;
            let id: u32 = id & !ALL_FLIP_FLAGS;
            match flags{
                FLIP_DIA_2_FLAG => { scale_y = -1.0; rotation = 1.57 },
                FLIP_VER_4_FLAG => {scale_y = -1.0;shift_y = tile_height},
                FLIP_HOR_8_FLAG => {scale_x = -1.0;shift_x = tile_width},
                FLIP_HOR_VER_DIA_14_FLAG => { scale_x = -1.0;rotation = 1.57; shift_y = tile_height;shift_x = tile_width},
                FLIP_VER_DIA_6_FLAG => { rotation = 4.71;shift_y = tile_height},
                FLIP_HOR_DIA_10_FLAG => {rotation = 1.57;shift_x = tile_width},
                FLIP_HOR_VER_12_FLAG => {rotation = 3.14; shift_y = tile_height; shift_x = tile_width},
                _ => ()
            }

            let t = Tile{
                id: id-firstgid,
                x,
                y,
                position_x: (x * tile_width + shift_x) as f32,
                position_y: (y * tile_height + shift_y) as f32,
                rotation,
                scale: (scale_x, scale_y)
            };
            tiles.push(t)
        }
    }
    tiles
}
