pub mod pyxeledit;
pub mod tiled;
pub mod tile_animation;

use std::collections::HashMap;
use log::{info,debug,error};

use tetra::graphics::{Texture, Color, Rectangle, Drawable, DrawParams};
use tetra::Context;
use tetra::math::Vec2;
use crate::tilemap::pyxeledit::PyxelTilemap;
use crate::tilemap::tiled::TiledTilemap;
use crate::TetraVec2;
use crate::utils::vecgrid::VecGrid;

#[allow(dead_code)]
impl Tilemap{
    /// create empty tilemap
    /// use add_tiles_from_map or set_tiles to fill
    pub fn new(clip: Rectangle, tile_width: i64, tile_height: i64, width: usize, height: usize) -> Tilemap{
        info!("create empty tilemap");
        Tilemap{
            width,
            height,
            viewport: DEFAULT_RECTANGLE,
            tile_height,
            tile_width,
            layers: vec![Layer {tiles: VecGrid::new(width, height),..Layer::default()}],
            tile_rectangles: get_tile_rectangles(clip, tile_width, tile_height),
            layer_to_draw: DEFAULT_LAYER_TO_DRAW,
        }
    }

    pub fn from_pyxeledit(clip: Rectangle, data: &str) -> Tilemap{
        let pyxeltilemap = PyxelTilemap::new(data);
        transform_pyxeltilemap(clip, pyxeltilemap)
    }

    pub fn from_tiled(clip: Rectangle, data: &str) -> Tilemap{
        let tiledtilemap = TiledTilemap::new(data);
        transform_tiledtilemap(clip, tiledtilemap)
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

    /// just a map with tile ids
    /// neither rotation nor flipping
    pub fn set_tiles_from_map(&mut self, layer: usize, list: &[Vec<u32>]){
        let tiles = self.create_tiles_from_map(list);
        match  self.layers.get_mut(layer){
            None => self.add_layer(tiles),
            Some(layer) => layer.tiles = tiles,
        }
    }

    pub fn viewport(&mut self, rectangle: Rectangle) -> &Tilemap{
        self.viewport = rectangle;
        self
    }

    pub fn get_position_from_id(&self,layer: usize, id: u32)-> TetraVec2{
        match self.layers.get(layer) {
            None => TetraVec2::zero(),
            Some(layer) => {
                let tiles = &*layer.tiles.get_data();
                for (i,t) in tiles.iter(). enumerate(){
                    if t.is_some() && t.as_ref().unwrap().id == id{
                        let x = (i % self.width) * self.tile_width as usize;
                        let y = ((i / self.width)+1) * self.tile_height as usize;
                        return TetraVec2::new((x+1) as f32,(y-1) as f32)
                    }
                }
                TetraVec2::zero()
            }
        }
    }

    pub fn replace_all_tileid(&mut self, layer: usize, old_id: u32, new_id: Option<u32>){
        match self.layers.get_mut(layer) {
            None => error!("layer{} not found!", layer),
            Some(layer) => {
                for x in 0..self.height{
                    for y in 0..self.width{
                        if let Some(tile) = layer.tiles.get_mut(x,y) {
                            if tile.id == old_id {
                                if let Some(id) = new_id{
                                    tile.id = id;
                                } else {
                                    layer.tiles.delete(x, y);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn set_tileid_at(&mut self, layer: usize, new_id: u32, position: TetraVec2){
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

    pub fn visibility(&mut self, layer: usize, visibility: bool){
        if let Some(mut l) = self.layers.get_mut(layer) {
            l.visibility = visibility
        }
    }

    pub fn get_layer_id(&self, name: &str) ->usize{
        for (i, layer) in self.layers.iter().enumerate(){
            if layer.name.eq(name){
                return i;
            }
        }
        99
    }

    pub fn get_layer_name(&self, layer: usize) ->&str{
        match self.layers.get(layer as usize){
            None => "",
            Some(layer) => &layer.name,
        }
    }

    pub fn get_id_at_position(&self, layer: usize, position: TetraVec2) -> Option<u32>{
        let x = position.x as i64 / self.tile_width;
        let y = position.y as i64 / self.tile_height;
        self.get_id_at(layer, x as usize, y as usize)
    }

    pub fn get_id_at(&self, layer_nr: usize, x: usize,y: usize) -> Option<u32>{
        match self.layers.get(layer_nr) {
            None => {
                debug!("no layer!");
                None
            },
            Some(layer) => {
                match layer.tiles.get(x,y){
                    None => {
                        debug!("layer[{}] {}, no tile at {},{}!",layer_nr, layer.name, x,y);
                        None
                    },
                    Some(tile) => Some(tile.id)
                }
            },
        }
    }

    fn is_inside_viewport(&self, position: TetraVec2) -> bool{
        !(position.x < self.viewport.x ||
            position.y < self.viewport.y ||
            position.x > self.viewport.x + self.viewport.width ||
            position.y > self.viewport.y + self.viewport.height
        )
    }
    pub fn get_clip_from_id(&self, id: u32) -> Rectangle{
        self.tile_rectangles[&id]
    }

    pub fn get_frames_from_ids(&self, ids: &[u32]) -> Vec<Rectangle>{
        let mut frames = Vec::with_capacity(ids.len());
        for id in ids{
            frames.push(self.tile_rectangles[&(id)]);
        }
        frames
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

    pub fn draw_layer<P>(&mut self, ctx: &mut Context,texture: &Texture, params: P, layer_to_draw: usize)
        where
            P: Into<DrawParams>,
    {
        let params = params.into();
        self.draw(ctx, texture, params,Some(layer_to_draw));
    }

    pub fn draw<P>(&mut self, ctx: &mut Context,texture: &Texture, params: P, layer_to_draw: Option<usize>)
        where
            P: Into<DrawParams>,
    {
        let params = params.into();
        for (i, layer) in self.layers.iter().enumerate() {
            if layer.visibility && layer_to_draw.is_none() || layer_to_draw.is_some() && i == layer_to_draw.unwrap(){
                for tile in layer.tiles.get_data().iter().filter(|t| t.is_some()) {
                    match tile {
                        None => (),
                        Some(tile) => {
                            let tmp_pos = Vec2::new(params.position.x + tile.position_x, params.position.y + tile.position_y);
                            if self.is_inside_viewport(tmp_pos) || draw_everything(&self.viewport) {
                                texture.draw(ctx, DrawParams::new()
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

#[allow(dead_code)]
pub struct Tilemap {
    width: usize,
    height: usize,
    viewport: Rectangle,
    tile_height: i64,
    tile_width: i64,
    layers: Vec<Layer>,
    tile_rectangles: HashMap<u32, Rectangle>,
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
    scale: TetraVec2,
}

fn get_tile_rectangles(clip: Rectangle, tile_width: i64, tile_height: i64) ->HashMap<u32, Rectangle>{
    let mut id = 0;
    let x = i64::from(clip.height as i32) / tile_width;
    let y = i64::from(clip.width as i32) / tile_height;
    let mut tile_rectangles: HashMap<u32, Rectangle> = HashMap::with_capacity((x * y) as usize);
    for i in 0..x{
        for j in 0..y{
            let rec = Rectangle::new(clip.x +(j*tile_width) as f32,clip.y +(i*tile_height) as f32, tile_width as f32, tile_height as f32); //switch x and y axis
            tile_rectangles.insert(id,rec);
            id +=1;
        }
    }
    tile_rectangles
}

fn transform_pyxeltilemap(clip: Rectangle, pyxeltilemap: PyxelTilemap) ->Tilemap{
    Tilemap{
        width: pyxeltilemap.tileswide as usize,
        height: pyxeltilemap.tileshigh as usize,
        viewport: DEFAULT_RECTANGLE,
        tile_height: pyxeltilemap.tile_height,
        tile_width: pyxeltilemap.tile_width,
        layers: transform_pyxellayer(&pyxeltilemap.layers, pyxeltilemap.tileswide as usize, pyxeltilemap.tileshigh as usize),
        tile_rectangles: get_tile_rectangles(clip, pyxeltilemap.tile_width, pyxeltilemap.tile_height),
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

fn transform_tiledtilemap(clip: Rectangle, tiledtilemap: TiledTilemap) ->Tilemap{
    Tilemap{
        width: tiledtilemap.tilewidth,
        height: tiledtilemap.tileheight,
        viewport: DEFAULT_RECTANGLE,
        tile_height: tiledtilemap.tile_height,
        tile_width: tiledtilemap.tile_width,
        layers: transform_tiledlayer(&tiledtilemap.layers,tiledtilemap.tilewidth,tiledtilemap.tileheight),
        tile_rectangles: get_tile_rectangles(clip, tiledtilemap.tile_width, tiledtilemap.tile_height),
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