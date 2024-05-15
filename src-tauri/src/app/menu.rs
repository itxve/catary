use crate::utils;

use tauri::{
  AppHandle, CustomMenuItem, Menu, Submenu, SystemTray, SystemTrayEvent, SystemTrayMenu,
  WindowMenuEvent, Wry,
};
use tauri::{Manager, MenuItem};

pub fn init_menu() -> Menu {
  let name = "catary";
  let app_menu = Submenu::new(
    name,
    Menu::with_items([
      CustomMenuItem::new("check_update", "检查更新").into(),
      MenuItem::Services.into(),
      MenuItem::Hide.into(),
      MenuItem::Separator.into(),
      MenuItem::Quit.into(),
    ]),
  );
  Menu::new().add_submenu(app_menu)
}

pub fn menu_handler(event: WindowMenuEvent<tauri::Wry>) {
  let win = Some(event.window()).unwrap();
  let app = win.app_handle();
  let menu_id = event.menu_item_id();
  match menu_id {
    "check_update" => {
      utils::run_check_update(app, false, None);
    }
    _ => (),
  }
}

pub fn init_tray_menu() -> SystemTrayMenu {
  let mut tray_menu = SystemTrayMenu::new();
  tray_menu = tray_menu.add_item(CustomMenuItem::new("config".to_owned(), "设置"));
  tray_menu = tray_menu.add_item(CustomMenuItem::new("exit".to_owned(), "退出"));
  tray_menu
}

pub fn init_tray() -> SystemTray {
  SystemTray::new().with_menu(init_tray_menu())
}

pub fn tray_handler(app: &AppHandle<Wry>, event: SystemTrayEvent) {
  if let SystemTrayEvent::MenuItemClick { id, tray_id, .. } = event {
    if id == "config" {
      match app.get_window("main") {
        Some(w) => {
          w.set_focus().unwrap();
          w.show().unwrap();
        }
        None => {
          let _ =
            tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
              .title("设置")
              .maximizable(false)
              .min_inner_size(500f64, 600f64)
              .max_inner_size(500f64, 600f64)
              .enable_clipboard_access()
              .disable_file_drop_handler() // 不禁用会导致前端 drop事件不回调
              .build()
              .unwrap();
        }
      };
      return;
    }

    match id.as_str() {
      "exit" => {
        std::process::exit(0);
      }
      _ => (),
    }
  }
}
