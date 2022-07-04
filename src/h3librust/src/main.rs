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
    let mut h2 = 0u64;
    h3Index::latLngToCell(
        h3api::LatLng {
            lat: 37.6095175f64.to_radians(),
            lng: -122.3566462f64.to_radians(),
        },
        9,
        &mut h2,
    );
    println!("Index: {:x}", h2);
}
