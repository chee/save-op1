use pbr::{ProgressBar, Units};
use std::fs::File;
use std::io::{copy, Result, Write};
use std::path;
use tee_readwrite::TeeWriter;

pub fn copy_file(source: &path::PathBuf, target: &path::PathBuf) -> Result<()> {
    let mut source = File::open(source)?;
    let bytes = source.metadata()?.len() as u64;
    let mut progress_bar = ProgressBar::new(bytes);
    progress_bar.set_units(Units::Bytes);
    let mut target = File::create(target)?;
    let mut tee = TeeWriter::new(&mut target, &mut progress_bar);
    copy(&mut source, &mut tee)?;
    progress_bar.finish_print("yay!");
    Ok(())
}
