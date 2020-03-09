use super::copy::copy_file;
use std::fs;
use std::path;

#[derive(PartialEq)]
pub enum Side {
    A(path::PathBuf),
    B(path::PathBuf),
    Neither,
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Side::A(_) => write!(f, "side_a"),
            Side::B(_) => write!(f, "side_b"),
            Side::Neither => write!(f, "NO SIDE"),
        }
    }
}

impl Side {
    pub fn path(&self) -> Option<&path::PathBuf> {
        match self {
            Side::A(path) => Some(path),
            Side::B(path) => Some(path),
            Side::Neither => None,
        }
    }
}

pub struct Album {
    pub side_a: Side,
    pub side_b: Side,
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

pub enum Track {
    One(path::PathBuf),
    Two(path::PathBuf),
    Three(path::PathBuf),
    Four(path::PathBuf),
}

impl Track {
    pub fn path(&self) -> String {
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

pub struct Tape {
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

    pub fn save(&self, tracks: Vec<String>) -> std::io::Result<()> {
        // lol this can't be right
        let disk_track_1 = path::PathBuf::from(tracks.get(0).unwrap());
        let disk_track_2 = path::PathBuf::from(tracks.get(1).unwrap());
        let disk_track_3 = path::PathBuf::from(tracks.get(2).unwrap());
        let disk_track_4 = path::PathBuf::from(tracks.get(3).unwrap());

        let op1_track_1 = path::PathBuf::from(self.track_1.path());
        let op1_track_2 = path::PathBuf::from(self.track_2.path());
        let op1_track_3 = path::PathBuf::from(self.track_3.path());
        let op1_track_4 = path::PathBuf::from(self.track_4.path());

        println!("writing track_1 to op1");
        copy_file(&disk_track_1, &op1_track_1)?;
        println!("writing track_2 to op1");
        copy_file(&disk_track_2, &op1_track_2)?;
        println!("writing track_3 to op1");
        copy_file(&disk_track_3, &op1_track_3)?;
        println!("writing track_4 to op1");
        copy_file(&disk_track_4, &op1_track_4)?;
        Ok(())
    }

    pub fn tracks(&self) -> Vec<&Track> {
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

pub struct Operator {
    pub album: Album,
    // pub drum: Drum,
    // pub synth: Synth,
    pub tape: Tape,
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

    pub fn save_tape(&self, tracks: Vec<String>) -> std::io::Result<()> {
        self.tape.save(tracks)
    }

    pub fn new(mount_path: &path::PathBuf) -> std::io::Result<Operator> {
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
