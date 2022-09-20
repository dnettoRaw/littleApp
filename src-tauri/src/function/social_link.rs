/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: social_link.rs                       */
/*       ## ##                                                */
/*                    C: 2022/06/15 16:14:12 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/06/15 16:14:14 by:dnettoRaw     */
/*    ###########                                             */


use tauri::{Manager};
use tauri::api::shell;

pub fn github(event: tauri::WindowMenuEvent)
{
    let url = "https://github.com/dnettoRaw/tauri".to_string();
    shell::open(&event.window().shell_scope(), url, None).unwrap();
}
pub fn my_site(event: tauri::WindowMenuEvent)
{
    let url = "https://dnetto.dev".to_string();
    shell::open(&event.window().shell_scope(), url, None).unwrap();
}
pub fn tauri(event: tauri::WindowMenuEvent)
{
    let url = "https://github.com/tauri/tauri".to_string();
    shell::open(&event.window().shell_scope(), url, None).unwrap();
}