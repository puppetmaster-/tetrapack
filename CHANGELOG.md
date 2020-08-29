# Changelog

## 0.3.0 - 2020-08-29
* cargo.toml rearranged
* add license.html
* add **Music** (fade in/out, playback can be repeated after a certain time)
* add **Soundpool**
* clippy fix (if let)
* **keyframe** adjustment
* added possibility to fast forward the **timer**
* tilemap extended, added possibility to delete tiles, replace tileid, get position for a specific tileid
* removed texture from tilemap
* add TileAnimation (is based on Tetra Animation with the possibility to set time per frame)

## 0.2.0 - 2020-08-08
* adapted for the tetra 0.4.0 version
* tilemap extends so that the used texture can come from a tileset
* add custom type **TetraVec2** for **Vec2\<f32>**
* **SceneManager** extended , can now be created with a **splashscreen** and/or with a **ScreenScaler**
* add inputActions (is_any_key(),is_cancel(),is_confirmation(), ...)
* added possibility (external crate, including example) to create and play tween and keyframe animations
