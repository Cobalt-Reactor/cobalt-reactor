use std::path::PathBuf;

pub fn check_create_folder(dir_name: &PathBuf) -> Result<(), std::io::Error> {
    if !dir_name.exists() {
        std::fs::create_dir_all(dir_name)?;
    }

    Ok(())
}
