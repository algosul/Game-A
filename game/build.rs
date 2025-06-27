use std::{env, fs, path::{Path, PathBuf}};
fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let target_path = dst.join(entry.file_name());
        if entry_path.is_dir() {
            copy_dir(&entry_path, &target_path)?;
        } else {
            fs::copy(&entry_path, &target_path)?;
        }
    }
    Ok(())
}
fn main() -> std::io::Result<()> {
    let src = Path::new("./rc");
    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());
    copy_dir(src, &dst)?;
    println!("cargo:rerun-if-changed={src}", src = src.display());
    Ok(())
}
