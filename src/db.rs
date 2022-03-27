pub fn open() -> Option<sled::Db> {
    match dirs::config_dir() {
        Some(dir) => {
            let db_path = format!("{}/klata/db", dir.to_str().unwrap());
            sled::open(db_path).ok()
        },
        None => None
    }
}
