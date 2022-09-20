/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: mac.rs                               */
/*       ## ##                                                */
/*                    C: 2022/06/15 16:13:39 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/06/16 14:37:57 by:dnettoRaw     */
/*    ###########                                             */


#![allow(dead_code)]
#[allow(unused_imports)]
use tauri::{CustomMenuItem,
    Manager,
    Menu,
    MenuEntry,
    MenuItem,
    Submenu,
    WindowBuilder,
    WindowUrl,
    AboutMetadata};
    
pub fn get_my_app() -> Menu {
  // for somme raison i dont know this dont work anymore 
  // let  medata = AboutMetadata::default().authors(vec![String::from("dnetto") , String::from("Raw"), String::from("sommeone")])
      // .comments(String::from("teste 1"))
      // .copyright(String::from("teste 2"))
      // .license(String::from("teste 3"))
      // .version(String::from("teste 4"))
      // .website(String::from("dnetto.dev"));

  Menu::with_items([
        MenuItem::About(String::from("myAppInDev"),AboutMetadata::default()).into(),
        MenuItem::Separator.into(),
        MenuItem::Services.into(),
        MenuItem::Separator.into(),
        MenuItem::Hide.into(),
        MenuItem::HideOthers.into(),
        MenuItem::ShowAll.into(),
        MenuItem::Separator.into(),
        MenuItem::Quit.into(),
  ])
}
pub fn get_file() -> Menu {
  Menu::with_items([
    MenuItem::CloseWindow.into(),
  ])
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

pub fn get_window() -> Menu {
  Menu::with_items([
      MenuItem::Minimize.into(),
      MenuItem::Zoom.into(),
      MenuItem::Separator.into(),
      MenuItem::Hide.into(),
      MenuItem::HideOthers.into(),
      MenuItem::ShowAll.into(),
      MenuItem::Separator.into(),
      MenuItem::CloseWindow.into(),
  ])
}


// use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};
// let quit = CustomMenuItem::new("quit".to_string(), "Quit");
// let hide = CustomMenuItem::new("hide".to_string(), "Hide");
// let tray_menu = SystemTrayMenu::new()
//   .add_item(quit)
//   .add_native_item(SystemTrayMenuItem::Separator)
//   .add_item(hide);
//   let system_trayw = SystemTray::new().with_menu(tray_menu);