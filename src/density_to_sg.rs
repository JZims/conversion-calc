// src/densitytosg.rs


   pub fn density_to_specific_gravity(density: f32) -> f32 {
    // Assuming the density of water is 1.0 g/cm³
    let water_density: f32 = 1.0;
    let specific_gravity = density / water_density;
    specific_gravity
    }


// fn density_to_specific_gravity(density: f32) -> f32 {
//     // Assuming the density of water is 1.0 g/cm³
//     let water_density: f32 = 1.0;
//     let specific_gravity = density / water_density;
//     specific_gravity
// }

// fn main() {
//     let density: f32 = 2.5; // Example density value
//     let specific_gravity = density_to_specific_gravity(density);
//     println!("The specific gravity for a density of {} g/cm³ is: {}", density, specific_gravity);
// }