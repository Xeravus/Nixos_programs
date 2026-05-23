use crate::{AppStatusReadable, AppStatusJson};

use std::time::{Instant, SystemTime, UNIX_EPOCH, Duration};
use humantime::format_duration;
use colored::Colorize;

#[derive(Debug)]
pub enum Format {
    Json(LayoutStyle),
    Readable(LayoutStyle),
} 

#[derive(Debug)]
pub enum LayoutStyle {
    Compact,
    Extended,
}

pub fn format_time(input: u64) -> String {
    let lesbar = format_duration(Duration::from_secs(input)).to_string();
    lesbar
}

pub fn take_input(json: bool, readable: bool, compact: bool, extended: bool) -> Format {
    if json && compact {
        Format::Json(LayoutStyle::Compact)
    } else if json && extended {
        Format::Json(LayoutStyle::Extended)
    } else if readable && compact {
        Format::Readable(LayoutStyle::Compact)
    } else if readable && extended {
        Format::Readable(LayoutStyle::Extended)
    } else {
        Format::Json(LayoutStyle::Compact)
    }
}

pub fn format_output(input: AppStatusJson, format: &Format, color_index: Option<usize>) {
    match &format { 
        Format::Json(LayoutStyle::Compact) => {
            let json_output = serde_json::to_string(&input)
                .expect("Konnte Struct nicht in JSON umwandeln!");
            println!("{}", json_output);
        },
        Format::Json(LayoutStyle::Extended) => {
            let json_output = serde_json::to_string(&input)
                .expect("Konnte Struct nicht in JSON umwandeln!");
            println!("{}", json_output);
        },
        Format::Readable(LayoutStyle::Compact) => {
            let status = AppStatusReadable {
                app: input.app,
                level: input.level,
                progress_percent: input.progress_percent,
                time: format_time(input.total_seconds),
            };
            let (app_c, lvl_c, prog_c, time_c) = match color_index {
                None => ( 
                    format!("{}", status.app).white(),
                    format!("{}", status.level).white(),
                    format!("{}", status.progress_percent).white(),
                    format!("{}", status.time).white(),
                ),
                Some(i) if i % 2 == 0 => (
                    format!("{}", status.app).cyan(),
                    format!("{}", status.level).cyan(),
                    format!("{}", status.progress_percent).cyan(),
                    format!("{}", status.time).cyan(),
                ),
                Some(_) => (
                    format!("{}", status.app).blue(),
                    format!("{}", status.level).blue(),
                    format!("{}", status.progress_percent).blue(),
                    format!("{}", status.time).blue(),
                ),
            };
            println!("{} | {} | {}% | {}", app_c, lvl_c, prog_c, time_c);
        },
        Format::Readable(LayoutStyle::Extended) => {
            let status = AppStatusReadable {
                app: input.app,
                level: input.level,
                progress_percent: input.progress_percent,
                time: format_time(input.total_seconds),
            };
            let (app_c, lvl_c, prog_c, time_c) = match color_index {
                None => ( 
                    format!("{}", status.app).white(),
                    format!("{}", status.level).white(),
                    format!("{}", status.progress_percent).white(),
                    format!("{}", status.time).white(),
                ),
                Some(i) if i % 2 == 0 => (
                    format!("{}", status.app).cyan(),
                    format!("{}", status.level).cyan(),
                    format!("{}", status.progress_percent).cyan(),
                    format!("{}", status.time).cyan(),
                ),
                Some(_) => (
                    format!("{}", status.app).blue(),
                    format!("{}", status.level).blue(),
                    format!("{}", status.progress_percent).blue(),
                    format!("{}", status.time).blue(),
                ),
            };
            println!("{:<28} | {:>2} | {:>2}% | {}", app_c, lvl_c, prog_c, time_c);
        },
    }
}
