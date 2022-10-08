use std::{
    fs::{self, File},
    path::Path,
};

/// Generates an authentication lock.
pub fn acquire_auth_lock() {
    // Some good, old-fashioned semaphore action.
    match File::create(Path::new(".authlock")) {
        Err(why) => panic!("couldn't create .authlock file: {}", why),
        Ok(file) => file,
    };
}

/// Removes the authentication lock.
pub fn release_auth_lock() -> bool {
    match fs::remove_file(".authlock") {
        Err(_) => false,
        Ok(_) => true,
    }
}

/// Checks whether a lockfile exists.
pub fn check_auth_lock() -> bool {
    match File::open(Path::new(".authlock")) {
        Err(_) => false,
        Ok(_) => true,
    }
}
