use crate::parsers::*;
use crate::generator::*;

use std::fs;

pub fn cl_themedir() {
    for i in pars_themes() {
        let folder_path: String = format!("{}/{}/", gen_path(5), i);
        fs::remove_dir_all(&folder_path);
        println!("Gelöschter Ordner: {}", i);
    }
}

