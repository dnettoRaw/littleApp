/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: crud.rs                              */
/*       ## ##                                                */
/*                    C: 2022/09/21 06:08:50 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/09/21 16:12:20 by:dnettoRaw     */
/*    ###########                                             */

#[allow(dead_code)]
// aqui voce encontrara as funções que irão fazer a comunicação com o banco de dados

use tauri::api::path;
use tauri::Config;
use std::fs::{self, OpenOptions};

#[tauri::command]
pub fn test() {
    let config = Config::default();

    println!("app_dir {:?}", path::app_dir(&config).unwrap());
    println!("audio_dir {:?}", path::audio_dir().unwrap());       	
    println!("cache_dir {:?}", path::cache_dir().unwrap());       
    println!("config_dir {:?}", path::config_dir().unwrap());       
    println!("data_dir {:?}", path::data_dir().unwrap());       	
    println!("desktop_dir {:?}", path::desktop_dir().unwrap());       
    println!("document_dir {:?}", path::document_dir().unwrap());       	
    println!("download_dir {:?}", path::download_dir().unwrap());       
    println!("executable_dir {:?}", path::executable_dir());       
    println!("font_dir {:?}", path::font_dir());       	
    println!("home_dir {:?}", path::home_dir());       	
    println!("local_data_dir {:?}", path::local_data_dir());       	
    println!("log_dir {:?}", path::log_dir(&config).unwrap());       	
    // println!("parse {:?}", path::parse().unwrap());       
    println!("picture_dir {:?}", path::picture_dir().unwrap());       	
    println!("public_dir {:?}", path::public_dir().unwrap());       	
    // println!("resolve_path {:?}", path::resolve_path().unwrap());       	
    // println!("resource_dir {:?}", path::resource_dir().unwrap());       	
    println!("runtime_dir {:?}", path::runtime_dir());       	
    println!("template_dir {:?}", path::template_dir().unwrap());  
    println!("video_dir {:?}", path::video_dir().unwrap());       
    let pat = path::desktop_dir().unwrap();
    // pushdata(&"liricos".to_string(), &pat.pa, &"teste".to_string());
}

fn createFolderIfNotExists(path: &str) {
    if !fs::metadata(path).is_ok() {
        fs::create_dir(path).unwrap();
    }
}

fn createFileIfNotExists(path: &str) {
    if !fs::metadata(path).is_ok() {
        fs::File::create(path).unwrap();
    }
}

fn createFileIfNotExistsAndWrite(path: &str, content: &str) {
    if !fs::metadata(path).is_ok() {
        fs::write(path, content).unwrap();
    }
}

fn updateFileIfExists(path: &str, content: &str) {
    if fs::metadata(path).is_ok() {
        fs::write(path, content).unwrap();
    }
}

fn deleteFileIfExists(path: &str) {
    if fs::metadata(path).is_ok() {
        fs::remove_file(path).unwrap();
    }
}

fn deleteFolderIfExists(path: &str) {
    if fs::metadata(path).is_ok() {
        fs::remove_dir(path).unwrap();
    }
}

fn pushdata( file:&str, path:&str, data:&str ) {
    createFolderIfNotExists(path);
    createFileIfNotExists(&format!("{}/{}", path, file));
}
    