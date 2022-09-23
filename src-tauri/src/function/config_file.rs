/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: config_file.rs                       */
/*       ## ##                                                */
/*                    C: 2022/09/22 12:23:39 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/09/23 16:01:22 by:dnettoRaw     */
/*    ###########                                             */

use machine_uid;

use std::time::SystemTime;
use std::{fs, time::UNIX_EPOCH};
use tauri::api::path;

const FILE_MAIN_STR: &str = "littleapp.conf.json";
const FILE_DB_STR: &str = "littleapp.db.json";
const FILE_LOG_STR: &str = "littleapp.log.json";
const FILE_EXTERN_STR: &str = "littleapp.extern.json";

#[allow(dead_code)]
pub const INIT_CONFIGS: u8 = 1 << 0;
pub	const FILE_MAIN: u8 = 1 << 1;
pub	const FILE_DB: u8 = 1 << 2;
pub	const FILE_LOG: u8 = 1 << 3;
pub	const FILE_EXTERN: u8 = 1 << 4;

#[macro_export]
macro_rules! get_config {
	($file:expr) => {
		$crate::function::config_file::get_config_file($file) 
	};
	() => {
		$crate::function::config_file::get_config_file(INIT_CONFIGS)
	};
}

/// .
///
/// # Panics
///
/// Panics if .
pub fn get_config_file(opt:u8) -> String {
	let path: String = format!(
		"{}\\{}",
		path::local_data_dir().unwrap().to_string_lossy(),
		"little_app"
	);
	// para verificar se o arquivo existe, se não existir, criar o arquivo
  if opt & INIT_CONFIGS != 0 {
    init_confg(path);
  }
	if opt & FILE_MAIN != 0 {
		return String::from("1");
	}
	if opt & FILE_DB != 0 {
		return String::from("2");
	}
	if opt & FILE_LOG != 0 {
		return String::from("3");
	}
	if opt & FILE_EXTERN != 0 {
		return String::from("4");
	}
	return String::from("5");

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

pub fn init_confg(path:String) {

  if !fs::metadata(&path).is_ok() {
    fs::create_dir(&path).unwrap();
	init_default(format!("{}\\{}",path,FILE_MAIN_STR));
	init_db(format!("{}\\{}",path,FILE_DB_STR));
	init_log(format!("{}\\{}",path,FILE_LOG_STR));
	init_externe(format!("{}\\{}",path,FILE_EXTERN_STR));
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


fn init_default(path: String) {
  let id_machine: String = machine_uid::get().unwrap();
  let elapsed_s_epoch: u64 = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("-1")
    .as_secs();
  let config_default:String = format!("{{\n\t\"title\": \"little App\",\n\t\"config\" : {{\n\t\t\"version\" : \"1\",\n\t\t\"reset\" : \"false\",\n\t\t\"created\" : \"{}\",\n\t\t\"lastUpdate\" : \"-1\"\n\t}},\n\t\t\"screenSize\" : {{\n\t\t\"width\" : \"700.0\",\n\t\t\"height\" : \"600.0\"\n\t}},\n\t\"useTray\" : \"false\",\n\t\"fullScreen\" : \"false\",\n\t\"alwaysOnTop\" : \"false\",\n\t\"machineId\" : \"{}\"\n}}", elapsed_s_epoch, id_machine);
  if fs::metadata(&path).is_ok() {
    fs::write(path, config_default).unwrap();
  }
}

fn init_db(path: String) {
  const CONFIG_DB: &str = "{\n\t\"db\" : {\n\t\t\"useLocal\" : \"true\",\n\t\t\"localTimeout\" : \"86400\",\n\t\t\"localName\" : \"-1\",\n\t\t\"host\" : \"-1\",\n\t\t\"port\" : \"-1\",\n\t\t\"user\" : \"root\",\n\t\t\"password\" : \"root\",\n\t\t\"database\" : \"littleApp\"\n\t}\n}";
  if fs::metadata(&path).is_ok() {
    fs::write(path, CONFIG_DB).unwrap();
  }
}

fn init_log(path: String) {
  const CONFIG_LOG: &str = "{\n\t\"useLog\" : \"true\",\n\t\"logPath\" : \"log\",\n\t\"logName\" : \"littleApp.log\",\n\t\"logSize\" : \"1000000\",\n\t\"logLevel\" : \"debug\"\n}";
  if fs::metadata(&path).is_ok() {
    fs::write(path, CONFIG_LOG).unwrap();
  }
}

fn init_externe(path: String) {
  const CONFIG_EXTERN: &str = "{\n\t\"externalScripts\" : true,\n\t\"path\" : \"little_app\\\\externalScripts\",\n\t\"verify\" : \"false\",\n\t\"verifyFile\" : \"externalScripts\\\\verify.dr\"\n}";
  if fs::metadata(&path).is_ok() {
    fs::write(path, CONFIG_EXTERN).unwrap();
  }
}
