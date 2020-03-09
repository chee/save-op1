use super::disk::Disk;

pub struct Song<'disk> {
    pub disk: &'disk Disk,
    pub name: String,
    pub slug: String,
    pub artist: String,
}

impl Song<'_> {
    pub fn new<'disklife>(disk: &'disklife Disk, name: &str, artist_name: &str) -> Song<'disklife> {
        Song {
            disk,
            name: name.to_owned(),
            slug: slug::slugify(name),
            artist: artist_name.to_owned(),
        }
    }

    // fn song(&self) -> path::PathBuf {
    //     self.disk.songs.song(self)
    // }

    // fn tape_string(&self) -> String {
    //     format!("{}/{}/tape", self.path, song.slug)
    // }

    // fn tape_track(&self, track: &Track) -> path::PathBuf {
    //     path::PathBuf::from(format!("{}/{}/tape/{}.aif", self.path, song.slug, track))
    // }

    // fn tape(&self) -> path::PathBuf {
    //     path::PathBuf::from(self.tape_string(song))
    // }

    // fn song_exists(&self) -> bool {
    //     path::Path::new(&self.song_string(song)).exists()
    // }

    // fn tape_exists(&self) -> bool {
    //     path::Path::new(&self.tape_string(song)).exists()
    // }

    // fn aif_string(&self) -> String {
    //     format!("{}/{}/{}.aif", self.path, song.slug, song.slug)
    // }

    // fn mp3_string(&self) -> String {
    //     format!("{}/{}/{}.mp3", self.path, song.slug, song.slug)
    // }

    // fn aif(&self) -> path::PathBuf {
    //     path::PathBuf::from(&self.aif_string(song))
    // }

    // fn mp3(&self) -> path::PathBuf {
    //     path::PathBuf::from(&self.mp3_string(song))
    // }
}
