#![windows_subsystem = "windows"]

use wallpaper;
use std::fs;
use rand::{self, Rng};
use std::path::Path;

fn main() {
    let picture_path = match fs::read_to_string("path.txt") {
        Ok(path) => path,
        Err(error) => {panic!("{:?}", error);},
    };

    let picture_list_raw = fs::read_dir(&Path::new(picture_path.as_str())).unwrap();

    let picture_list =
       picture_list_raw.filter_map(|entry| {
           entry.ok().and_then(|e|
               e.path()
                   .to_str().map(|s| String::from(s))
           )
       }).collect::<Vec<String>>();

    let mut new_background = &picture_list[rand::thread_rng().gen_range(0..picture_list.len())];
    while new_background == &*wallpaper::get().unwrap() {
       new_background = &picture_list[rand::thread_rng().gen_range(0..picture_list.len())];
    }
    wallpaper::set_from_path(&*new_background).unwrap();
}