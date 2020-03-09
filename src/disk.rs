use super::operator::Track;
use super::song::Song;
use pbr::{ProgressBar, Units};
use std::fs::{create_dir_all, read_dir, File};
use std::io::{copy, Error, ErrorKind, Result};
use std::path;
use tee_readwrite::TeeWriter;

struct SongsPath {
    path: String,
}

enum SongArg<'life> {
    Slug(&'life str),
    Song(&'life Song<'life>),
}

impl SongArg<'_> {
    fn slug(&self) -> String {
        match self {
            SongArg::Slug(string) => string.to_string(),
            SongArg::Song(song) => song.slug.to_owned(),
        }
    }

    fn _artist(&self) -> Option<String> {
        match self {
            SongArg::Slug(_) => None,
            SongArg::Song(song) => Some(song.artist.to_owned()),
        }
    }

    fn _name(&self) -> Option<String> {
        match self {
            SongArg::Slug(_) => None,
            SongArg::Song(song) => Some(song.name.to_owned()),
        }
    }
}

impl SongsPath {
    fn new(path_name: String) -> SongsPath {
        SongsPath { path: path_name }
    }

    fn song_string(&self, song: &SongArg) -> String {
        format!("{}/{}", self.path, song.slug())
    }

    fn song(&self, song: &SongArg) -> path::PathBuf {
        path::PathBuf::from(self.song_string(song))
    }

    fn tape_string(&self, song: &SongArg) -> String {
        format!("{}/{}/tape", self.path, song.slug())
    }

    fn tape_track(&self, song: &SongArg, track: &Track) -> path::PathBuf {
        path::PathBuf::from(format!("{}/{}/tape/{}.aif", self.path, song.slug(), track))
    }

    fn tape(&self, song: &SongArg) -> path::PathBuf {
        path::PathBuf::from(self.tape_string(song))
    }

    fn _song_exists(&self, song: &SongArg) -> bool {
        path::Path::new(&self.song_string(song)).exists()
    }

    fn tape_exists(&self, song: &SongArg) -> bool {
        path::Path::new(&self.tape_string(song)).exists()
    }

    fn aif_string(&self, song: &SongArg) -> String {
        format!("{}/{}/{}.aif", self.path, song.slug(), song.slug())
    }

    fn mp3_string(&self, song: &SongArg) -> String {
        format!("{}/{}/{}.mp3", self.path, song.slug(), song.slug())
    }

    fn aif(&self, song: &SongArg) -> path::PathBuf {
        path::PathBuf::from(&self.aif_string(song))
    }

    fn _mp3(&self, song: &SongArg) -> path::PathBuf {
        path::PathBuf::from(&self.mp3_string(song))
    }
}

pub struct Disk {
    songs: SongsPath,
    /*
    synth_dir: path::PathBuf,
    drum_dir: path::PathBuf,
    */
}

fn copy_file(source: &path::PathBuf, target: &path::PathBuf) -> Result<()> {
    let mut source = File::open(source)?;
    let bytes = source.metadata()?.len() as u64;
    let mut progress_bar = ProgressBar::new(bytes);
    progress_bar.set_units(Units::Bytes);
    let mut target = File::create(target)?;
    let mut tee = TeeWriter::new(&mut target, &mut progress_bar);
    copy(&mut source, &mut tee)?;
    progress_bar.finish_print("done");
    Ok(())
}

impl Disk {
    fn make_song_dir(&self, song: &Song) -> Result<()> {
        create_dir_all(self.songs.song(&SongArg::Song(song)))
    }

    fn make_tape_dir(&self, song: &Song) -> Result<()> {
        create_dir_all(self.songs.tape(&SongArg::Song(song)))
    }

    fn list_songs(&self) -> Result<Vec<String>> {
        let mut names = vec![];

        for dir in read_dir(&self.songs.path)? {
            let dir = match dir {
                Ok(dir) => dir,
                Err(_) => continue,
            };

            let file_type = match dir.file_type() {
                Ok(ft) => ft,
                Err(_) => continue,
            };

            if file_type.is_dir() {
                &names.push(dir.file_name().to_str().unwrap().to_owned());
            }
        }

        Ok(names)
    }

    pub fn list_tapes(&self) -> Result<Vec<String>> {
        let mut names = vec![];

        for song_name in self.list_songs()? {
            let slug = SongArg::Slug(&song_name);
            if self.songs.tape_exists(&slug) {
                &names.push(song_name.to_owned());
            }
        }

        Ok(names)
    }

    pub fn save_aif(&self, song: &Song, source: &path::PathBuf) -> Result<()> {
        self.make_song_dir(song)?;
        copy_file(source, &self.songs.aif(&SongArg::Song(song)))?;
        Ok(())
    }

    pub fn save_tape(&self, song: &Song, tracks: Vec<&Track>) -> Result<()> {
        self.make_tape_dir(song)?;

        for track in tracks {
            println!("copying {}", track);
            copy_file(
                &path::PathBuf::from(track.path()),
                &self.songs.tape_track(&SongArg::Song(song), track),
            )?;
        }

        Ok(())
    }

    pub fn create_mp3(&self, song: &Song) -> Result<()> {
        std::process::Command::new("ffmpeg")
            .args(&[
                "-i",
                &self.songs.aif_string(&SongArg::Song(song)),
                "-ab",
                "320k",
                &self.songs.mp3_string(&SongArg::Song(song)),
            ])
            .output()?;

        Ok(())
    }

    pub fn tag_mp3(&self, song: &Song) -> Result<()> {
        let mp3 = taglib::File::new(&self.songs.mp3_string(&SongArg::Song(song))).unwrap();
        let mut tag = mp3.tag().unwrap();
        tag.set_title(&song.name);
        tag.set_artist(&song.artist);
        tag.set_comment("large rabbit");
        mp3.save();
        Ok(())
    }

    pub fn upload_mp3(&self, song: &Song) -> Result<()> {
        std::process::Command::new("rsync")
            .args(&[
                "-av",
                &self.songs.mp3_string(&SongArg::Song(song)),
                "snoot:music",
            ])
            .output()?;
        Ok(())
    }

    pub fn _tape_path(&self, slug: &str) -> String {
        self.songs.tape_string(&SongArg::Slug(slug))
    }

    pub fn _track_paths(&self, slug: &str) -> Vec<String> {
        let song = &SongArg::Slug(slug);
        vec![
            format!("{}/track_1.aif", self.songs.tape_string(song)),
            format!("{}/track_2.aif", self.songs.tape_string(song)),
            format!("{}/track_3.aif", self.songs.tape_string(song)),
            format!("{}/track_4.aif", self.songs.tape_string(song)),
        ]
    }

    pub fn new(disk_path: &path::PathBuf) -> Result<Disk> {
        let songs_dir = format!("{}/songs", disk_path.to_str().unwrap());

        if !path::Path::new(&songs_dir).exists() {
            return Err(Error::new(ErrorKind::NotFound, "disk path had no songs"));
        }

        Ok(Disk {
            songs: SongsPath::new(songs_dir),
        })
    }
}
