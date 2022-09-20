/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: main.rs                              */
/*       ## ##                                                */
/*                    C: 2022/06/15 16:11:27 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/09/20 09:47:41 by:dnettoRaw     */
/*    ###########                                             */


#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod menu_toolbar;
mod function;
use tauri::WindowBuilder;

use crate::menu_toolbar::menu::{dr_menu, dr_event};
use crate::function::button_test;

fn main() {
  tauri::Builder::default()
  .setup(|app|{
    WindowBuilder::new(
      app,
      "main".to_string(),
      tauri::WindowUrl::App("index.html".into()),
    )
    .title("little App")
    .resizable(false)
    .min_inner_size(700.0, 600.0)
    .build()?;
    Ok(())
  }).invoke_handler(tauri::generate_handler![button_test::my_button])
  .menu(dr_menu())
  .on_menu_event(dr_event)
  .run(tauri::generate_context!("tauri.conf.json"))
  .expect("error while running tauri application");
}
