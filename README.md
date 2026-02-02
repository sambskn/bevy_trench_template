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
- [Butler](https://itch.io/docs/butler/) for deploying to itch
- [Just](https://just.systems/man/en/) for runnin' stuff

## Usage
- Do a `cargo run` (or a `just`) at least once to populate Trenchbroom's config files with the latest macro-defined stuff from your code
- Select your bevy app by name when making a new map in Trenchbroom
- Load the saved `.map` file in bevy (see current example implementation in `main.rs`)

## Deploying to itch.io
- WASM
  - run `just wasm-build wasm-deploy`
    - after running the first time, edit the itch project and mark this channel 'wasm' as being what gets run in the web

### Further Context
Under the hood bevy_trenchbroom is using all the structs you've tagged with their macros to generate the config data for Trenchbroom to use when you're editing in that app, it can be configured to only save that config sometimes, but for this jam template I think it's ok to just leave it as is - but as a result you should be mindful of how in sync your Trenchbroom and your code are. 

## TODO/Stretch Goals 
### (ideally before the Bevy 0.18 gamejam)
- Add a CLI command for automatically making a `zoo` map (one instance of every texture/brush/sprite/etc)
- Add example of 'picking' working (e.g. add a highlight border around sprites within a certain distance in the center of the camera view)
- Figure out what's up with directional lighting funkiness 
    - It's built in with bevy_trenchbroom but I can't figure out how to change the direction of the light. Prefer point lights for now.
- Add justfile commands for other deploy types

## Contributing
Sincerely would love to get any feedback from trying out this tool, in the form of issues or pull requests! Would also like to keep gen-AI out of here for now, read my AGENTS.md if you have any questions lol.
