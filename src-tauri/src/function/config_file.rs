/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: config_file.rs                       */
/*       ## ##                                                */
/*                    C: 2022/09/22 12:23:39 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/10/17 19:17:55 by:dnettoRaw     */
/*    ###########                                             */

use machine_uid;

use std::fs::File;
use std::io::Write;
use std::io::Read;


use std::time::SystemTime;
use std::{fs, time::UNIX_EPOCH};
use tauri::api::path;

use crate::log;

const FILE_MAIN_STR: &str = "conf.dr";
const FILE_DB_STR: &str = "db.dr";
const FILE_LOG_STR: &str = "log.dr";
const FILE_EXTERN_STR: &str = "extern.dr";

#[allow(dead_code)]
pub const INIT_CONFIGS: u8 = 1 << 0;
pub const FORCE_CONFIGS: u8 = 1 << 1 | INIT_CONFIGS;
pub	const FILE_MAIN: u8 = 1 << 2;
pub	const FILE_DB: u8 = 1 << 3;
pub	const FILE_LOG: u8 = 1 << 4;
pub	const FILE_EXTERN: u8 = 1 << 5;

#[macro_export]
macro_rules! get_config {
	($file:expr) => {
		$crate::function::config_file::get_config_file($file) 
	};
	() => {
		$crate::function::config_file::get_config_file($crate::function::config_file::INIT_CONFIGS)
	};
}

/// Get the config file
/// 
/// set the file to INIT_CONFIGS to set all files
/// 
/// set the file to FILE_MAIN to get the main config file
/// set the file to FILE_DB to get the db config file
/// set the file to FILE_LOG to get the log config file
/// set the file to FILE_EXTERN to get the extern config file
/// 
/// # Example
/// get_config!(INIT_CONFIGS);
/// get_config!(FORCE_CONFIGS);
/// get_config!(FILE_MAIN);
/// get_config!(FILE_DB);
/// get_config!(FILE_LOG);
/// get_config!(FILE_EXTERN);
/// get_config!();  // same as get_config!(INIT_CONFIGS);
/// 
/// the value of the config file is returned as a bool and String
/// 
/// in the case of an error, the error is returned: bool as false and String with the error message
/// 
/// 
/// in the future i think is better to write file as binary to keep the data safe
  
pub fn get_config_file(opt:u8) -> (bool, String) {
  let path: String = format!(
		"{}\\{}\\{}",
		path::local_data_dir().unwrap().to_string_lossy(),
		"little_app",
    "config"
	);
  log!("opt {} {0:b}", opt);
	// para verificar se o arquivo existe, se não existir, criar o arquivo
  if opt & INIT_CONFIGS != 0 {
    if opt & FORCE_CONFIGS != 0 {
      log!("force configs");
      if let Err(e) = fs::remove_dir_all(&path) {
        log!("Error: {}", e);
      }
    }
    init_confg(&path);
    log!("init_confg");
  }
	if opt & FILE_MAIN != 0 {
    if _exists_file(&path, FILE_MAIN_STR){
      return (true, _read_file(&path, FILE_MAIN_STR));
    }
		return (false, String::from("error: file not found")); //not implemented yet
	}
	if opt & FILE_DB != 0 {
    if _exists_file(&path, FILE_DB_STR){
      return (true, _read_file(&path, FILE_DB_STR));
    }
    return (false, String::from("error: file not found")); //not implemented yet
	}
  if opt & FILE_LOG != 0 {
    if _exists_file(&path, FILE_LOG_STR){
      return (true, _read_file(&path, FILE_LOG_STR));
    }
    return (false, String::from("error: file not found")); //not implemented yet
  }
  if opt & FILE_EXTERN != 0 {
	 	if _exists_file(&path, FILE_EXTERN_STR){
      return (true, _read_file(&path, FILE_EXTERN_STR));
    }
    return (false, String::from("error: file not found")); //not implemented yet
	}
	return (false, String::from("error: no option selected"));

  // Early return on error
 
}

// a = 1;
// b = 2;

// c = a & b;  //0  (01 && 10 -> 00)
// d = a | b;  //3  (01 || 10 -> 11)
// e = a ^ b;  //3  (01 != 10 -> 11)
// f = a << b; //4  (Add b number of 0s to the end of a -> '01'+'00' -> 100)
// g = a >> b; //0  (Remove b number of bits from the end of a -> o̶1̶ -> 0)
//  let mut opt: u8 = 0b00000000;
fn _read_file(path : &String, file : &str) -> String {
  let file_path = format!("{}\\{}", path, file);
  let file = fs::read_to_string(file_path).unwrap();
  return file;
}

fn _exists_file(path: &String, file: &str) -> bool {
  let file_path: String = format!("{}/{}", path, file);
  if fs::metadata(file_path).is_ok() {
    return true;
  }
  return false;
}

pub fn init_confg(path:&String) {

  if !fs::metadata(&path).is_ok() {
    fs::create_dir(&path).unwrap();
	init_default(format!("{}\\{}", path, FILE_MAIN_STR));
	init_db(format!("{}\\{}",path, FILE_DB_STR));
	init_log(format!("{}\\{}",path, FILE_LOG_STR));
	init_externe(format!("{}\\{}",path, FILE_EXTERN_STR));
  } else {
    if !fs::metadata(format!("{}\\{}", path, FILE_MAIN_STR)).is_ok() {
		init_default(format!("{}\\{}",path,FILE_MAIN_STR));
    }
    if !fs::metadata(format!("{}\\{}", path, FILE_DB_STR)).is_ok() {
		init_db(format!("{}\\{}",path,FILE_DB_STR));
    }
    if !fs::metadata(format!("{}\\{}", path, FILE_LOG_STR)).is_ok() {
		init_log(format!("{}\\{}",path,FILE_LOG_STR));
    }
    if !fs::metadata(format!("{}\\{}", path, FILE_EXTERN_STR)).is_ok() {
		init_externe(format!("{}\\{}",path,FILE_EXTERN_STR));
    }
  }
}

/// down here all default files values

fn init_default(path: String) {
  let id_machine: String = machine_uid::get().unwrap();
  let elapsed_s_epoch: u64 = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("-1")
    .as_secs();
  let config_default:String = format!("{{\n\t\"title\": \"little App\",\n\t\"config\" : {{\n\t\t\"version\" : \"1\",\n\t\t\"reset\" : \"false\",\n\t\t\"created\" : \"{}\",\n\t\t\"lastUpdate\" : \"-1\"\n\t}},\n\t\t\"screenSize\" : {{\n\t\t\"width\" : \"700.0\",\n\t\t\"height\" : \"600.0\"\n\t}},\n\t\"useTray\" : \"false\",\n\t\"fullScreen\" : \"false\",\n\t\"alwaysOnTop\" : \"false\",\n\t\"machineId\" : \"{}\"\n}}", elapsed_s_epoch, id_machine);

  if !fs::metadata(&path).is_ok() {
    fs::write(path, config_default).unwrap();
  }
}

fn init_db(path: String) {
  const CONFIG_DB : &str = "{\n\t\"name\" : \"littleApp.sqlite\",\n\t\"useLocal\" : \"true\",\n\t\"local\":{\n\t\t\"Timeout\" : \"86400\",\n\t\t\"Name\" : \"littleApp.sqlite\",\n\t\t\"Path\" : \"dataBase\"\n\t},\n\t\"conection\" : {\n\t\t\"host\" : \"-1\",\n\t\t\"port\" : \"-1\"\n\t},\n\t\"auth\" : {\n\t\t\"password\" : \"root\",\n\t\t\"user\" : \"root\"\n\t}\n}";
  if !fs::metadata(&path).is_ok() {
    fs::write(path, CONFIG_DB).unwrap();
  }
}

fn init_log(path: String) {
  const CONFIG_LOG: &str = "{\n\t\"useLog\" : \"true\",\n\t\"logPath\" : \"log\",\n\t\"logName\" : \"littleApp.log\",\n\t\"logSize\" : \"1000000\",\n\t\"logLevel\" : \"debug\"\n}";
  if !fs::metadata(&path).is_ok() {
    fs::write(path, CONFIG_LOG).unwrap();
  }
}

fn init_externe(path: String) {
  const CONFIG_EXTERN: &str = "{\n\t\"externalScripts\" : true,\n\t\"path\" : \"little_app\\\\externalScripts\",\n\t\"verify\" : \"false\",\n\t\"verifyFile\" : \"externalScripts\\\\verify.dr\"\n}";
  if !fs::metadata(&path).is_ok() {
    fs::write(path, CONFIG_EXTERN).unwrap();
  }
}