
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// Do not import above import below
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {

    scrabblechecker_lib::run()

}
