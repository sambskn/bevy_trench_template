# bevy_trench_template
Simple bevy project showcasing usage of the bevy_trenchbroom crate to make a simple 3d level

Focuses:
- Quick iteration with the help of the Trenchbroom editor
- Figuring out the bare minimum to get 3d world + 2d billboard-ed sprites on the bevy end
- Hopefully supporting the new bevy game jam!

## Setup
1. Have Trenchbroom installed
2. Do a `cargo run` once locally to set the Trenchbroom config to recognize our game
3. Open up the `.map` file in Trenchbroom or make a new one and select the game's config
4. Repeat? Add more?


## Stretch Goals (ideally before the Bevy 0.18 gamejam)
- Add example of 'picking' working (e.g. add a highlight border around sprites within a certain distance in the center of the camera view)
- Figure out what's up with directional lighting funkiness 
- Make sure this all works for a WASM build, and/or implement nice to have build helpers from other Bevy templates