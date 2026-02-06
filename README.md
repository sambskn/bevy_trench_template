# bevy_trench_template
<img width="1275" height="720" alt="image" src="https://github.com/user-attachments/assets/e5158f2b-fae8-46b4-89d8-671fada37b74" />
<img width="1800" height="1002" alt="image" src="https://github.com/user-attachments/assets/cf644ca5-62b9-418b-af75-002e455d7435" />


Simple [bevy](https://github.com/bevyengine/bevy) project showcasing usage of the [bevy_trenchbroom](https://github.com/Noxmore/bevy_trenchbroom) crate to make a simple 3d level with some sprites and what not.

There's some sample sprites (ok, *a* sample sprite) and textures in here too, go crazy with em.

## Goals for this template
- Quick iteration with the help of the Trenchbroom editor
- Figuring out the bare minimum to get 3d world + 2d billboard-ed sprites on the bevy end
- Hopefully supporting the new bevy game jam!

## Setup
### Pre-reqs
- [Trenchbroom](https://github.com/TrenchBroom/TrenchBroom/releases)
- Rust (i assume you got this if you're here)
- [Butler](https://itch.io/docs/butler/) for deploying to itch
- [Just](https://just.systems/man/en/) for runnin' stuff

### Getting Started
- Make a new itch project, set it to your environment vars (see `.env.example`)
- Do a project wide Find/Replace for `bevy_trench_template` and replace with your project name
- Hoepfully everything works? lol

## Usage
- Do a `cargo run` (or a `just`) at least once to populate Trenchbroom's config files with the latest macro-defined stuff from your code
- Select your bevy app by name when making a new map in Trenchbroom
- Load the saved `.map` file in bevy (see current example implementation in `main.rs`)

### Trenchbroom

- Lights have a bunch of properties you can configure in Trenchbroom's Enitty Key-Value editor
  - E.g. you can toggle shadow settings/color - and the angle, which I initially missed when tryign to get the DriectionalLight analog to work as I'd expect. Mess around with the light enitty values in Trenchbroom.


## Deploying to itch.io
- WASM
  - run `just wasm-build wasm-deploy`
    - after running the first time, edit the itch project and mark this channel 'wasm' as being what gets run in the web
- Other
  - idk man probably just run `cargo build --release` then copy the binary along with the assets folder and bada bing! you got a game ready to be zipped up
  - will add commands for that using butler simiar to the ones currently defined in the justfile, but you coul dalso jsut to the step above and uplaod the zip to itch.io manually
  - just make the web build work bro please, nobody wants to be downlaoding stuff

### Further Context
Under the hood bevy_trenchbroom is using all the structs you've tagged with their macros to generate the config data for Trenchbroom to use when you're editing in that app, it can be configured to only save that config sometimes, but for this jam template I think it's ok to just leave it as is - but as a result you should be mindful of how in sync your Trenchbroom and your code are. 

## TODO/Stretch Goals 
### (ideally before the Bevy 0.18 gamejam)
- Add example of 'picking' working (e.g. add a highlight border around sprites within a certain distance in the center of the camera view)
- Add justfile commands for other deploy types
- Add a CLI command for automatically making a `zoo` map (one instance of every texture/brush/sprite/etc)

## Contributing
Sincerely would love to get any feedback from trying out this tool, in the form of issues or pull requests! Would also like to keep gen-AI out of here for now, read my AGENTS.md if you have any questions lol.
