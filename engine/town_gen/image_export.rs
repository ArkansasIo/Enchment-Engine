//! Export a generated city map as a PNG image using the `image` crate

use image::{Rgb, RgbImage};
use super::city_map::CityMap;

pub fn export_city_map_png(city: &CityMap, path: &str) {
    let size = city.size as u32;
    let mut img = RgbImage::new(size, size);
    // Draw Voronoi cell sites as dots
    for cell in &city.cells {
        let x = cell.site.x as u32;
        let y = cell.site.y as u32;
        if x < size && y < size {
            img.put_pixel(x, y, Rgb([0, 0, 0]));
        }
    }
    img.save(path).expect("Failed to save city map PNG");
}

// Usage:
// let city = CityMap::generate(42, 512);
// export_city_map_png(&city, "city_map.png");
