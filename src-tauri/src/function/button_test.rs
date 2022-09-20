/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: button_test.rs                       */
/*       ## ##                                                */
/*                    C: 2022/06/15 16:14:24 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/06/15 16:34:31 by:dnettoRaw     */
/*    ###########                                             */

#[tauri::command]
pub fn my_button(){
    println!("banana?");
}