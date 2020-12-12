extern crate image;

use image::{GenericImageView, GenericImage, DynamicImage};
use delta_e::DE2000;
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};

use std::path::{Path};
use url::{Url as UrlLib, ParseError};


use std::fs::File;
use std::fs;
use std::io::Write;
use std::ops::Deref;
use std::collections::HashMap;
use crate::geo_coder::{GeoCoords, Point};
use telegram_bot::MessageEntityKind::Url;


struct GeoCoder {
    top_left: GeoCoords,
    bottom_right: GeoCoords,
    width: u32,
    height: u32,
}

impl GeoCoder {
    fn new(top_left: GeoCoords, bottom_right: GeoCoords, width: u32, height: u32) -> GeoCoder {
        GeoCoder {
            top_left,
            bottom_right,
            width,
            height,
        }
    }

    fn point_to_geo_coords(&self, point: Point) -> GeoCoords {
        GeoCoords(
            self.top_left.0 + ((self.bottom_right.0 - self.top_left.0) / self.height as f32) * point.1 as f32,
            self.top_left.1 + ((self.bottom_right.1 - self.top_left.1) / self.width as f32) * point.0 as f32,
        )
    }
}


fn are_points_too_close(first: &Point, second: &Point) -> bool {
    return (((first.0 - second.0).pow(2) - (first.1 - second.1).pow(2)) as f32).sqrt() < 40.;
}

fn is_in_radius(vec: &Vec<Point>, point: Point) -> bool {
    vec.iter().any(
        |point_on_map| are_points_too_close(&point_on_map, &point)
    )
}

fn build_html_page(geo_coords: &Vec<GeoCoords>, top_left: &GeoCoords, bottom_right: &GeoCoords, center: &GeoCoords, overlay_file: &str) -> String {
    let tera = match Tera::new("templates/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };


    let mut context = Context::new();
    context.insert("coordinates", geo_coords);
    context.insert("top_left", top_left);
    context.insert("bottom_right", bottom_right);
    context.insert("center", center);
    context.insert("overlay_file", overlay_file);

    return tera.render("points.html", &context).unwrap();
}

const MIN_MARK_DISTANCE: i32 = 1;
const PATH_COLOR_REFERENCE: [u8; 3] = [241, 125, 12];

fn traverse_path(img: &mut DynamicImage, initial_point: &Point, visited_points: &mut Vec<u8>) -> Vec<Point> {
    let (image_width, image_height) = img.dimensions();

    let mut marker_points: Vec<Point> = Vec::new();

    let mut points_to_visit: Vec<Point> = Vec::new();
    points_to_visit.push(initial_point.clone());
    let mut next_points_to_visit: Vec<Point> = Vec::new();
    let mut distance = 0;

    let path_color = lab::Lab::from_rgb(&PATH_COLOR_REFERENCE);

    let mut visited_pixel_counter = 0;
    loop {
        next_points_to_visit.clear();
        for current_point in points_to_visit.iter() {
            for x in -1..2 {
                for y in -1..2 {
                    let next_point = Point(current_point.0 + x, current_point.1 + y);
                    if next_point.0 < 0 || next_point.0 >= image_width as i32 {
                        continue;
                    }

                    if next_point.1 < 0 || next_point.1 >= image_height as i32 {
                        continue;
                    }

                    if visited_points[(next_point.0 * image_width as i32 + next_point.1) as usize] == 0 {
                        continue;
                    }
                    visited_points[(next_point.0 * image_width as i32 + next_point.1) as usize] = 0;

                    let color_diff = DE2000::new(
                        lab::Lab::from_rgba(&img.get_pixel(next_point.0 as u32, next_point.1 as u32).0),
                        path_color,
                    );

                    if color_diff < 5. {
                        img.put_pixel(next_point.0 as u32, next_point.1 as u32, image::Rgba([255u8; 4]));
                        next_points_to_visit.push(next_point);
                    }
                }
            }
        }

        if next_points_to_visit.is_empty() {
            break;
        }

        distance += 1;
        if distance >= MIN_MARK_DISTANCE {
            let (first_point, last_point) = (next_points_to_visit.first().unwrap().clone(), next_points_to_visit.last().unwrap().clone());
            marker_points.push(first_point);
            marker_points.push(last_point);
            distance = 0;
        }

        points_to_visit.clear();
        points_to_visit.append(&mut next_points_to_visit);
    }
    return marker_points;
}

pub fn publish_map(top_left: &GeoCoords, bottom_right: &GeoCoords, image: &[u8], file_name: &str) -> String {
    let img = image::load_from_memory(image).unwrap();
    let mut img_dest: DynamicImage = img.clone();

    let path_color = lab::Lab::from_rgb(&[241, 125, 12]);
    let geo_coder = GeoCoder::new(
        (*top_left).clone(),
        (*bottom_right).clone(),
        img.dimensions().0,
        img.dimensions().1,
    );

    let (image_width, image_height) = img.dimensions();
    let mut geo_coords: Vec<GeoCoords> = Vec::new();

    let mut visited_points: Vec<u8> = Vec::new();
    visited_points.resize((image_width * image_width) as usize, 1);

    'outer: for x in 0..image_width {
        for y in 0..image_height {
            let diff = DE2000::new(
                lab::Lab::from_rgba(&img.get_pixel(x, y).0),
                path_color,
            );
            if diff < 5. && visited_points[(x * image_width as u32 + y) as usize] != 0 {
                for marker in traverse_path(&mut img_dest, &Point(x as i32, y as i32), &mut visited_points).iter() {
                    geo_coords.push(geo_coder.point_to_geo_coords(*marker));
                };
            }
        }
    }

    let center = GeoCoords(
        (top_left.0 + bottom_right.0) / 2.,
        (top_left.1 + bottom_right.1) / 2.
    );

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config")).unwrap();


    let storage_path_raw = settings.get_str("map_storage_path").unwrap();
    let storage_path = Path::new(storage_path_raw.as_str());

    let file_name_wo_ext = Path::new(file_name).file_stem().unwrap().to_str().unwrap();
    let file_ext = Path::new(file_name).extension().unwrap().to_str().unwrap();

    let page_name = format!("{}.html", file_name_wo_ext);
    let overlay_name = format!("{}.{}", file_name_wo_ext, file_ext);
    let overlay_path = storage_path.join(&overlay_name);


    let map = build_html_page(&geo_coords, top_left, bottom_right, &center, overlay_name.as_str());

    fs::create_dir_all(&storage_path_raw);

    let mut generated_map_file = File::create(storage_path.join(&page_name)).expect("unable to create file");
    img_dest.save(overlay_path).unwrap();
    generated_map_file.write_all(map.as_bytes()).expect("unable to write");

    let dest_url = settings.get_str("server_url").unwrap();
    UrlLib::parse(dest_url.as_str()).unwrap().join(&page_name.as_str()).unwrap().into_string()
}

fn run() {
    let img = image::open("track.jpg").unwrap();
    let mut img_dest: DynamicImage = image::open("track.jpg").unwrap();
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_run_as_is() {
        run();
    }
}

