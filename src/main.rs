use ears::AudioController;
use std::fs;
use std::path;

enum Side {
    A(path::PathBuf),
    B(path::PathBuf),
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Side::A(_) => write!(f, "side_a"),
            Side::B(_) => write!(f, "side_b"),
        }
    }
}

struct Album {
    side_a: Side,
    side_b: Side,
}

impl Album {
    fn check_structure(dir_path: &path::PathBuf) -> bool {
        let exists = |file_name: &str| -> bool {
            path::Path::new::<str>(format!("{}/{}", dir_path.to_str().unwrap(), file_name).as_ref())
                .exists()
        };

        exists("side_a.aif") && exists("side_b.aif")
    }
}

enum Track {
    One(path::PathBuf),
    Two(path::PathBuf),
    Three(path::PathBuf),
    Four(path::PathBuf),
}

impl Track {
    fn path(&self) -> String {
        match self {
            Track::One(path) => path.to_str().unwrap().to_owned(),
            Track::Two(path) => path.to_str().unwrap().to_owned(),
            Track::Three(path) => path.to_str().unwrap().to_owned(),
            Track::Four(path) => path.to_str().unwrap().to_owned(),
        }
    }
}

impl std::fmt::Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Track::One(_) => write!(f, "track_1"),
            Track::Two(_) => write!(f, "track_2"),
            Track::Three(_) => write!(f, "track_3"),
            Track::Four(_) => write!(f, "track_4"),
        }
    }
}

struct Tape {
    track_1: Track,
    track_2: Track,
    track_3: Track,
    track_4: Track,
}

impl Tape {
    fn new(t1: path::PathBuf, t2: path::PathBuf, t3: path::PathBuf, t4: path::PathBuf) -> Tape {
        Tape {
            track_1: Track::One(t1),
            track_2: Track::Two(t2),
            track_3: Track::Three(t3),
            track_4: Track::Four(t4),
        }
    }

    fn tracks(&self) -> Vec<&Track> {
        return vec![&self.track_1, &self.track_2, &self.track_3, &self.track_4];
    }

    fn check_structure(dir_path: &path::PathBuf) -> bool {
        let exists = |file_name: &str| -> bool {
            path::Path::new::<str>(format!("{}/{}", dir_path.to_str().unwrap(), file_name).as_ref())
                .exists()
        };

        exists("track_1.aif")
            && exists("track_2.aif")
            && exists("track_3.aif")
            && exists("track_4.aif")
    }
}

struct Operator {
    album: Album,
    // drum: Drum,
    // synth: Synth,
    tape: Tape,
}

impl Operator {
    fn check_structure(dir_path: &path::PathBuf) -> std::io::Result<bool> {
        let mut has_album = false;
        let mut has_tape = false;
        let mut has_drum = false;
        let mut has_synth = false;
        for entry in fs::read_dir(dir_path)? {
            let entry: fs::DirEntry = entry?;
            let file_path = entry.path();
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                match entry.file_name().to_str().unwrap() {
                    "album" => has_album = Album::check_structure(&file_path),
                    "drum" => has_drum = true,
                    "tape" => has_tape = Tape::check_structure(&file_path),
                    "synth" => has_synth = true,
                    _ => {}
                };
            }
        }
        return Ok(has_album && has_tape && has_drum && has_synth);
    }

    fn new(mount_path: &path::PathBuf) -> std::io::Result<Operator> {
        let is_valid = Operator::check_structure(mount_path)?;
        let get_path = |suffix: &str| -> path::PathBuf {
            path::PathBuf::from(format!("{}/{}", mount_path.to_str().unwrap(), suffix))
        };
        if is_valid {
            Ok(Operator {
                album: Album {
                    side_a: Side::A(get_path("album/side_a.aif")),
                    side_b: Side::B(get_path("album/side_b.aif")),
                },
                tape: Tape::new(
                    get_path("tape/track_1.aif"),
                    get_path("tape/track_2.aif"),
                    get_path("tape/track_3.aif"),
                    get_path("tape/track_4.aif"),
                ),
            })
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "oh no!",
            ))
        }
    }
}

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
        .item("exit")
        .interact()
        .unwrap()
    {
        0 => Ok(&op1.album.side_a),
        1 => Ok(&op1.album.side_b),
        2 => std::process::exit(0),
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
    let side_path = match side {
        Side::A(path) => path,
        Side::B(path) => path,
    };

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

fn create_song(
    op1: &Operator,
    source: &path::PathBuf,
    music_dir: &path::PathBuf,
    artist_name: &str,
    song_name: &str,
    with_tape: bool,
) -> std::io::Result<()> {
    let slug = slug::slugify(song_name);
    let song_dir_name = format!("{}/{}", music_dir.to_str().unwrap(), slug);
    let aif_file_name = format!("{}/{}/{}.aif", music_dir.to_str().unwrap(), slug, slug);
    let mp3_file_name = format!("{}/{}/{}.mp3", music_dir.to_str().unwrap(), slug, slug);
    let song_dir = std::path::Path::new(&song_dir_name);
    fs::create_dir(&song_dir).unwrap_or_default();

    println!("copying aif");
    fs::copy(&source, &aif_file_name)?;

    if with_tape {
        let tape_dir_name = format!("{}/{}/tape", music_dir.to_str().unwrap(), slug);
        fs::create_dir(&tape_dir_name).unwrap_or_default();

        for track in op1.tape.tracks() {
            println!("copying {}", track);
            let track_path = format!("{}/{}", tape_dir_name, track);
            fs::copy(track.path(), track_path)?;
        }
    }

    println!("creating mp3");
    std::process::Command::new("ffmpeg")
        .args(&["-i", &aif_file_name, "-ab", "320k", &mp3_file_name])
        .output()?;

    println!("tagging mp3");

    let mp3 = taglib::File::new(&mp3_file_name).unwrap();
    let mut tag = mp3.tag().unwrap();
    tag.set_title(song_name);
    tag.set_artist(artist_name);
    tag.set_comment("large rabbit");
    mp3.save();

    println!("{} - {}", artist_name, song_name);
    println!("{}", &mp3_file_name);

    if ask("upload?") {
        std::process::Command::new("rsync")
            .args(&["-av", &mp3_file_name, "snoot:music"])
            .output()?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let op1_dir = path::PathBuf::from("/media/chee/54FF-1FEE");
    let music_dir = path::PathBuf::from("/home/chee/Documents/electronic-music/op1/songs");
    let artist_name = "quiet party";

    let op1 = Operator::new(&op1_dir)?;

    loop {
        let side = choose_side(&op1)?;
        let side_path = match side {
            Side::A(path) => path,
            Side::B(path) => path,
        };
        match ask_about_side(&side)? {
            SideChoice::Nothing => {}
            SideChoice::Save(name) => {
                create_song(&op1, side_path, &music_dir, artist_name, &name, false)?
            }
            SideChoice::SaveWithTape(name) => {
                create_song(&op1, side_path, &music_dir, artist_name, &name, true)?
            }
        }
    }
}
