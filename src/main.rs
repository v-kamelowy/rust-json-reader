use std::env::args;

//use jwalk::{WalkDir, Parallelism};

/*
fn _walk_dir() -> Result<Vec<String>, jwalk::Error> {
    let mut paths = Vec::new();
    for entry in WalkDir::new("Z:\\media\\yt-dlp").parallelism(Parallelism::RayonNewPool(0)) {
        let entry = entry?;
        if entry.file_type().is_file() {
            paths.push(entry.path().display().to_string());
        }
    }
    Ok(paths)
}


fn all_videos() -> String {
    let paths = _walk_dir().unwrap();
    let mut output = String::new();
    for path in paths {
        output.push_str(&path);
        output.push_str("\n");
    }
    output

}
*/
fn main() {
    let args: Vec<String> = args().collect();
    let file_name = &args[1];
    let _file = std::fs::File::open(file_name).expect("could not open file");
    let file_contents = std::fs::read_to_string(file_name).expect("could not read file");
    println!("{}", file_contents);
}