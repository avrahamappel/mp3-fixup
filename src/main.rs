// use std::fs;
// use std::fs::DirEntry;
use std::path::Path;

use clap::Parser;
// use ffmpeg_next as f;

#[derive(Parser, Debug)]
struct Args {
    /// The path to the directory containing mp3/m4a files
    path: String,

    /// Print what would be done
    #[arg(short, long)]
    dry_run: bool,

    /// Modify files in place
    #[arg(short, long)]
    in_place: bool,
}

// #[derive(Debug)]
// enum Error {
//     PathNotFound,
// }

struct Track {
    name: String,
    // order: u16,
}

impl Track {
    fn new(path: &Path) -> Self {
        Self {
            name: path
                .file_name()
                .expect("Path didn't have a file name")
                .to_str()
                .expect("Path couldn't be converted to unicode string")
                .to_string(),
        }
    }
}

struct Album {
    name: String,
    tracks: Vec<Track>,
}

impl Album {
    fn new(path: &Path) -> Self {
        let name = path
            .file_name()
            .expect("Path didn't have a file name")
            .to_str()
            .expect("Path couldn't be converted to unicode string")
            .to_string();

        let tracks = path
            .read_dir()
            .expect("Path wasn't a directory")
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let path = entry.path();

                // println!("{path:?}");

                match path.extension() {
                    None => None,
                    Some(ext) => ["mp3", "m4a"]
                        .contains(
                            &ext.to_str()
                                .expect("Path extension couldn't be converted to unicode"),
                        )
                        .then(|| Track::new(&path)),
                }
            })
            .collect();

        Self { name, tracks }
    }
}

struct Metadata {
    author: String,
    albums: Vec<Album>,
}

impl Metadata {
    fn new(path: &str) -> Self {
        let path = Path::new(path);
        let author = path
            .file_name()
            .expect("Path didn't have a file name")
            .to_str()
            .expect("Path couldn't be converted to unicode string")
            .to_string();

        let albums = path
            .read_dir()
            .unwrap()
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let path = entry.path();

                // println!("{path:?}");

                path.is_dir().then(|| Album::new(&path))
            })
            .collect();

        Self { author, albums }
    }
}

fn main() {
    let args = Args::parse();

    if args.dry_run {
        println!("This is a dry run");
    }

    println!("Path: {}", args.path);

    let meta = Metadata::new(&args.path);

    for album in meta.albums {
        for track in album.tracks {
            println!(
                "AUTHOR: {}, ALBUM: {}, TRACK: {}",
                meta.author, album.name, track.name
            );
        }
    }

    // f::init();
}
