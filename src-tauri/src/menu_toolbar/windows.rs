/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: windows.rs                           */
/*       ## ##                                                */
/*                    C: 2022/06/02 16:38:13 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/09/20 09:55:23 by:dnettoRaw     */
/*    ###########                                             */

#[cfg(windows)]
use tauri::{
 CustomMenuItem, Menu, MenuItem
};

pub fn get_my_app() -> Menu {
  Menu::with_items([
    CustomMenuItem::new("20", "settings").into(),
    MenuItem::Hide.into(),
    MenuItem::Separator.into(),
    MenuItem::Quit.into(),
  ])
}
pub fn get_file() -> Menu {
  Menu::with_items([])
}
pub fn get_edit() -> Menu {
  Menu::with_items([
    MenuItem::Undo.into(),
    MenuItem::Redo.into(),
    MenuItem::Separator.into(),
    MenuItem::Cut.into(),
    MenuItem::Copy.into(),
    MenuItem::Paste.into(),
    MenuItem::Separator.into(),
    MenuItem::SelectAll.into(),
  ])
}
pub fn get_help() -> Menu {
  Menu::with_items([
    MenuItem::Separator.into(),
    //  firts is Id this need to be same in your functions, secound is the name displayed
    CustomMenuItem::new("01", "dnetto github").into(),
    CustomMenuItem::new("02", "dnetto site").into(),
    CustomMenuItem::new("03", "tauri github").into(),
  ])
}
