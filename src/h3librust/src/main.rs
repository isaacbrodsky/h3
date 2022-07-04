mod constants;
mod h3api;

mod baseCells;
mod coordijk;
mod faceijk;
mod h3Index;
mod latLng;
mod vec2d;
mod vec3d;

type Error = ();

fn main() {
    let child = 0x8a589c98475ffffu64;
    let mut h = 0u64;
    h3Index::cellToParent(child, 2, &mut h);
    println!("Hello, world! {:x}", h);
}
