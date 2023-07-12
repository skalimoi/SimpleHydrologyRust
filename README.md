# SimpleHydrologyRust
A minimal Rust port of [weigert](https://github.com/weigert)'s [SimpleHydrology](https://github.com/weigert/SimpleHydrology) program.

I tried porting this myself but failed - so I paid the great [itaig2022](https://www.fiverr.com/itaig2022) on Fiverr to do it. Give him some love! 
I'm making a military simulation game on Rust that will make use of this program, but it might be useful for anyone else. 

## Differences with the original
This is not an identical port. Most of the rendering stuff has been stripped out so that this program just fetches a premade heightmap and erodes it, giving three files as output: an eroded heightmap, a discharge map, and a momentum map. It accepts any resolution, the only thing that will change is the rendering time. I have tried it with 4096x4096 heightmaps and runs just fine.

## Usage
Be sure to place your heightmap in the root folder (where the `cargo.toml` is located) and change the constant `FILE_NAME` on `main.rs` to the name and extension of your heightmap.

Do `cargo run`. 

Default is 100 erosion cycles - to change it, edit the `CYCLES` constant on `main.rs`.
