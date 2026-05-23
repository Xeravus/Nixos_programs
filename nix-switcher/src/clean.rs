use crate::parsers::*;
use crate::generator::*;

use std::fs;
use std::path::*; 


pub fn cl_themedir() {
    for i in pars_themes() {
        let folder_path: String = PathBuf::from(gen_path(PathType::Themes)).join(&i).to_str().unwrap().to_string();
        let _ = fs::remove_dir_all(&folder_path);
        println!("Gelöschter Ordner: {}", i);
    }
}

