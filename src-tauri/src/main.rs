// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::Rng;
use tauri::{window, Manager, PhysicalPosition, Position};

// variable signals where we are in the prank, index wise
static mut INDEX: usize = 0;

// getZeData is a tauri command that returns the data from the json file
#[tauri::command]
fn getZeData() -> String {
    // read the file
    let data = std::fs::read_to_string("/Users/danielkhankin/data.json").unwrap();
    // and returns the object that corresponds to the index
    let json: serde_json::Value = serde_json::from_str(&data).unwrap();

    return json[unsafe { INDEX }].to_string();
}

// next is a tauri command that increments the index, closes the current window and opens another one, until the end of the prank, which is at length of json array.
// it also places the window at random coordinates on the screen
#[tauri::command]
fn nextPrank(window: tauri::Window) {
    // read the file
    let data = std::fs::read_to_string("/Users/danielkhankin/data.json").unwrap();

    let json: serde_json::Value = serde_json::from_str(&data).unwrap();

    // increment the index
    unsafe {
        INDEX += 1;
    }

    // use tauri to get the current monitor size
    // let monitor = tauri::window::Monitor::size();
    // get the width and height of the monitor
    // let width = monitor.width;
    // let height = monitor.height;
    // subtract it by the size of our window so that it doesn't go out of bounds
    // let width = width - 300;
    // let height = height - 300;
    let width = 1920;
    let height = 1080 - 300;
    // random
    let x = rand::thread_rng().gen_range(0..(width - 300));
    let y = rand::thread_rng().gen_range(0..(height - 300));
    // get current window

    window
        .set_position(Position::Physical(PhysicalPosition { x, y }))
        .unwrap();

    // if we are not at the end of the prank
    if unsafe { INDEX } >= json.as_array().unwrap().len() {
        return window.close().unwrap();
    }

    return;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![getZeData, nextPrank])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
