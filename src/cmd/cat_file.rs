use std::fs;
use std::io::Read;
use std::path::Path;
use flate2::read::ZlibDecoder;

pub fn handle(hash: &str) {
    let (dir, file) = hash.split_at(2);
    let object_path = Path::new(".xlit/objects").join(dir).join(file);

    let file = fs::File::open(&object_path)
        .unwrap_or_else(|_| panic!("Object {} not found", hash));

    let mut decoder = ZlibDecoder::new(file);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).unwrap();

    // Strip header ("blob <len>\0") — find first null byte
    if let Some(pos) = decompressed.iter().position(|&b| b == 0) {
        let content = &decompressed[pos + 1..];
        println!("{}", String::from_utf8_lossy(content));
    } else {
        eprintln!("Invalid object format");
    }
}
