/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: build.rs                             */
/*       ## ##                                                */
/*                    C: 2022/06/15 16:11:41 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/06/19 09:17:23 by:dnettoRaw     */
/*    ###########                                             */

// #[allow(unused_imports)]
// use tauri_bundler::{try_build, Attributes, WindowsAttributes};

// fn main() {
//   if let Err(error) = try_build(
//     // #[cfg(any(target_os = "linux", target_os = "macos"))]
//     Attributes::new().windows_attributes(WindowsAttributes::new().window_icon_path("../icons/icon.ico")),
//     // #[cfg(target_os = "windows")]
//     // Attributes::new().windows_attributes(WindowsAttributes::new().window_icon_path("..\\icons\\icon.ico")),
//   ) {
//     panic!("error found during tauri-build: {:#?}", error);
//   }
// }

#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
  if std::path::Path::new("icons/icon.ico").exists() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icons/icon.ico");
    res.compile().expect("Unable to find visual studio tools");
  } else {
    panic!("No Icon.ico found. Please add one or check the path");
  }
}

#[cfg(not(windows))]
fn main() {}