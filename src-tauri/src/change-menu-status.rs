// change-menu-status
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    image::Image,
    menu::{CheckMenuItemBuilder, IconMenuItem, MenuBuilder, MenuItem, SubmenuBuilder},
};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let check_sub_item_en = CheckMenuItemBuilder::with_id("en", "EN")
                .checked(true)
                .build(app)?;

            let check_sub_item_zh = CheckMenuItemBuilder::with_id("zh", "ZH")
                .checked(false)
                .build(app)?;

            let text_menu = MenuItem::with_id(
                app,
                "change_text",
                &"Change menu".to_string(),
                true,
                Some("Ctrl+Z"),
            )
            .unwrap();

            let icon_menu = IconMenuItem::with_id(
                app,
                "change_icon",
                &"Change icon menu",
                true,
                Some(Image::from_bytes(include_bytes!("../icons/icon.png")).unwrap()),
                Some("Ctrl+F"),
            )
            .unwrap();

            let menu_item = SubmenuBuilder::new(app, "Change menu")
                .item(&text_menu)
                .item(&icon_menu)
                .items(&[&check_sub_item_en, &check_sub_item_zh])
                .build()?;
            let menu = MenuBuilder::new(app).items(&[&menu_item]).build()?;
            app.set_menu(menu)?;
            app.on_menu_event(move |_app_handle: &tauri::AppHandle, event| {
                match event.id().0.as_str() {
                    "change_text" => {
                        text_menu
                            .set_text("changed menu text")
                            .expect("Change text error");

                        text_menu
                            .set_text("changed menu text")
                            .expect("Change text error");
                    }
                    "change_icon" => {
                        icon_menu
                            .set_text("changed menu-icon text")
                            .expect("Change text error");
                        icon_menu
                            .set_icon(Some(
                                Image::from_bytes(include_bytes!("../icons/icon-2.png")).unwrap(),
                            ))
                            .expect("Change icon error");
                    }

                    "en" | "zh" => {
                        check_sub_item_en
                            .set_checked(event.id().0.as_str() == "en")
                            .expect("Change check error");
                        check_sub_item_zh
                            .set_checked(event.id().0.as_str() == "zh")
                            .expect("Change check error");
                        check_sub_item_zh.set_accelerator(Some("Ctrl+L"))
                        .expect("Change accelerator error");
                    }
                    _ => {
                        println!("unexpected menu event");
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
