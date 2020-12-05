extern crate image;

use image::io::Reader as ImageReader;
use image::{GenericImageView, GenericImage, DynamicImage};
//use lab::Lab;
use delta_e::DE2000;
use tera::{Tera, Context};

//use ndarray::Array2;

//use serde::{Serialize};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::collections::HashMap;
use crate::geo_coder::{Point, GeoCoords, trace};


fn are_points_too_close(first: &Point, second: &Point) -> bool {
    return (((first.0 - second.0).pow(2) - (first.1 - second.1).pow(2)) as f32).sqrt() < 40.;
}

fn is_in_radius(vec: &Vec<Point>, point: Point) -> bool {
//    let rng = 0..30;
//    let v: Vec<Point> = Vec::new();
//    let point = Point(1, 1);

    vec.iter().any(
        |point_on_map| are_points_too_close(&point_on_map, &point)
    )
}

fn build_html_page(geo_coords: &Vec<GeoCoords>) -> String {
    let tera = match Tera::new("templates/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };


    let mut context = Context::new();
    context.insert("coordinates", geo_coords);

    return tera.render("points.html", &context).unwrap();
//    context.insert("vat_rate", &0.20);
//    println!("{}", );
//
//    #[derive(Serialize)]
//    struct Product {
//        name: String
//    }


//    let tera = match Tera::new("templates/*.html") {
//        Ok(t) => t,
//        Err(e) => {
//            println!("Parsing error(s): {}", e);
//            ::std::process::exit(1);
//        }
//    };
}

const MIN_MARK_DISTANCE: i32 = 30;
const PATH_COLOR_REFERENCE: [u8; 3] = [241, 125, 12];



pub fn generate_page(image: &[u8]) {
    // let img: DynamicImage = image::load_from_memory(image).unwrap();
    // let mut img_dest: DynamicImage = image::load_from_memory(image).unwrap();
    let path_color = &[241, 125, 12];
    // let path_color = lab::Lab::from_rgb(&[241, 125, 12]);
    // let (image_width, image_height) = img.dimensions();
    //
    // let geo_coder = PathTracer::new(
    //     GeoCoords(48.361830, 134.970572),
    //     GeoCoords(48.354341, 134.990645),
    //     image
    //     // image_width,
    //     // image_height,
    // );


    let mut geo_coords: Vec<GeoCoords> = Vec::new();

    // let mut visited_points: Vec<u8> = Vec::new();
    // visited_points.resize((image_width * image_width) as usize, 1);

    trace(GeoCoords(48.361830, 134.970572), GeoCoords(48.354341, 134.990645), image, path_color);
    // 'outer: for x in 0..image_width {
    //     for y in 0..image_height {
    //         let diff = DE2000::new(
    //             lab::Lab::from_rgba(&img.get_pixel(x, y).0),
    //             path_color,
    //         );
    //         if diff < 5. && visited_points[(x * image_width as u32 + y) as usize] != 0 {
    //             for marker in traverse_path(&mut img_dest, &Point(x as i32, y as i32), &mut visited_points).iter() {
    //                 geo_coords.push(geo_coder.point_to_geo_coords(*marker));
    //             };
    //         }
    //     }
    // }

    let map = build_html_page(&geo_coords);

    let mut generated_map_file = File::create("generated_map.html")
        .expect("unable to create file");
    // img_dest.save("/tmp/track_points.jpg").unwrap();
    // generated_map_file.write_all(map.as_bytes()).expect("unable to write");
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(2, 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(3, 3);
    }
}

//
//#[tokio::main]
//async fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let resp = reqwest::get("https://httpbin.org/ip")
//        .await?
//        .json::<HashMap<String, String>>()
//        .await?;
//    println!("{:#?}", resp);
//    Ok(())
//}
