extern crate plist;

mod folder_organizer;
mod get_froms;

use std::env;
use std::fs::create_dir_all;

use anyhow::Result;
use log::{self, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use crate::folder_organizer::FolderOrganizer;

const IGNORE_FILES: [&str; 1] = [".dialog.lock"];

fn init_logger() -> Result<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} {l} - {m}\n",
        )))
        .build("folder_organizer/logs")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Debug),
        )?;

    log4rs::init_config(config)?;

    Ok(())
}

fn set_current_dir() -> Result<()> {
    let a = env::current_exe()?;
    env::set_current_dir(a.parent().unwrap())?;
    Ok(())
}
fn main() {
    if let Err(exec_err) = entrypoint() {
        log::error!("Failed to execute {}", exec_err);
    }
}

fn entrypoint() -> Result<()> {
    set_current_dir()?;
    create_dir()?;
    init_logger()?;

    log::info!("Starting new session.");
    let mut organizer = FolderOrganizer::new();

    let files = get_files();
    match organizer.organize_files(files) {
        Ok(_) => {
            if let Err(save_err) = organizer.save_data() {
                log::error!("Failed to save organizer data: {}", save_err);
            }
        }
        Err(err) => log::error!("Encountered error while organizing! {}", err),
    }

    Ok(())
}

fn create_dir() -> Result<()> {
    create_dir_all("folder_organizer")?;
    Ok(())
}
fn get_files() -> Vec<String> {
    env::args()
        .into_iter()
        .skip(1)
        .filter(|arg| IGNORE_FILES.iter().position(|&x| x == arg).is_none())
        .collect()
}
