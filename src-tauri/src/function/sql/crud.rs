/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: crud.rs                              */
/*       ## ##                                                */
/*                    C: 2022/09/21 06:08:50 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/10/17 16:13:04 by:dnettoRaw     */
/*    ###########                                             */

#![allow(dead_code)]
// aqui voce encontrara as funções que irão fazer a comunicação com o banco de dados

use serde_json::Value;
use tauri::api::path;

// use rusqlite::{Connection, Result};

use crate::{get_config, function::config_file::{FILE_MAIN,INIT_CONFIGS, FORCE_CONFIGS, FILE_DB}};


use super::tool::{ create_file_if_not_exists, create_folder_if_not_exists};
// push_data/

#[tauri::command]
pub fn test() {
    let (_result, str) = get_config!(FORCE_CONFIGS | FILE_MAIN);
    println!("test {}", str);
}

    
fn create_db() {
    let (mut has_init,mut data) = get_config!(FILE_DB);
    if !has_init {
        (has_init,data) = get_config!(INIT_CONFIGS | FILE_DB);
    }
    let _configs:Value =serde_json::from_str(&data).unwrap();
    let db_path = _configs["db_path"].as_str().unwrap();
    let db_name = _configs["db_name"].as_str().unwrap();
    let db_full_path = format!("{}{}", db_path, db_name);
    
 
    create_folder_if_not_exists(&db_path);
    create_file_if_not_exists(&db_full_path);
}

fn create_table() {

    
}




        // let config = Config::default();

        // println!("app_dir {:?}", path::app_dir(&config).unwrap());
        // println!("audio_dir {:?}", path::audio_dir().unwrap());       	

        
        // println!("cache_dir {:?}", path::cache_dir().unwrap());       
        // println!("config_dir {:?}", path::config_dir().unwrap());       
        // println!("data_dir {:?}", path::data_dir().unwrap());       	
        // println!("desktop_dir {:?}", path::desktop_dir().unwrap());       
        // println!("document_dir {:?}", path::document_dir().unwrap());       	
        // println!("download_dir {:?}", path::download_dir().unwrap());       
        // println!("executable_dir {:?}", path::executable_dir());       
        // println!("font_dir {:?}", path::font_dir());       	
        // println!("home_dir {:?}", path::home_dir());       	
        // println!("local_data_dir {:?}", path::local_data_dir());       	
        // println!("log_dir {:?}", path::log_dir(&config).unwrap());       	
        // // println!("parse {:?}", path::parse().unwrap());       
        // println!("picture_dir {:?}", path::picture_dir().unwrap());       	
        // println!("public_dir {:?}", path::public_dir().unwrap());       	
        // // println!("resolve_path {:?}", path::resolve_path().unwrap());       	
        // // println!("resource_dir {:?}", path::resource_dir().unwrap());       	
        // println!("runtime_dir {:?}", path::runtime_dir());       	
        // println!("template_dir {:?}", path::template_dir().unwrap());  
        // println!("video_dir {:?}", path::video_dir().unwrap()); 