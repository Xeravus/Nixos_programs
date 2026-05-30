use log::{debug, info, error};
use std::process::{self, Command};

use crate::*;

pub enum ListDebug {
    Drives,
    Taildevices,
}

pub fn list_debug(function: &ListDebug) {
    match function {
        ListDebug::Drives => {
            println!("{:?}", get_drives());
        },
        ListDebug::Taildevices => {
            println!("{:?}", get_taildevices());
        },
    }
}

