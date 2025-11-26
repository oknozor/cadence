use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo::rerun-if-changed=.");
    watch_dir("./android/".into());

    #[cfg(debug_assertions)]
    let dest = PathBuf::from(format!(
        "../../target/dx/app/debug/android/app/app/src/main/kotlin/dev/dioxus/main"
    ));

    #[cfg(not(debug_assertions))]
    let dest = PathBuf::from(format!(
        "../../target/dx/app/release/android/app/app/src/main/kotlin/dev/dioxus/main"
    ));

    fs::create_dir_all(&dest).unwrap();
    for entry in fs::read_dir("../../android").unwrap() {
        let entry = entry.unwrap();
        fs::copy(entry.path(), dest.join(entry.file_name())).unwrap();
    }
}

fn watch_dir(path: PathBuf) {
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let p = entry.path();
            if p.is_dir() {
                watch_dir(p);
            } else {
                println!("cargo::rerun-if-changed={}", p.display());
            }
        }
    }
}
