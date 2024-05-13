#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod app;
mod plugins;
mod utils;

use std::sync::{Arc, Mutex};

use app::{commands, config::AppConf, menu};
use log::info;
use tauri::Manager;

#[tokio::main]
async fn main() {
  let context = tauri::generate_context!();
  let app = tauri::Builder::default();
  let _app_conf = AppConf::read().write();

  app
    .setup(move |_app| {
      info!("app running...");
      let mut tary_state = plugins::tary_manage::DTaryState::from_file("".to_owned());
      if "" != &_app_conf.select.id {
        tary_state.set_gif(plugins::tary_manage::GifData::File(format!(
          "{}/{}.gif",
          utils::app_root().to_string_lossy(),
          &_app_conf.select.id
        )));
        tary_state.run(_app.app_handle());
      }
      _app.manage(Arc::new(Mutex::new(tary_state)));
      Ok(())
    })
    .system_tray(menu::init_tray())
    .on_system_tray_event(menu::tray_handler)
    .menu(menu::init_menu())
    .on_menu_event(menu::menu_handler)
    .invoke_handler(tauri::generate_handler![
      commands::refresh_tray_menu,
      commands::app_info,
      commands::add_image,
      commands::user_gifs,
      commands::set_current_gif,
      commands::del_current_gif,
      plugins::tary_manage::play,
      plugins::tary_manage::pause,
      plugins::tary_manage::set_speed_millis,
      plugins::tary_manage::set_speed_secs,
      plugins::tary_manage::change_tary,
      plugins::tary_manage::set_template_icon
    ])
    .build(context)
    .expect("error while running catary application")
    .run(|_app_handle, event| match event {
      tauri::RunEvent::ExitRequested { api, .. } => {
        println!("last close"); //最后一个窗口关闭不退出程序
        api.prevent_exit();
      }
      _ => {}
    })
}
