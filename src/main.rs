// struct Point {
//     x: f32,
//     y: f32,
// }

// use std::io;
use std::path::{Path, PathBuf};

fn main() {
    let path = Path::new("D:\\NVR\\192.168.0.144_0012155fc97a");

    println!("Folder Size in KB: {}", get_folder_size(path));

    let folder_size = get_folder_size(path); // Convert to KB

    if folder_size > 20_000_000 {
        println!("yikes");
        process_folders(path);
    } else {
        println!("good 2 go");
    }

}

///Process the given folder to remove files
fn process_folders(folder: &Path) {
    let mut size_diff = get_folder_size(folder) - 20_000_000;

    let mut date_folders: Vec<_> = Vec::new();

    for path in folder.read_dir().expect("Unable to get directory contents") {
        let path = path.expect("Unable to get path");
        if path.metadata().expect("Unable to get metadata").is_dir() {
            let name = path.file_name();
            date_folders.push(path)
        }
    }

    date_folders.sort_by_key(|dir| dir.path());

    for name in date_folders {
        let size = get_folder_size(name.path().as_path());
        println!("Folder: {}, Size: {} KB", name.file_name().to_str().unwrap(), size);

        if size < size_diff {
            delete_folder(name.path().as_path());
            println!("{}, {}", size_diff, size);
            size_diff -= size;
        } else {
            size_diff -= delete_by_folder_content(folder, size_diff);
            println!("{}", size_diff);
            return;
        }
    }
}

///Delete entire folder
fn delete_folder(folder: &Path) {
    println!("Test Deleting {}", folder.file_name().unwrap().to_str().unwrap())
}

///Delete folder files until size is under limit and returns delta file size
fn delete_by_folder_content(folder: &Path, size_diff: u64) -> u64 {
    let size = get_folder_size(folder);
    let mut deleted_size: u64 = 0;
    let contents = folder_walk(folder);
    for file in contents {
        println!("File: {}, Size: {} KB", file.to_str().unwrap(), file.metadata().unwrap().len() / 1_000);

        let file_size = file.metadata().unwrap().len() / 1_000;

        if file_size < size_diff - deleted_size {
            println!("fake deleting");
            deleted_size += file_size;
        } else {
            break;
        }
    }
    deleted_size
}

///Return Vec<PathBuf> of all files inside the given folder recursively
fn folder_walk(folder: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<_> = Vec::new();
    for new_path in folder.read_dir().expect("Unable to get directory contents") {
        let new_path = new_path.unwrap();
        let metadata = new_path.metadata().unwrap();

        if metadata.is_dir() {
            let sub_paths = folder_walk(new_path.path().as_path());
            paths.extend(sub_paths);
        } else {
            paths.push(new_path.path().to_owned())
        }
    }
    paths
}

///Returns folder size in kilobytes
fn get_folder_size(folder: &Path) -> u64 {
    let mut size = 0;

    for new_path in folder.read_dir().expect("Unable to get directory contents") {
        let new_path = new_path.unwrap();
        let metadata = new_path.metadata().unwrap();

        if metadata.is_dir() {
            size += get_folder_size(new_path.path().as_path())
        } else {
            size += metadata.len() / 1000;
        }
    }
    size
}