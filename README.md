# bevy_trench_template
<img width="1282" height="752" alt="trench_temp" src="https://github.com/user-attachments/assets/cc781751-6ad8-4e8e-a837-0ffbc2364850" />

Simple [bevy](https://github.com/bevyengine/bevy) project showcasing usage of the [bevy_trenchbroom](https://github.com/Noxmore/bevy_trenchbroom) crate to make a simple 3d level with some sprites and what not.

There's some sample sprites (ok, *a* sample sprite) and textures in here too, go crazy with em.

## Goals for this template
- Quick iteration with the help of the Trenchbroom editor
- Figuring out the bare minimum to get 3d world + 2d billboard-ed sprites on the bevy end
- Hopefully supporting the new bevy game jam!

## Pre-reqs
- [Trenchbroom](https://github.com/TrenchBroom/TrenchBroom/releases)
- Rust (i assume you got this if you're here)

## Usage
- Do a `cargo run` at least once to populate Trenchbroom's config files with the latest macro-defined stuff from your code
- Select your bevy app by name when making a new map in Trenchbroom
- Load the saved `.map` file in bevy (see current example implementation in `main.rs`)

### Further Context
Under the hood bevy_trenchbroom is using all the structs you've tagged with their macros to generate the config data for Trenchbroom to use when you're editing in that app, it can be configured to only save that config sometimes, but for this jam template I think it's ok to just leave it as is - but as a result you should be mindful of how in sync your Trenchbroom and your code are. 

## TODO/Stretch Goals 
### (ideally before the Bevy 0.18 gamejam)
- Add a CLI command for automatically making a `zoo` map (one instance of every texture/brush/sprite/etc)
- Add example of 'picking' working (e.g. add a highlight border around sprites within a certain distance in the center of the camera view)
- Figure out what's up with directional lighting funkiness 
    - It's built in with bevy_trenchbroom but I can't figure out how to change the direction of the light. Prefer point lights for now.
- Make sure this all works for a WASM build, and/or implement nice to have build helpers from other Bevy templates
- Uhhh why do the sprites be rotating weird....

## Contributing
Sincerely would love to get any feedback from trying out this tool, in the form of issues or pull requests! Would also like to keep gen-AI out of here for now, read my AGENTS.md if you have any questions lol.
