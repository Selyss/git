use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::io::{Error, Write};
use std::path::PathBuf;
use std::{error, fs};

enum ObjectType {
    Commit,
    Tree,
    Blob,
}

fn read_file(path: PathBuf) -> String {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    return contents;
}

fn write_file(path: PathBuf, data: &str) {
    fs::write(path, data).expect("Unable to write to file");
}

fn init(repo_name: &str) -> Result<(), Error> {
    // create dir for repo and initialize .git/
    fs::create_dir(repo_name)?;

    // create .git/ folders
    let primary_git_dir = repo_name.to_owned() + "/.git/";
    fs::create_dir(&primary_git_dir)?;

    let git_dirs: [&str; 3] = ["/objects/", "/refs/", "/refs/heads/"];
    for dir in git_dirs {
        match fs::create_dir(primary_git_dir.clone() + dir) {
            Ok(_) => println!("Directory '{}' created successfully", dir),
            Err(e) => return Err(e),
        }
    }
    let head = PathBuf::from(primary_git_dir + "/HEAD");
    write_file(head, "ref: refs/heads/main");
    Ok(())
}

fn hash_object(data: Vec<u8>, obj_type: &str, write: bool) -> Result<String, Error> {
    // this entire function is basically an error
    let header = format!("{} {}", obj_type, data.len()).into_bytes();
    let all_data = [header, b"\x00".to_vec(), data].concat(); // FIXME: What.!

    let mut hasher = Sha1::new();
    hasher.update(&all_data);
    let result = hasher.finalize();

    if write {
        let path_str = String::from_utf8([b".git/objects/", &result[..2], &result[2..]].concat());
        let path_str = path_str.unwrap();
        let path = PathBuf::from(&path_str);

        if !path.exists() {
            fs::create_dir_all(path)?;
            let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
            e.write_all(&all_data)?;
            let compressed_data = e.finish().unwrap();

            write_file(path, std::str::from_utf8(&compressed_data).unwrap());
        }
    }

    Ok(format!("{:x}", result))
}

fn find_object(sha1_prefix: &str) -> Result<PathBuf, Error> {
    if sha1_prefix.len() < 2 {
        return Err(Error::new(
            // yes?
            std::io::ErrorKind::InvalidData,
            "hash prefix must be 2 or more characters".into(),
        ));
    }
    let obj_dir = PathBuf::from(".git/objects{}".to_owned() + &sha1_prefix[..2]);
    let rest = &sha1_prefix[2..];
    let objects;

    for name in fs::read_dir(obj_dir) {}
}

fn read_index() -> Vec<String> {}

fn main() {
    match init("test_repo") {
        Ok(_) => println!("Repository initialized successfully"),
        Err(e) => eprintln!("Error initializing repository: {}", e),
    }
}
