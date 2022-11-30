/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: crud.rs                              */
/*       ## ##                                                */
/*                    C: 2022/09/21 06:08:50 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/11/30 13:43:01 by:dnettoRaw     */
/*    ###########                                             */

#![allow(dead_code)]
// aqui voce encontrara as funções que irão fazer a comunicação com o banco de dados

use serde_json::Value;
use tauri::api::path;

use rusqlite::{Connection};

use crate::{get_config, function::config_file::{FILE_MAIN,INIT_CONFIGS, FORCE_CONFIGS, FILE_DB}, log};


use super::tool::{ create_file_if_not_exists, create_folder_if_not_exists};
// push_data/

#[tauri::command]
pub fn test() {
    let (_result, str) = get_config!(FORCE_CONFIGS | FILE_MAIN);
    println!("test {}", str);
    create_db();
}

    
fn create_db() {
    let (mut has_init,mut data) = get_config!(FILE_DB);
    if !has_init {
        (has_init,data) = get_config!(INIT_CONFIGS | FILE_DB);
    }
    let _configs:Value =serde_json::from_str(&data).unwrap();
    // if _configs["useLocal"].as_str().unwrap() == "true" {
        log!("local db true");
        let db_path = _configs["local"]["Path"].as_str().unwrap();
        let db_name = _configs["local"]["Name"].as_str().unwrap();
        let db_full_path = format!("{}//{}//{}//{}", path::local_data_dir().unwrap().to_string_lossy(),"little_app",db_path, db_name);
        create_folder_if_not_exists(&format!("{}//{}//{}", path::local_data_dir().unwrap().to_string_lossy(),"little_app",db_path));
        create_file_if_not_exists(&db_full_path);
    // }
    log!("{}", _configs); 
    
    let conn = Connection::open(&db_full_path).unwrap();
    delete_tables(conn);  
}

fn create_table(conn: Connection) {
    let createables = vec![// auto generated 
    "CREATE TABLE IF NOT EXISTS access (  id INTEGER  NOT NULL UNIQUE,  level INTEGER  NULL,  name TEXT NULL,  PRIMARY KEY (id AUTOINCREMENT));",
    "CREATE TABLE IF NOT EXISTS adress (  id INTEGER  NOT NULL UNIQUE,  streat_number INTEGER  NULL,  streat TEXT NULL,  cep INTEGER  NULL,  country TEXT NULL,  PRIMARY KEY (id AUTOINCREMENT));",
    "CREATE TABLE IF NOT EXISTS category (  id INTEGER  NOT NULL UNIQUE,  name TEXT NOT NULL,  PRIMARY KEY (id AUTOINCREMENT));",
    "CREATE TABLE IF NOT EXISTS configs (  version INTEGER  NOT NULL UNIQUE,  flags_offset TEXT NULL,  PRIMARY KEY (version AUTOINCREMENT));",
    "CREATE TABLE IF NOT EXISTS created (  id INTEGER  NOT NULL UNIQUE,  creat_user INTEGER  NOT NULL,  edit_user INTEGER  NOT NULL,  creat_date INTEGER  NOT NULL,  edit_date INTEGER  NOT NULL,  PRIMARY KEY (id AUTOINCREMENT),  FOREIGN KEY (edit_user) REFERENCES user (id),  FOREIGN KEY (creat_user) REFERENCES user (id));",
    "CREATE TABLE IF NOT EXISTS ean (  id INTEGER  NOT NULL UNIQUE,  type INTEGER  NULL,  data TEXT NULL UNIQUE,  next_ean INTEGER  NULL,  article INTEGER  NOT NULL,  price INTEGER  NULL,  PRIMARY KEY (id AUTOINCREMENT),  FOREIGN KEY (next_ean) REFERENCES ean (id),  FOREIGN KEY (article) REFERENCES product (id),  FOREIGN KEY (price) REFERENCES price (id));",
    "CREATE TABLE IF NOT EXISTS lot (  id INTEGER  NOT NULL UNIQUE,  quantity INTEGER  NULL,  ean INTEGER  NOT NULL,  product INTEGER  NOT NULL,  price INTEGER  NOT NULL,  date INTEGER  NOT NULL,  expiration INTEGER  NULL,  name TEXT NULL,  PRIMARY KEY (id AUTOINCREMENT),  FOREIGN KEY (ean) REFERENCES ean (id),  FOREIGN KEY (product) REFERENCES product (id),  FOREIGN KEY (price) REFERENCES price (id),  FOREIGN KEY (date) REFERENCES created (id));",
    "CREATE TABLE IF NOT EXISTS price (  id INTEGER  NOT NULL UNIQUE,  tax INTEGER  NOT NULL,  tax_price number NULL,  sell number NULL,  price_log INTEGER  NULL,  PRIMARY KEY (id AUTOINCREMENT),  FOREIGN KEY (tax) REFERENCES tax (id),  FOREIGN KEY (price_log) REFERENCES price_log (id));",
    "CREATE TABLE IF NOT EXISTS price_log (  id INTEGER  NOT NULL UNIQUE,  type INTEGER  NULL,  price number NULL,  date INTEGER  NULL,  next INTEGER  NULL,  product INTEGER  NOT NULL,  PRIMARY KEY (id AUTOINCREMENT),  FOREIGN KEY (next) REFERENCES price_log (id),  FOREIGN KEY (product) REFERENCES product (id));",
    "CREATE TABLE IF NOT EXISTS product (  id INTEGER  NOT NULL UNIQUE,  ean INTEGER  NULL,  name TEXT NOT NULL,  price INTEGER  NULL,  created INTEGER  NULL,  lot INTEGER  NULL,  providers INTEGER  NULL,  category INTEGER  NULL,  flags INTEGER  NULL,  PRIMARY KEY (id AUTOINCREMENT),  FOREIGN KEY (ean) REFERENCES ean (id),  FOREIGN KEY (price) REFERENCES price (id),  FOREIGN KEY (lot) REFERENCES lot (id),  FOREIGN KEY (created) REFERENCES created (id),  FOREIGN KEY (providers) REFERENCES providers (id),  FOREIGN KEY (category) REFERENCES category (id));",
    "CREATE TABLE IF NOT EXISTS provider (  id INTEGER  NOT NULL UNIQUE,  name TEXT NOT NULL,  contact TEXT NULL,  email TEXT NULL,  adress INTEGER  NOT NULL,  prefix TEXT NULL,  PRIMARY KEY (id AUTOINCREMENT),  FOREIGN KEY (adress) REFERENCES adress (id));",
    "CREATE TABLE IF NOT EXISTS providers (  id INTEGER  NOT NULL UNIQUE,  provider INTEGER  NOT NULL,  next INTEGER  NULL,  date INTEGER  NULL,  buy_prices INTEGER  NOT NULL,  PRIMARY KEY (id AUTOINCREMENT),  FOREIGN KEY (next) REFERENCES providers (id),  FOREIGN KEY (provider) REFERENCES provider (id),  FOREIGN KEY (buy_prices) REFERENCES price_log (id));",
    "CREATE TABLE IF NOT EXISTS sector (  id INTEGER  NOT NULL UNIQUE,  name TEXT NULL,  default_tax INTEGER  NULL,  childs INTEGER  NULL,  PRIMARY KEY (id AUTOINCREMENT),  FOREIGN KEY (default_tax) REFERENCES tax (id),  FOREIGN KEY (childs) REFERENCES sector_child (id));",
    "CREATE TABLE IF NOT EXISTS sector_child (  id INTEGER  NOT NULL UNIQUE,  parent INTEGER  NOT NULL,  next_child INTEGER  NULL,  name TEXT NULL,  cat INTEGER  NOT NULL,  PRIMARY KEY (id AUTOINCREMENT),  FOREIGN KEY (next_child) REFERENCES sector_child (id),  FOREIGN KEY (cat) REFERENCES category (id));",
    "CREATE TABLE IF NOT EXISTS stock (  id INTEGER  NOT NULL UNIQUE,  stock INTEGER  NULL,  mim INTEGER  NULL,  edit INTEGER  NULL,  product INTEGER  NOT NULL,  PRIMARY KEY (id AUTOINCREMENT),  FOREIGN KEY (edit) REFERENCES created (id),  FOREIGN KEY (product) REFERENCES product (id));",
    "CREATE TABLE IF NOT EXISTS tax (  id INTEGER  NOT NULL UNIQUE,  name TEXT NULL,  value number NULL,  PRIMARY KEY (id AUTOINCREMENT));",
    "CREATE TABLE IF NOT EXISTS user (  id INTEGER  NOT NULL UNIQUE,  name TEXT NOT NULL,  last_name TEXT NULL,  active INTEGER  NULL,  sector INTEGER  NOT NULL,  PRIMARY KEY (id),  FOREIGN KEY (sector) REFERENCES user_sector (id));",
    "CREATE TABLE IF NOT EXISTS user_sector (  id INTEGER  NOT NULL UNIQUE,  acess INTEGER  NOT NULL,  PRIMARY KEY (id AUTOINCREMENT),  FOREIGN KEY (acess) REFERENCES access (id));];"
    ];
    
    for (i, table) in createables.iter().enumerate() {
        let mut stmt = conn.prepare(table).unwrap();
        stmt.execute([]).unwrap();
    }
}
fn delete_tables(conn: Connection) {
    let deleteables = vec![
    "DROP TABLE IF EXISTS adress;",
    "DROP TABLE IF EXISTS access;",
    "DROP TABLE IF EXISTS category;",
    "DROP TABLE IF EXISTS created;",
    "DROP TABLE IF EXISTS configs;",
    "DROP TABLE IF EXISTS ean;",
    "DROP TABLE IF EXISTS lot;",
    "DROP TABLE IF EXISTS price;",
    "DROP TABLE IF EXISTS price_log;",
    "DROP TABLE IF EXISTS product;",
    "DROP TABLE IF EXISTS provider;",
    "DROP TABLE IF EXISTS providers;",
    "DROP TABLE IF EXISTS sector;",
    "DROP TABLE IF EXISTS sector_child;",
    "DROP TABLE IF EXISTS stock;",
    "DROP TABLE IF EXISTS tax;",
    "DROP TABLE IF EXISTS user;",
    "DROP TABLE IF EXISTS user_sector;",
    ];
for (i, table) in deleteables.iter().enumerate() {
        let mut stmt = conn.prepare(table).unwrap();
        stmt.execute([]).unwrap();
    }
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