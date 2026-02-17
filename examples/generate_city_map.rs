//! Example: Generate and export a procedural city map as PNG

use engine::town_gen::{generate_town, image_export::export_city_map_png};

fn main() {
    let city = generate_town(42, 512);
    export_city_map_png(&city, "city_map.png");
    println!("City map exported to city_map.png");
}
