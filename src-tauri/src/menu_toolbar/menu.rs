/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: menu.rs                              */
/*       ## ##                                                */
/*                    C: 2022/06/03 16:12:26 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/06/16 14:22:42 by:dnettoRaw     */
/*    ###########                                             */


#[allow(unused_imports)]
use tauri::{Menu, MenuItem, Submenu};


#[allow(unused_imports)]
use crate::menu_toolbar::{android,ios,linux,mac,windows};
use crate::function::social_link;

pub fn dr_menu() -> Menu {

  // if target_os unknow empty menu 
  // Menu::new();

  // add all our childs to the menu (order is how they'll appear)
  #[cfg(target_os = "ios")]{
    // unsupported for this tauri vertion
    Menu::new()
  }
  #[cfg(target_os = "android")]{
    // unsupported for this tauri vertion
    Menu::new()
  }
  #[cfg(target_os = "linux")]{
    Menu::new()
      .add_submenu(Submenu::new("app ", linux::get_my_app()))
      .add_submenu(Submenu::new("File", linux::get_file()))
      .add_submenu(Submenu::new("Edit", linux::get_edit()))
      .add_submenu(Submenu::new("Help", linux::get_help()))
  }
  #[cfg(target_os = "windows")]{
    Menu::new()
      .add_submenu(Submenu::new("app ", windows::get_my_app()))
      .add_submenu(Submenu::new("File", windows::get_file()))
      .add_submenu(Submenu::new("Edit", windows::get_edit()))
      .add_submenu(Submenu::new("Help", windows::get_help()))
  }
  #[cfg(target_os = "macos")]{
    Menu::new()
      .add_submenu(Submenu::new("app ", mac::get_my_app()))
      .add_submenu(Submenu::new("File", mac::get_file()))
      .add_submenu(Submenu::new("Edit", mac::get_edit()))
      .add_submenu(Submenu::new("Window", mac::get_window()))
      .add_submenu(Submenu::new("Help", mac::get_help())) 
  }
}

pub fn dr_event(event: tauri::WindowMenuEvent){  
/*
  the id's is in Hex start at 01 and at ff

  // social section
  01  github
  02  tauri doc
  
  // data base 
  10 init db
  11 update db
  ...

  the id can be anything you want, this is a String, but for me 
  i prefere to keep it in numerical id
  
*/
    let event_name = event.menu_item_id();
    match event_name {
      "01" => {
        social_link::github(event);
      },
      "02" => {
        social_link::my_site(event);
      },
      "03" =>{
        social_link::tauri(event);
      }
      _ => {}
    }
}


// Menu::with_items([
//     #[cfg(target_os = "macos")]
//     MenuEntry::Submenu(Submenu::new(
//       &ctx.package_info().name,
//       Menu::with_items([
//         MenuItem::About(ctx.package_info().name.clone(),aboutMe).into(),
//         MenuItem::Separator.into(),
//         MenuItem::Services.into(),
//         MenuItem::Separator.into(),
//         MenuItem::Hide.into(),
//         MenuItem::HideOthers.into(),
//         MenuItem::ShowAll.into(),
//         MenuItem::Separator.into(),
//         MenuItem::Quit.into(),
//       ]),
//     )),
//     MenuEntry::Submenu(Submenu::new(
//       "File",
//       Menu::with_items([MenuItem::CloseWindow.into()]),
//     )),
//     MenuEntry::Submenu(Submenu::new(
//       "Edit",
//       Menu::with_items([
//         MenuItem::Undo.into(),
//         MenuItem::Redo.into(),
//         MenuItem::Separator.into(),
//         MenuItem::Cut.into(),
//         MenuItem::Copy.into(),
//         MenuItem::Paste.into(),
//         #[cfg(not(target_os = "macos"))]
//         MenuItem::Separator.into(),
//         MenuItem::SelectAll.into(),
//       ]),
//     )),
//     MenuEntry::Submenu(Submenu::new(
//       "View",
//       Menu::with_items([MenuItem::EnterFullScreen.into()]),
//     )),
//     MenuEntry::Submenu(Submenu::new(
//       "Window",
//       Menu::with_items([MenuItem::Minimize.into(), MenuItem::Zoom.into()]),
//     )),
//     // You should always have a Help menu on macOS because it will automatically
//     // show a menu search field
//     MenuEntry::Submenu(Submenu::new(
//       "Help",
//       Menu::with_items([CustomMenuItem::new("Learn More", "Learn More").into()]),
//     )),
//   ])