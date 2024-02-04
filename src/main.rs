use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;

fn init(repo_name: &str) -> Result<(), std::io::Error> {
    // create dir for repo and initialize .git/
    if let Err(e) = fs::create_dir(repo_name) {
        return Err(e);
    }

    // create .git/ folders
    let primary_git_dir = repo_name.to_owned() + "/.git/";
    if let Err(e) = fs::create_dir(&primary_git_dir) {
        return Err(e);
    }

    let git_dirs: [&str; 3] = ["/objects/", "/refs/", "/refs/heads/"];
    for dir in git_dirs {
        match fs::create_dir(primary_git_dir.clone() + dir) {
            Ok(_) => println!("Directory '{}' created successfully", dir),
            Err(e) => return Err(e),
        }
    }
    let mut head = fs::File::create(primary_git_dir + "/HEAD")?;
    if let Err(e) = head.write_all(b"ref: refs/heads/main") {
        return Err(e);
    }

    Ok(())
}

fn hash_object(data: Vec<u8>, obj_type: &str, write: bool) -> Result<String, std::io::Error> {
    let header = format!("{} {}", obj_type, data.len()).into_bytes();
    let all_data = [header, b"\x00".to_vec(), data].concat(); // FIXME: What.!

    let mut hasher = Sha1::new();
    hasher.update(&all_data);
    let result = hasher.finalize();

    if write == true {
        let path_str = String::from_utf8([b".git/objects/", &result[..2], &result[2..]].concat());
        let path_str = path_str.unwrap();
        let path = Path::new(&path_str);

        if !path.exists() {
            if let Err(e) = fs::create_dir_all(path) {
                return Err(e);
            }
            let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
            if let Err(e) = e.write_all(&all_data) {
                return Err(e);
            }
            let compressed_data = e.finish().unwrap();

            let file = fs::File::create(path);
            if let Err(e) = file.unwrap().write_all(&compressed_data) {
                return Err(e);
            }
        }
    }

    return Ok(format!("{:x}", result));
}

fn main() {
    match init("test_repo") {
        Ok(_) => println!("Repository initialized successfully"),
        Err(e) => eprintln!("Error initializing repository: {}", e),
    }
}
