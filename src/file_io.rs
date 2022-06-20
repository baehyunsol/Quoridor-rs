#![allow(dead_code)]

use std::str::FromStr;
use std::fs::{self, File, read};
use std::path::PathBuf;
use std::io::{Read, Write};

// `a/b/c.d -> `d``
pub fn extension(path: &str) -> Result<String, ()> {

    match PathBuf::from_str(path) {
        Err(_) => Err(()),
        Ok(path) => match path.extension() {
            None => Err(()),
            Some(s) => match s.to_str() {
                None => Err(()),
                Some(ext) => Ok(ext.to_string())
            }
        }
    }

}

// `a/b/c.d` -> `c.d`
pub fn basename(path: &str) -> Result<String, ()> {

    match PathBuf::from_str(path) {
        Err(_) => Err(()),
        Ok(path) => match path.file_name() {
            None => Err(()),
            Some(s) => match s.to_str() {
                None => Err(()),
                Some(ext) => Ok(ext.to_string())
            }
        }
    }

}

// `a/b/c.d` -> `a/b`
pub fn parent(path: &str) -> Result<String, ()> {

    match PathBuf::from_str(path) {
        Err(_) => Err(()),
        Ok(path) => match path.parent() {
            None => Err(()),
            Some(s) => match s.to_str() {
                None => Err(()),
                Some(ext) => Ok(ext.to_string())
            }
        }
    }

}

// `a/b/c.d -> `c``
pub fn file_name(path: &str) -> Result<String, ()> {

    match PathBuf::from_str(path) {
        Err(_) => Err(()),
        Ok(path) => match path.file_stem() {
            None => Err(()),
            Some(s) => match s.to_str() {
                None => Err(()),
                Some(ext) => Ok(ext.to_string())
            }
        }
    }

}

pub fn is_dir(path: &str) -> bool {

    match PathBuf::from_str(path) {
        Err(_) => false,
        Ok(path) => path.is_dir()
    }

}

pub fn is_file(path: &str) -> bool {

    match PathBuf::from_str(path) {
        Err(_) => false,
        Ok(path) => path.is_file()
    }

}

pub fn read_dir(path: &str) -> Result<Vec<String>, ()> {

    match fs::read_dir(path) {
        Err(_) => Err(()),
        Ok(entries) => {
            let mut result = vec![];

            for entry in entries {

                match entry {
                    Err(_) => {return Err(());}
                    Ok(e) => {
                        match e.path().to_str() {
                            None => {return Err(());}
                            Some(ee) => {
                                result.push(ee.to_string());
                            }
                        }
                    }
                }
            }

            Ok(result)
        }
    }
}

pub fn read_bytes(path: &str) -> Result<Vec<u8>, ()> {

    match read(path) {
        Ok(data) => Ok(data),
        Err(_) => Err(())
    }

}

pub fn read_string(path: &str) -> Result<String, ()> {

    let mut s = String::new();

    match File::open(path) {
        Err(_) => Err(()),
        Ok(mut f) => match f.read_to_string(&mut s) {
            Err(_) => Err(()),
            Ok(_) => Ok(s)
        }
    }

}

pub fn write_to_file(path: &str, bytes: &[u8]) -> Result<(), ()> {

    match File::create(path) {
        Err(_) => Err(()),
        Ok(mut f) => match f.write_all(bytes) {
            Err(_) => Err(()),
            Ok(_) => Ok(())
        }
    }

}
