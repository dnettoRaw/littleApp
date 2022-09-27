/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: main.rs                              */
/*       ## ##                                                */
/*                    C: 2022/06/15 16:11:27 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/09/27 16:30:34 by:dnettoRaw     */
/*    ###########                                             */


#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod menu_toolbar;
mod function;
use function::config_file::{FILE_MAIN, INIT_CONFIGS};
use tauri::WindowBuilder;

use crate::menu_toolbar::menu::{dr_menu, dr_event};
use crate::function::button_test;
use crate::function::sql::crud;

use serde_json::Value;

fn main() {
  
  let (mut has_init,mut data) = get_config!(FILE_MAIN);
  
  if !has_init {
    (has_init,data) = get_config!(INIT_CONFIGS | FILE_MAIN);
  }

  let _configs:Value =serde_json::from_str(&data).unwrap();
    
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
  }).invoke_handler(tauri::generate_handler![button_test::my_button, crud::test])
  .menu(dr_menu())
  .on_menu_event(dr_event)
  .run(tauri::generate_context!("tauri.conf.json"))
  .expect("error while running tauri application");
}
