<!--[![](https://img.shields.io/discord/273534239310479360.svg?label=discord&style=flat&logo=discord)](https://discordapp.com/channels/273534239310479360)-->
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![](https://img.shields.io/twitter/url/https/shields.io.svg?style=social)](https://twitter.com/fischspiele)
# tetrapack

Extension for the 2D game framework [Tetra](https://github.com/17cupsofcoffee/tetra)

While creating my games I filled the tetrapack with useful extensions.
As far as possible I tried to create them independently so that they can be used in other games.

```rust
[dependencies.tetrapack]
git = "https://github.com/puppetmaster-/tetrapack"
```

___

## Extensions

### [Timer](https://github.com/puppetmaster-/tetrapack/blob/master/src/utils/timer.rs)
Timer runs once and returns a value between 0 and 1.
```rust
let my_timer2 = Timer::new(1800);
let my_timer = Timer::new_sec(30);
```

### [Music](https://github.com/puppetmaster-/tetrapack/blob/master/src/sound/music.rs)
Can be used as background music. The music is faded in and faded out when stopped manually.
Can be configured with a repeat interval.
```rust
// play music.ogg and repeat it after 300 seconds
let my_music = Music::new(ctx,include_bytes!("../../assets/music.ogg"),300)?;
```

### [Mouse](https://github.com/puppetmaster-/tetrapack/blob/master/src/gui/mouse.rs)
Can be used to draw a custom mouse cursor.

### [input actions](https://github.com/puppetmaster-/tetrapack/blob/master/src/input_action.rs)
there are the following fixed actions: is_any_key(), is_cancel(),is_confirmation()
and they must be used in the event() function
```rust
fn event(&mut self, _ctx: &mut Context, event: Event) -> tetra::Result {
    if is_any_key(&event){
        // do something
    }
    Ok()
}
```
### [tilemap](https://github.com/puppetmaster-/tetrapack/blob/master/src/tilemap/mod.rs)
Tilemap is the extension I use most often and has already gone through many iterations.

It can for example be created by a PyxelEdit tilemap (json). 
```rust
let my_tilemap = Tilemap::from_pyxeledit(Rectangle::new(0.0,0.0,512.0,512.0),include_str!("../../assets/tilemap.json"));
```
this is how the visibility of the layer can be set
```rust
my_tilemap.visibility(my_tilemap.get_layer_id("logic"),false);
```
this way a single layer can be drawn
```rust
self.my_tilemap.draw_layer(ctx,&self.atlas,TetraVec2::zero(),self.my_tilemap.get_layer_id("top"));
```
The player start position can be read out this way.
```rust
let player_pos = my_tilemap.get_position_from_id(my_tilemap.get_layer_id("logic"),0);
```
and much more...

### [TileAnimation](https://github.com/puppetmaster-/tetrapack/blob/master/src/tilemap/tile_animation.rs)
If you have a tilemap then you can also create a TileAnimation. 
```rust
let my_tileanimation = TileAnimation::new(&my_tilemap,&[10,11],vec![Duration::from_millis(1000), Duration::from_millis(500)]);
```
___
### Custom Type
**TetraVec2** as tetra::math::Vec2\<f32>
___

It would be great if this list would grow further...

I have been programming as a hobby for around a year in rust.
It is very possible that my code does not comply with the textbook and contains errors.
If you find something that can be solved better or more elegantly or if you even have a new useful extension,
it would be great if you could contribute to this project then we can all benefit from each other.

Have fun using and extending it.