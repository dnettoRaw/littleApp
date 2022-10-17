/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: main.rs                              */
/*       ## ##                                                */
/*                    C: 2022/06/15 16:11:27 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/10/17 19:34:53 by:dnettoRaw     */
/*    ###########                                             */


#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod menu_toolbar;
mod function;
use function::config_file::{FILE_MAIN, INIT_CONFIGS};
use tauri::{WindowBuilder, window};
use tauri::Size;

use crate::menu_toolbar::menu::{dr_menu, dr_event};
use crate::function::button_test;
use crate::function::sql::crud;

use serde_json::Value;


#[tauri::command]
async fn create_window(app: tauri::AppHandle) {
  let (has_init,mut data) = get_config!(FILE_MAIN);
  
  if !has_init {
    ( _ ,data) = get_config!(INIT_CONFIGS | FILE_MAIN);
  }

  let _configs:Value =serde_json::from_str(&data).unwrap();
log!("configs: {:?}",_configs);
  
  let tittle = _configs["tittle"].as_str().unwrap();
  
  let window = tauri::WindowBuilder::new(&app, "label", tauri::WindowUrl::App("index.html".into()))
    .build()
    .unwrap();
  window.show().unwrap();
  window.set_title(tittle).unwrap();
}

fn main() {
    
  // let tittle = _configs["tittle"].as_str().unwrap();
  // let width = _configs["width"].as_f64().unwrap();
  // let height = _configs["height"].as_f64().unwrap();
  
  tauri::Builder::default()
  .setup(|app|{
    WindowBuilder::new(
      app,
      "main".to_string(),
      tauri::WindowUrl::App("index.html".into()),
    )
    .title("little app".to_string())
    .resizable(false)
    .min_inner_size(700.0, 600.0)
    .build()?;
    Ok(())
  }).invoke_handler(tauri::generate_handler![button_test::my_button, crud::test,create_window])
  .menu(dr_menu())
  .on_menu_event(dr_event)
  .run(tauri::generate_context!("tauri.conf.json"))
  .expect("error while running tauri application");
  
  
}
