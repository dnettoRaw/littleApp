/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: config_file.rs                       */
/*       ## ##                                                */
/*                    C: 2022/09/22 12:23:39 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/09/22 15:32:25 by:dnettoRaw     */
/*    ###########                                             */

use tauri::api::path;
use std::{fs, time::UNIX_EPOCH};
use machine_uid;
use std::time::{SystemTime};

pub fn get_config_file() -> String {

    default_config_file();

    // Early return on error

    return "olares".to_string();
}

fn default_config_file() {
    let path:String = format!("{}\\{}", path::local_data_dir().unwrap().to_string_lossy() ,"little_app");
    let file_main:String = "littleapp.conf.json".to_string();
    let file_db:String = "littleapp.db.json".to_string();
    let file_log:String = "littleapp.log.json".to_string();
    let file_extern:String = "littleapp.extern.json".to_string();
    let id: String = machine_uid::get().unwrap();
    let n = SystemTime::now().duration_since(UNIX_EPOCH);
    
    let config_default = format!("{{\n\t\"title\": \"little App\",\n\t\"config\" : {{\n\t\t\"version\" : \"1\",\n\t\t\"reset\" : \"false\",\n\t\t\"created\" : \"{}\",\n\t\t\"lastUpdate\" : \"-1\"\n\t}},\n\t\t\"screenSize\" : {{\n\t\t\"width\" : \"700.0\",\n\t\t\"height\" : \"600.0\"\n\t}},\n\t\"useTray\" : \"false\",\n\t\"fullScreen\" : \"false\",\n\t\"alwaysOnTop\" : \"false\",\n\t\"machineId\" : \"{}\"\n}}",n.expect("-1").as_secs(),id);
    let config_db = "{\n\t\"db\" : {\n\t\t\"useLocal\" : \"true\",\n\t\t\"localTimeout\" : \"86400\",\n\t\t\"localName\" : \"-1\",\n\t\t\"host\" : \"-1\",\n\t\t\"port\" : \"-1\",\n\t\t\"user\" : \"root\",\n\t\t\"password\" : \"root\",\n\t\t\"database\" : \"littleApp\"\n\t}\n}";
    let config_log = "{\n\t\"useLog\" : \"true\",\n\t\"logPath\" : \"log\",\n\t\"logName\" : \"littleApp.log\",\n\t\"logSize\" : \"1000000\",\n\t\"logLevel\" : \"debug\"\n}";
    let config_extern = "{\n\t\"externalScripts\" : true,\n\t\"path\" : \"little_app\\externalScripts\",\n\t\"verify\" : \"false\",\n\t\"verifyFile\" : \"externalScripts\\verify.dr\"\n}";


    // criar a pasta se ela não existir
    if !fs::metadata(&path).is_ok() {
        fs::create_dir(&path).unwrap();
        fs::File::create(format!("{}\\{}",path,file_main)).unwrap();
        fs::File::create(format!("{}\\{}",path,file_db)).unwrap();
        fs::File::create(format!("{}\\{}",path,file_log)).unwrap();
        fs::File::create(format!("{}\\{}",path,file_extern)).unwrap();
    }
    
    // escrever o conteúdo no arquivo
    if fs::metadata(format!("{}\\{}",path,file_main)).is_ok() {
        fs::write(format!("{}\\{}",path,file_main), config_default).unwrap();
        fs::write(format!("{}\\{}",path,file_db), config_db).unwrap();
        fs::write(format!("{}\\{}",path,file_log), config_log).unwrap();
        fs::write(format!("{}\\{}",path,file_extern), config_extern).unwrap();
    }
}