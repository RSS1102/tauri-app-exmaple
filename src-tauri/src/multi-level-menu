// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{image::Image, menu::{CheckMenuItemBuilder, IconMenuItemBuilder, MenuBuilder, SubmenuBuilder}};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let file_menu = SubmenuBuilder::new(app, "File")
                .text("open", "Open")
                .text("quit", "Quit")
                .build()?;

            let lang_str = "en";
            let check_sub_item_1 = CheckMenuItemBuilder::new("English")
                .id("en")
                .checked(lang_str == "en")
                .build(app)?;

            let check_sub_item_2 = CheckMenuItemBuilder::new("Chinese")
                .id("en")
                .checked(lang_str == "en")
                .enabled(false)
                .build(app)?;

            let icon_image = Image::from_bytes(include_bytes!("../icons/icon.png")).unwrap();

            let icon_item = IconMenuItemBuilder::new("icon")
                .icon(icon_image)
                .build(app)?;

            let other_item = SubmenuBuilder::new(app, "language")
                .item(&check_sub_item_1)
                .item(&check_sub_item_2)
                .build()?;

            let menu = MenuBuilder::new(app)
                .items(&[&file_menu, &other_item, &icon_item])
                .build()?;

            app.set_menu(menu)?;

            print!("Hello from setup");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
