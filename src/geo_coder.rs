use serde::{Serialize, Deserialize};
use delta_e::DE2000;
use image::{DynamicImage, GenericImage, GenericImageView};
use lab::Lab;
use std::num::ParseIntError;
use std::convert::TryInto;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point(pub i32, pub i32);

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct GeoCoords(pub f32, pub f32);

#[derive(Debug, PartialEq)]
pub struct GeoCoordsError;

impl GeoCoords {
    pub fn from_str(s: &str) -> Result<Self, GeoCoordsError> {
        let cleaned = s.replace(" ", "");
        let coords:Vec<&str> = cleaned.split(",").collect();
        if coords.len() != 2 {
            return Err(GeoCoordsError);
        }

        let res:Result<Vec<_>, _> = coords.iter().map(|&c| {
            c.parse::<f32>()
        } ).collect();

        if let Ok(cord_nums) = res {
            Ok(Self (cord_nums[0], cord_nums[1]))
        } else {
            return Err(GeoCoordsError);
        }
    }
}

impl std::fmt::Display for GeoCoords {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.0, self.1)
    }
}


const MIN_MARK_DISTANCE: i32 = 30;


fn traverse_path(img: &mut DynamicImage, initial_point: &Point, visited_points: &mut Vec<u8>, path_color: &Lab) -> Vec<Point> {
    let (image_width, image_height) = img.dimensions();

    let mut marker_points: Vec<Point> = Vec::new();

    let mut points_to_visit: Vec<Point> = Vec::new();
    points_to_visit.push(initial_point.clone());
    let mut next_points_to_visit: Vec<Point> = Vec::new();
    let mut distance = 0;

    // let path_color = lab::Lab::from_rgb(&PATH_COLOR_REFERENCE);

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
                        *path_color,
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


pub fn trace(top_left: GeoCoords, bottom_right: GeoCoords, image: &[u8], path_color: &[u8; 3]) -> Vec<GeoCoords> {
    let mut geo_coords: Vec<GeoCoords> = Vec::new();
    let img: DynamicImage = image::load_from_memory(image).unwrap();

    let mut img_dest: DynamicImage = image::load_from_memory(image).unwrap();
    // img_dest.
    let (image_width, image_height) = img.dimensions();

    let path_color_lab = lab::Lab::from_rgb(path_color);


    let mut visited_points: Vec<u8> = Vec::new();
    visited_points.resize((image_width * image_width) as usize, 1);


    'outer: for x in 0..image_width {
        for y in 0..image_height {
            let diff = DE2000::new(
                lab::Lab::from_rgba(&img.get_pixel(x, y).0),
                path_color_lab,
            );
            if diff < 5. && visited_points[(x * image_width as u32 + y) as usize] != 0 {
                for point_screen_coords in traverse_path(&mut img_dest, &Point(x as i32, y as i32), &mut visited_points, &path_color_lab).iter() {
                    let point_geo_cords = GeoCoords(
                        top_left.0 + ((bottom_right.0 - top_left.0) / image_height as f32) * point_screen_coords.1 as f32,
                        top_left.1 + ((bottom_right.1 - top_left.1) / image_width as f32) * point_screen_coords.0 as f32,
                    );


                    geo_coords.push(point_geo_cords);
                };
            }
        }
    }

    return geo_coords;
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_wrong_format() {
        assert_eq!(GeoCoords::from_str("++++"), Err(GeoCoordsError))
    }

    #[test]
    fn ok_coords() {
        assert_eq!(GeoCoords::from_str("12.3456, 12.3457").unwrap(), GeoCoords(12.3456, 12.3457))
    }
}
