/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: main.rs                              */
/*       ## ##                                                */
/*                    C: 2022/06/15 16:11:27 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/06/15 16:51:42 by:dnettoRaw     */
/*    ###########                                             */


#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod menu_toolbar;
mod function;
use crate::menu_toolbar::menu::{dr_menu, dr_event};
use crate::function::button_test;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![button_test::my_button])
    .menu(dr_menu())
    .on_menu_event(dr_event)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
