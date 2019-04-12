
use tetra::graphics::{DrawParams, Drawable, Rectangle, Texture, Color};
use tetra::glm::Vec2;
use tetra::{Context};

use log::{info, error, debug};

use std::collections::{HashMap};
use crate::pyxeledit;
use crate::tiled;

use pyxeledit::PyxelTilemap;
use tiled::TiledTilemap;
use crate::vecgrid::VecGrid;

#[allow(dead_code)]
impl Tilemap{
    /// create empty tilemap
    /// use add_tiles_from_map or set_tiles to fill
    pub fn new(texture: Texture,tile_width: i64, tile_height: i64, width: usize, height: usize) -> Tilemap{
        info!("create empty tilemap");
        Tilemap{
            width,
            height,
            viewport: DEFAULT_RECTANGLE,
            tile_height,
            tile_width,
            layers: vec![Layer {tiles: VecGrid::new(width, height),..Layer::default()}],
            tile_rectangles: get_tile_rectangles(texture.width(), texture.height(), tile_width, tile_height),
            texture,
            layer_to_draw: DEFAULT_LAYER_TO_DRAW,
        }
    }

    pub fn from_pyxeledit(texture: Texture, data: &str) -> Tilemap{
        let pyxeltilemap = PyxelTilemap::new(data);
        transform_pyxeltilemap(texture, pyxeltilemap)
    }

    pub fn from_tiled(texture: Texture, data: &str) -> Tilemap{
        let tiledtilemap = TiledTilemap::new(data);
        transform_tiledtilemap(texture, tiledtilemap)
    }

    pub fn color(&mut self, color: Color) ->&Tilemap{
        if self.layer_to_draw == -1{
            for mut l in self.layers.iter_mut(){
                l.color = color;
            }
            self
        }else{
            match self.layers.get_mut(self.layer_to_draw as usize){ //layer vec id not = layer.number
                None => self,
                Some(mut l) => {
                    l.color = color;
                    self
                },
            }
        }
    }

    pub fn texture(&mut self, texture: &Texture) ->&Tilemap{
        self.texture = texture.clone();
        self
    }

    /// just a map with tile ids
    /// neither rotation nor flipping
    pub fn set_tiles_from_map(&mut self, layer: usize, list: &[Vec<u32>]){
        let tiles = self.create_tiles_from_map(list);
        match  self.layers.get_mut(layer){
            None => self.add_layer(tiles),
            Some(layer) => layer.tiles = tiles,
        }
    }

    fn create_tiles_from_map(&mut self, list: &[Vec<u32>])->VecGrid<Tile>{
        let mut tiles = VecGrid::new(list.len(), list[0].len());
        for (x,row) in list.iter().enumerate() {
            for (y,id) in row.iter().enumerate(){
                tiles.set(Tile {
                    id: *id,
                    x: x as i64,
                    y: y as i64,
                    position_x: (x as i64 * self.tile_width) as f32,
                    position_y: (y as i64 * self.tile_height) as f32,
                    ..Tile::default()
                },x,y);
            };
        }
        tiles
    }

    fn add_layer(&mut self, tiles: VecGrid<Tile>){
        let layer = Layer{
            tiles,
            ..Layer::default()
        };
        self.layers.push(layer);
    }

    pub fn viewport(&mut self, rectangle: Rectangle) -> &Tilemap{
        self.viewport = rectangle;
        self
    }

    pub fn set_tileid_at(&mut self, layer: usize, new_id: u32, position: Vec2){
        let x = (position.x as i64 / self.tile_width) as usize;
        let y = (position.y as i64 / self.tile_height) as usize;
        match self.layers.get_mut(layer){
            None => error!("layer{} not found!", layer),
            Some(layer) => {
                match layer.tiles.get_mut(x, y){
                    None => layer.tiles.set(Tile{
                            id: new_id,
                            x: x as i64,
                            y: y as i64,
                            position_x: (x as i64 * self.tile_width) as f32,
                            position_y: (y as i64 * self.tile_height) as f32,
                            ..Tile::default()
                        }, x, y),
                    Some(tile) => tile.id = new_id,
                };
            },
        };
    }

    fn is_inside_viewport(&self, position: &Vec2) -> bool{
        !(position.x < self.viewport.x ||
            position.y < self.viewport.y ||
            position.x > self.viewport.x + self.viewport.width ||
            position.y > self.viewport.y + self.viewport.height
        )
    }

    pub fn draw_layer(&mut self, layer_to_draw: i64) ->&Tilemap{
        self.layer_to_draw = layer_to_draw;
        self
    }

    pub fn visibility(&mut self, layer: i64, visibility: bool) ->&Tilemap{
        match self.layers.get_mut(layer as usize){
            None => self,
            Some(mut l) => {
                l.visibility = visibility;
                self
            },
        }
    }

    pub fn get_layer_name(&self, layer: usize) ->&str{
        match self.layers.get(layer as usize){
            None => "",
            Some(layer) => &layer.name,
        }
    }

    pub fn get_id_at_position(&self, layer: usize, position: Vec2) -> i32{
        let x = position.x as i64 / self.tile_width;
        let y = position.y as i64 / self.tile_height;
        self.get_id_at(layer, x as usize, y as usize)
    }

    pub fn get_id_at(&self, layer_nr: usize, x: usize,y: usize) -> i32{
        match self.layers.get(layer_nr) {
            None => {
                debug!("no layer!");
                -1
            },
            Some(layer) => {
                match layer.tiles.get(x,y){
                    None => {
                        debug!("layer[{}] {}, no tile at {},{}!",layer_nr, layer.name, x,y);
                        -1
                    },
                    Some(tile) => tile.id as i32
                }
            },
        }
    }
}

impl Drawable for Tilemap {
    fn draw<P>(&self, ctx: &mut Context, params: P)
        where
          P: Into<DrawParams>,
    {
        let params = params.into();
        for (i, layer) in self.layers.iter().enumerate() {
            if (self.layer_to_draw == -1 || self.layer_to_draw == i as i64) && layer.visibility {
                for tile in layer.tiles.get_data().iter().filter(|t| t.is_some()) {
                    match tile {
                        None => (),
                        Some(tile) => {
                            let tmp_pos = Vec2::new(params.position.x + tile.position_x, params.position.y + tile.position_y);
                            if self.is_inside_viewport(&tmp_pos) || draw_everything(&self.viewport) {
                                self.texture.draw(ctx, DrawParams::new()
                                    .position(tmp_pos)
                                    .clip(self.tile_rectangles[&tile.id])
                                    .rotation(tile.rotation)
                                    .scale(tile.scale)
                                    .color(layer.color)
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct Tilemap {
    width: usize,
    height: usize,
    viewport: Rectangle,
    tile_height: i64,
    tile_width: i64,
    layers: Vec<Layer>,
    tile_rectangles: HashMap<u32, Rectangle>,
    texture: Texture,
    layer_to_draw: i64,
}

pub struct Layer {
    tiles: VecGrid<Tile>,
    name: String,
    visibility: bool,
    color: Color,
}

#[allow(dead_code)]
pub struct Tile {
    id: u32,
    x: i64,
    y: i64,
    position_x: f32,
    position_y: f32,
    rotation: f32,
    scale: Vec2,
}

fn get_tile_rectangles(texture_height: i32, texture_width: i32, tile_width: i64, tile_height: i64) ->HashMap<u32, Rectangle>{
    let mut id = 0;
    let mut tile_rectangles: HashMap<u32, Rectangle> = HashMap::new();
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

fn transform_pyxeltilemap(texture: Texture, pyxeltilemap: PyxelTilemap) ->Tilemap{
    Tilemap{
        width: pyxeltilemap.tileswide as usize,
        height: pyxeltilemap.tileshigh as usize,
        viewport: DEFAULT_RECTANGLE,
        tile_height: pyxeltilemap.tile_height,
        tile_width: pyxeltilemap.tile_width,
        layers: transform_pyxellayer(&pyxeltilemap.layers, pyxeltilemap.tileswide as usize, pyxeltilemap.tileshigh as usize),
        tile_rectangles: get_tile_rectangles(texture.width(), texture.height(), pyxeltilemap.tile_width, pyxeltilemap.tile_height),
        texture,
        layer_to_draw: DEFAULT_LAYER_TO_DRAW,
    }
}

fn transform_pyxellayer(pyxellayers: &[pyxeledit::Layers], width: usize, height: usize) ->Vec<Layer>{
    let mut layers: Vec<Layer> = Vec::new();
    for pyxellayer in pyxellayers.iter().rev(){
        let l = Layer{
            tiles: transform_pyxeltile(&pyxellayer.tiles, width, height),
            name: pyxellayer.name.clone(),
            ..Layer::default()
        };
        layers.push(l);
    }
    layers
}

fn transform_pyxeltile(pyxeltiles: &[pyxeledit::Tile], width: usize, height: usize) -> VecGrid<Tile>{
    let mut vecgrid: VecGrid<Tile> = VecGrid::new(width, height);
    for t in pyxeltiles.iter(){
        let tile = Tile{
                id: t.id as u32,
                x: t.x,
                y: t.y,
                position_x: t.position_x,
                position_y: t.position_y,
                rotation: t.rotation,
                scale: Vec2::new(t.scale.0,t.scale.1),
        };
        vecgrid.set(tile, t.x as usize, t.y as usize);
    };
    vecgrid
}

fn transform_tiledtilemap(texture: Texture, tiledtilemap: TiledTilemap) ->Tilemap{
    Tilemap{
        width: tiledtilemap.tilewidth,
        height: tiledtilemap.tileheight,
        viewport: DEFAULT_RECTANGLE,
        tile_height: tiledtilemap.tile_height,
        tile_width: tiledtilemap.tile_width,
        layers: transform_tiledlayer(&tiledtilemap.layers,tiledtilemap.tilewidth,tiledtilemap.tileheight),
        tile_rectangles: get_tile_rectangles(texture.width(), texture.height(), tiledtilemap.tile_width, tiledtilemap.tile_height),
        texture,
        layer_to_draw: DEFAULT_LAYER_TO_DRAW,
    }
}

fn transform_tiledlayer(tiledlayers: &[tiled::Layer], width: usize, height: usize) ->Vec<Layer>{
    let mut layers: Vec<Layer> = Vec::new();
    for tiledlayer in tiledlayers.iter(){
        let l = Layer{
            tiles: transform_tiledtile(&tiledlayer.tiles, width, height),
            name: tiledlayer.name.clone(),
            ..Layer::default()
        };
        layers.push(l);
    }
    layers
}

fn transform_tiledtile(tiledtiles: &[tiled::Tile], width: usize, height: usize) -> VecGrid<Tile>{
    let mut vecgrid: VecGrid<Tile> = VecGrid::new(width, height);
    for t in tiledtiles.iter(){
        let tile = Tile{
            id: t.id,
            x: t.x,
            y: t.y,
            position_x: t.position_x,
            position_y: t.position_y,
            rotation: t.rotation,
            scale: Vec2::new(t.scale.0,t.scale.1),
        };
        vecgrid.set(tile, t.x as usize, t.y as usize);
    };
    vecgrid
}

fn draw_everything(rectangle: &Rectangle) -> bool{
    let rectangle_to_compare = DEFAULT_RECTANGLE;
    rectangle_to_compare.eq(rectangle)
}

impl Default for Layer {
    fn default() -> Layer {
        Layer{
            tiles: VecGrid::new(1,1),
            name: "".to_string(),
            visibility: true,
            color: Color::rgb(1.0, 1.0, 1.0),
        }
    }
}

impl Default for Tile {
    fn default() -> Tile {
        Tile{
            id: 0,
            x: 0,
            y: 0,
            position_x: 0.0,
            position_y: 0.0,
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
        }
    }
}

const DEFAULT_RECTANGLE: Rectangle = Rectangle{ x: 0.0, y: 0.0, width: 0.0, height: 0.0 };
const DEFAULT_LAYER_TO_DRAW: i64 = -1;