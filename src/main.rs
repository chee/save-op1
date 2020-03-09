use ears::AudioController;
use std::path;
mod copy;
mod disk;
mod operator;
mod song;

use disk::Disk;
use operator::{Operator, Side};

fn ask(question: &str) -> bool {
    dialoguer::Confirmation::new()
        .with_text(question)
        .interact()
        .unwrap_or(false)
}

fn choose_side(op1: &Operator) -> std::io::Result<&Side> {
    match dialoguer::Select::new()
        .with_prompt("Choose a side")
        .items(&[&op1.album.side_a, &op1.album.side_b])
        .item("back")
        .interact()
        .unwrap()
    {
        0 => Ok(&op1.album.side_a),
        1 => Ok(&op1.album.side_b),
        2 => Ok(&Side::Neither),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "wtf?",
        )),
    }
}

enum SideChoice {
    SaveWithTape(String),
    Save(String),
    Nothing,
}

fn save() -> std::io::Result<SideChoice> {
    println!("(name will be slugified for filename)");
    let name: String = dialoguer::Input::new().with_prompt("name").interact()?;
    if ask("bring tape?") {
        Ok(SideChoice::SaveWithTape(name))
    } else {
        Ok(SideChoice::Save(name))
    }
}

fn preview(side: &Side) -> std::io::Result<()> {
    let side_path = side.path().unwrap();

    let mut music = ears::Music::new(side_path.to_str().unwrap()).unwrap();

    music.play();

    println!("playing!");

    loop {
        if ask("all good?") {
            break;
        }
    }
    Ok(())
}

fn ask_about_side(side: &Side) -> std::io::Result<SideChoice> {
    match dialoguer::Select::new()
        .with_prompt(format!("{}", side).as_ref())
        .items(&["save", "preview", "back"])
        .interact()
        .unwrap()
    {
        0 => save(),
        1 => {
            preview(side)?;
            ask_about_side(side)
        }
        2 => Ok(SideChoice::Nothing),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "how did you do that?",
        )),
    }
}

#[derive(PartialEq)]
enum TapeOption {
    WithTape,
    WithoutTape,
}

fn create_song(
    op1: &Operator,
    disk: &Disk,
    source: &path::PathBuf,
    artist_name: &str,
    song_name: &str,
    tape_option: TapeOption,
) -> std::io::Result<()> {
    let song = song::Song::new(&disk, song_name, artist_name);

    println!("copying aif");
    disk.save_aif(&song, source)?;

    println!("creating mp3");
    disk.create_mp3(&song)?;

    println!("tagging mp3");
    disk.tag_mp3(&song)?;

    if tape_option == TapeOption::WithTape {
        println!("copying tape");
        disk.save_tape(&song, op1.tape.tracks())?;
    }

    if ask("upload?") {
        disk.upload_mp3(&song)?;
    }

    Ok(())
}

fn album_menu(op1: &Operator, disk: &Disk) -> std::io::Result<()> {
    let artist_name = "quiet party";
    let side = choose_side(&op1)?;

    let side_path = match side.path() {
        Some(path) => path,
        None => return Ok(()),
    };

    match ask_about_side(&side)? {
        SideChoice::Nothing => {}
        SideChoice::Save(name) => create_song(
            &op1,
            &disk,
            side_path,
            artist_name,
            &name,
            TapeOption::WithoutTape,
        )?,
        SideChoice::SaveWithTape(name) => create_song(
            &op1,
            &disk,
            side_path,
            artist_name,
            &name,
            TapeOption::WithTape,
        )?,
    }
    Ok(())
}

enum Menu {
    Album,
    Tape,
}

fn main_menu() -> std::io::Result<Menu> {
    match dialoguer::Select::new()
        .items(&["albums", "tapes", "exit" /*"synths", "drums"*/])
        .interact()
        .unwrap()
    {
        0 => Ok(Menu::Album),
        1 => Ok(Menu::Tape),
        2 => std::process::exit(0),
        _ => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "sorry?")),
    }
}

fn load_tape(op1: &Operator, disk: &Disk, slug: &str) -> std::io::Result<()> {
    op1.save_tape(disk.track_paths(slug))
}

fn load_tape_menu(op1: &Operator, disk: &Disk, slug: &str) -> std::io::Result<()> {
    match dialoguer::Select::new()
        .with_prompt(slug)
        .items(&["write to op-1", "back"])
        .interact()
        .unwrap()
    {
        0 => load_tape(op1, disk, slug),
        1 => Ok(()),
        _ => Ok(()),
    }
}

fn load_tapes_menu(op1: &Operator, disk: &Disk) -> std::io::Result<()> {
    let tapes = disk.list_tapes()?;
    let choice = dialoguer::Select::new().items(&tapes).interact().unwrap();
    if let Some(tape) = tapes.get(choice) {
        load_tape_menu(op1, disk, tape)?;
    }
    Ok(())
}

fn save_tape(op1: &Operator, disk: &Disk) -> std::io::Result<()> {
    let name: String = dialoguer::Input::new().with_prompt("name").interact()?;
    let song = song::Song::new(&disk, &name, "quiet party");
    disk.save_tape(&song, op1.tape.tracks())
}

fn tape_menu(op1: &Operator, disk: &Disk) -> std::io::Result<()> {
    match dialoguer::Select::new()
        .items(&["save to disk", "load to op-1", "back"])
        .interact()
        .unwrap()
    {
        0 => save_tape(op1, disk),
        1 => load_tapes_menu(op1, disk),
        2 => Ok(()),
        _ => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "sorry?")),
    }?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let op1_dir = path::PathBuf::from("/media/chee/54FF-1FEE");
    // let op1_dir = path::PathBuf::from("/home/chee/Projects/save-op1/fake-op1");
    let music_dir = path::PathBuf::from("/home/chee/Documents/electronic-music/op1");

    let op1 = Operator::new(&op1_dir)?;
    let disk = Disk::new(&music_dir)?;

    loop {
        match main_menu()? {
            Menu::Album => album_menu(&op1, &disk),
            Menu::Tape => tape_menu(&op1, &disk),
        }?;

        if !ask("would you like do something else?") {
            std::process::exit(0)
        }
    }
}
