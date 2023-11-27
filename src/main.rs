use image::GenericImageView;
use std::fs;

pub mod walk_dirs;

fn main() {
    let image_list = walk_dirs::walk_dir("/media/pipi/USB01/DeDuped1_backup".to_string());
    for img in image_list {
        let _aspect_ratio = filter_by_aspect_ratio(img);
    }
}

pub fn mv_to_banner_folder(apath: String) {
    let fparts = apath.split("/").collect::<Vec<&str>>();
    let filename = fparts.last().unwrap().replace(" ", "_");
    let addr = "/media/pipi/USB01/Banners/".to_string() + &filename;
    println!("addr: {}\n apath: {}\n", addr, apath);
    match fs::rename(&apath, &addr) {
        Ok(_) => println!("Moved: {}", addr),
        Err(e) => println!("Error: {}", e),
    };
}

pub fn mv_to_square_folder(apath: String) {
    let fparts = apath.split("/").collect::<Vec<&str>>();
    let filename = fparts.last().unwrap().replace(" ", "_");
    let addr = "/media/pipi/USB01/Square/".to_string() + &filename;
    println!("addr: {}\n apath: {}\n", addr, apath);
    match fs::rename(&apath, &addr) {
        Ok(_) => println!("Moved: {}", addr),
        Err(e) => println!("Error: {}", e),
    };
}

fn filter_by_aspect_ratio(apath: String) -> f64{
    let image = image::open(apath.clone()).expect(&apath);
    let (width, height) = image.dimensions();
    let oldwidth = width.clone() as f64;
    let oldheight = height.clone() as f64;
    let aspect_ratio = oldwidth / oldheight;
    // if oldwidth > oldheight {
    //     println!("landscape");
    // } else if oldwidth < oldheight {
    //     print!("portrait")
    // } else {
    //     println!("square")
    // };

    if aspect_ratio > 1.9 {
        let _mv_banner_image = mv_to_banner_folder(apath.clone());
        println!("Banner filename: {}\n aspect_ratio: {}\n", apath, aspect_ratio);
    };
    if aspect_ratio == 1.0 {
        let _mv_square_image = mv_to_square_folder(apath.clone());
        println!("Square filename: {}\n aspect_ratio: {}\n", apath, aspect_ratio);
    };



    aspect_ratio
}