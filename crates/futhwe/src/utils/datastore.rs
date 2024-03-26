use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Result;

const DATASTORE: &str = "__datastore__";

pub fn create_or_open(path: String) -> Result<PathBuf> {
    let mut dir_path = PathBuf::from(std::env::current_dir()?);
    dir_path.push(DATASTORE); // Create directory
    dir_path.push(path); // Create directory

    if !dir_path.exists() {
        create_dir_all(&dir_path).unwrap_or_else(|_| {
            eprintln!("Error creating directory: {:?}", dir_path);
        });
        // add .gitignore to the directory
        let mut file_path = dir_path.clone();
        file_path.push(".gitignore");
        let mut file = File::create(file_path)?;
        file.write_all(b"*\n")?;
    }

    Ok(dir_path)
}

pub fn unzip_file(path: PathBuf, name: &str) -> Result<()> {
    let file = File::open(path.join(name))?;
    let mut archive = zip::ZipArchive::new(file)?;

    archive.extract(path)?;

    Ok(())
}
