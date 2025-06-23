use std::fs;
use std::io::Write;
use std::path::Path;

pub fn handle(){
    let xlit_dir = Path::new(".xlit");

    if xlit_dir.exists(){
        println!(".xlit repo already exists");
        return;
    }

    //core directories
    let core_dirs = [
        "objects/info",
        "objects/pack",
        "refs/heads",
        "refs/tags",
    ];

    for dir in core_dirs.iter(){
        fs::create_dir_all(xlit_dir.join(dir)).unwrap();
    }

    //HEAD pointing to master
    let mut head = fs::File::create(xlit_dir.join("HEAD")).unwrap();
    head.write_all(b"ref: refs/heads/master\n").unwrap();

    //config file
    let mut config = fs::File::create(xlit_dir.join("config")).unwrap();
    config.write_all(b"[core]\n\trepositoryformatversion = 0\n\tfilemode = true\n\tbare = false\n").unwrap();

    println!("Initialized empty xlit repo in {}", xlit_dir.display());
}