// Prevents additional console window on Windows in Release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    client_lib::run()
}
