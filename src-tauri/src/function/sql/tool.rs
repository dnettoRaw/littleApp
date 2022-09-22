/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: tool.rs                              */
/*       ## ##                                                */
/*                    C: 2022/09/22 10:21:17 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/09/22 11:36:47 by:dnettoRaw     */
/*    ###########                                             */

use std::fs::{self, OpenOptions};

pub fn create_folder_if_not_exists(path: &str) {
    if !fs::metadata(path).is_ok() {
        fs::create_dir(path).unwrap();
    }
}

pub fn create_file_if_not_exists(path: &str) {
    if !fs::metadata(path).is_ok() {
        fs::File::create(path).unwrap();
    }
}

pub fn create_file_if_not_exists_and_write(path: &str, content: &str) {
    if !fs::metadata(path).is_ok() {
        fs::write(path, content).unwrap();
    }
}

pub fn update_file_insert_new_line(path: &str, content: &str) {
    if fs::metadata(path).is_ok() {
        let temp = &format!("{}\n{}",fs::read_to_string(path).unwrap(), content);
        fs::write(path, temp).unwrap();
    }
}

pub fn delete_file_if_exists(path: &str) {
    if fs::metadata(path).is_ok() {
        fs::remove_file(path).unwrap();
    }
}

pub fn delete_folder_if_exists(path: &str) {
    if fs::metadata(path).is_ok() {
        fs::remove_dir(path).unwrap();
    }
}

pub fn push_data( file:&str, path:&str, data:&str ) {
    create_folder_if_not_exists(path);
    create_file_if_not_exists(&format!("{}/{}", path, file));
    
    update_file_insert_new_line(&format!("{}/{}", path, file), data)
}