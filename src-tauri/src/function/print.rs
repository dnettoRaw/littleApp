/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: print.rs                             */
/*       ## ##                                                */
/*                    C: 2022/10/17 14:28:58 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/10/17 14:40:04 by:dnettoRaw     */
/*    ###########                                             */

// Disable warnings

#[allow(unused_macros)]

// The debug version

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

// Non-debug version

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log {
    ($( $args:expr ),*) => {()}
}