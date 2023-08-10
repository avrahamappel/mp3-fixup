// use std::fs;
use std::fs::DirEntry;
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
                .expect("Path {path} didn't have a file name")
                .to_str()
                .expect("Path {path} couldn't be converted to unicode string")
                .to_string(),
        }
    }
}

struct Album {
    name: String,
    tracks: Vec<Track>,
}

impl Album {
    fn new(dir: DirEntry) -> Self {
        println!("{dir:?}");

        let name = dir
            .file_name()
            .to_str()
            .expect("Path {path} couldn't be converted to unicode string")
            .to_string();

        let tracks = dir
            .path()
            .read_dir()
            .unwrap()
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let path = entry.path();
                ["mp3", "m4a"]
                    .contains(
                        &path
                            .extension()
                            .expect("Path {path} doesn't have an extension")
                            .to_str()
                            .expect("Path {path} extension couldn't be converted to unicode"),
                    )
                    .then_some(Track::new(&path))
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
    fn new(path: String) -> Self {
        let path = Path::new(&path);
        let author = path
            .file_name()
            .expect("Path {path} didn't have a file name")
            .to_str()
            .expect("Path {path} couldn't be converted to unicode string")
            .to_string();

        let albums = path
            .read_dir()
            .unwrap()
            .filter_map(Result::ok)
            .filter_map(|entry| {
                entry
                    .file_type()
                    .expect("Couldn't get file type for {entry}")
                    .is_dir()
                    .then_some(Album::new(entry))
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

    let meta = Metadata::new(args.path);

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
