use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use sha1::{Digest, Sha1};
use flate2::write::ZlibEncoder;
use flate2::Compression;

pub fn handle(file_path : &str){
    let path = Path::new(file_path);
    let mut f = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file {}: {}", file_path, e);
            return;
        }
    };

    let mut contents = Vec::new();
    match f.read_to_end(&mut contents) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            return;
        }
    }

    //Build blob string
    let header = format!("blob {}\0", contents.len());
    let mut blob = header.into_bytes();
    blob.extend(&contents);

    //Split hash path
    let hash = Sha1::digest(&blob);
    let hex_hash = hex::encode(hash);

    //Split hash path
    let (dir, file) = hex_hash.split_at(2);
    let object_dir = Path::new(".xlit/objects").join(dir);
    fs::create_dir_all(&object_dir).unwrap();

    let object_path = object_dir.join(file);
    if object_path.exists(){
        println!("{hex_hash}");
        return;
    }

    //Compress and write the object
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&blob).unwrap();
    let compressed = encoder.finish().unwrap();

    let mut out = fs::File::create(&object_path).unwrap();
    out.write_all(&compressed).unwrap();

    println!("{hex_hash}");  
}