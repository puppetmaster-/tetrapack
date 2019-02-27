
use tetra::graphics::{DrawParams, Drawable, Rectangle, Texture, Color};
use tetra::glm::Vec2;
use tetra::{Context};

use std::collections::{HashMap};
use crate::pyxeledit;
use crate::tiled;

use pyxeledit::PyxelTilemap;
use tiled::TiledTilemap;

impl Tilemap{
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

    pub fn viewport(&mut self, rectangle: Rectangle) -> &Tilemap{
        self.viewport = rectangle;
        self
    }

    pub fn remove_tile(&mut self,layer: usize, position: Vec2){
        let x = position.x as i64 / self.tile_width;
        let y = position.y as i64 / self.tile_height;
        let mut layer = self.layers.get_mut(layer).unwrap();
        layer.tiles.retain(|t| !(t.x ==x && t.y == y));
    }

    pub fn set_tileid_at(&mut self, layer: usize, new_id: u32, position: Vec2){
        let x = position.x as i64 / self.tile_width;
        let y = position.y as i64 / self.tile_height;
        let id: i32 = self.get_id_at(x,y);
        let mut layer = self.layers.get_mut(layer).unwrap();
        if id == -1{
            println!("add new tile!");
            let new_tile = Tile{
                id: new_id,
                x,
                y,
                position_x: (x * self.tile_width) as f32,
                position_y: (y * self.tile_height) as f32,
                rotation: 0.0,
                scale: Vec2::new(1.0,1.0),
            };

            layer.tiles.push(new_tile);
        }else{
            debug!("change existing tile!");
            layer.tiles.iter_mut().filter(|tile| tile.x == x && tile.y == y).for_each(|tile| tile.id = new_id)
        }
    }

    fn is_inside_viewport(&self, position: &Vec2) -> bool{
        if position.x < self.viewport.x ||
            position.y < self.viewport.y ||
            position.x > self.viewport.x + self.viewport.width ||
            position.y > self.viewport.y + self.viewport.height {
            false
        }else{
            true
        }
    }

    pub fn layer(&mut self, layer_to_draw: i64) ->&Tilemap{
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

    pub fn get_layer_name(&self, layer: i64) ->&str{
        match self.layers.get(layer as usize){
            None => "",
            Some(l) => {
                &l.name
            },
        }
    }

    pub fn get_id_at_position(&self,position: Vec2) -> i32{
        let x = position.x as i64 / self.tile_width;
        let y = position.y as i64 / self.tile_height;
        return self.get_id_at(x,y);
    }

    pub fn get_id_at(&self,x: i64,y: i64) -> i32{
        if self.layer_to_draw == -1{
            for layer in self.layers.iter() {
                for tile in layer.tiles.iter(){
                    if tile.x == x && tile.y == y{
                        return tile.id as i32;
                    }
                }
            }
        }else{
            for tile in self.layers.get(self.layer_to_draw as usize).unwrap().tiles.iter(){
                if tile.x == x && tile.y == y{
                    return tile.id as i32;
                }
            }
        }
        return -1;
    }

    pub fn draw<P>(&self, ctx: &mut Context, params: P) where P: Into<DrawParams>{
        let params = params.into();
        for (i, layer) in self.layers.iter().enumerate(){
            if (self.layer_to_draw == -1 && layer.visibility)  || (self.layer_to_draw == i as i64 && layer.visibility){
                for tile in layer.tiles.iter(){
                    let tmp_pos = Vec2::new(params.position.x + tile.position_x, params.position.y + tile.position_y);
                    let rectangle = self.tile_rectangles.get(&tile.id).unwrap();
                    if self.is_inside_viewport(&tmp_pos){ //only draw whats inside viewport
                        self.texture.draw(ctx, DrawParams::new()
                            .position(tmp_pos)
                            .clip(*rectangle)
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

impl Drawable for Tilemap {
    fn draw<P>(&self, ctx: &mut Context, params: P)
        where
            P: Into<DrawParams>,
    {
        let params = params.into();
        for (i, layer) in self.layers.iter().enumerate(){
            if (self.layer_to_draw == -1 && layer.visibility)  || (self.layer_to_draw == i as i64 && layer.visibility){
                for tile in layer.tiles.iter(){
                    let tmp_pos = Vec2::new(params.position.x + tile.position_x, params.position.y + tile.position_y);
                    if self.is_inside_viewport(&tmp_pos) { //only draw whats inside viewport
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

pub struct Tilemap {
    viewport: Rectangle,
    tile_height: i64,
    tile_width: i64,
    layers: Vec<Layer>,
    tile_rectangles: HashMap<u32, Rectangle>,
    texture: Texture,
    layer_to_draw: i64,
}

pub struct Layer {
    tiles: Vec<Tile>,
    name: String,
    visibility: bool,
    color: Color,
}

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
    let x = texture_width as i64 / tile_width;
    let y = texture_height as i64 / tile_height;
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
        viewport: Rectangle::new(0.0, 0.0,100.0,100.0),
        tile_height: pyxeltilemap.tile_height,
        tile_width: pyxeltilemap.tile_width,
        layers: transform_pyxellayer(&pyxeltilemap.layers),
        tile_rectangles: get_tile_rectangles(texture.width(), texture.height(), pyxeltilemap.tile_width, pyxeltilemap.tile_height),
        texture,
        layer_to_draw: -1,
    }
}

fn transform_pyxellayer(pyxellayers: &Vec<pyxeledit::Layers>) ->Vec<Layer>{
    let mut layers: Vec<Layer> = Vec::new();
    for pyxellayer in pyxellayers.iter().rev(){
        let l = Layer{
            tiles: transform_pyxeltile(&pyxellayer.tiles),
            name: pyxellayer.name.clone(),
            visibility: true,
            color: Color::rgb(1.0, 1.0, 1.0),
        };
        layers.push(l);
    }
    layers
}

fn transform_pyxeltile(pyxeltiles: &Vec<pyxeledit::Tile>) -> Vec<Tile>{
    let mut tiles: Vec<Tile> = Vec::new();
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
        tiles.push(tile);
    };
    tiles
}

fn transform_tiledtilemap(texture: Texture, tiledtilemap: TiledTilemap) ->Tilemap{
    Tilemap{
        viewport: Rectangle::new(0.0, 0.0,100.0,100.0),
        tile_height: tiledtilemap.tile_height,
        tile_width: tiledtilemap.tile_width,
        layers: transform_tiledlayer(&tiledtilemap.layers),
        tile_rectangles: get_tile_rectangles(texture.width(), texture.height(), tiledtilemap.tile_width, tiledtilemap.tile_height),
        texture,
        layer_to_draw: -1,
    }
}

fn transform_tiledlayer(tiledlayers: &Vec<tiled::Layer>) ->Vec<Layer>{
    let mut layers: Vec<Layer> = Vec::new();
    for tiledlayer in tiledlayers.iter(){
        let l = Layer{
            tiles: transform_tiledtile(&tiledlayer.tiles),
            name: tiledlayer.name.clone(),
            visibility: true,
            color: Color::rgb(1.0, 1.0, 1.0),
        };
        layers.push(l);
    }
    layers
}

fn transform_tiledtile(tiledtiles: &Vec<tiled::Tile>) -> Vec<Tile>{
    let mut tiles: Vec<Tile> = Vec::new();
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
        tiles.push(tile);
    };
    tiles
}