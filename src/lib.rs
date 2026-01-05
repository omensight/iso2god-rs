pub mod converter;
pub mod executable;
pub mod game_list;
pub mod god;
pub mod iso;

use anyhow::Error;
use converter::{Cli, TrimMode};
use std::path::PathBuf;

pub fn convert_iso_to_god(
    source_iso: PathBuf,
    dest_dir: PathBuf,
    game_title: String,
    dry_run: bool,
    num_threads: usize,
    trim: TrimMode,
) -> Result<(), Error> {
    let args = Cli {
        source_iso,
        dest_dir,
        dry_run,
        game_title: Some(game_title),
        num_threads,
        trim: Some(trim),
    };
    converter::convert_iso_to_god(args)
}
