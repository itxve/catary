use std::fs;

use super::config::{self, AppConf};
use tauri::AppHandle;

use super::menu;
use crate::plugins;
use crate::plugins::tary_manage::GifData;
use crate::utils;

#[tauri::command]
pub fn refresh_tray_menu(app: AppHandle) -> () {
  let _ = app.tray_handle().set_menu(menu::init_tray_menu());
}

#[tauri::command]
pub fn app_info(app: AppHandle) -> config::AppInfo {
  let conf = config::AppConf::read();
  config::AppInfo {
    home_dir: utils::app_root().to_string_lossy().into(),
    resource_dir: utils::resource_root(app),
    current_gif: conf.select.id,
  }
}

#[tauri::command]
pub fn add_image(item: config::GIfItem) {
  let mut conf = AppConf::read();
  conf.items.insert(0, item.clone());
  conf.select = item;
  conf.write();
}

#[tauri::command]
pub fn user_gifs() -> Vec<config::GIfItem> {
  let conf = AppConf::read();
  conf.items
}

#[tauri::command]
pub fn set_current_gif(app: AppHandle, data: config::GIfItem) {
  let mut conf = AppConf::read();
  conf.select = data.clone();
  plugins::tary_manage::_change_tary(
    app,
    GifData::File(format!(
      "{}/{}.gif",
      utils::app_root().to_string_lossy(),
      &data.id
    )),
  );
  conf.write();
}

#[tauri::command]
pub fn del_current_gif(app: AppHandle, data: config::GIfItem) {
  let mut conf = AppConf::read();
  conf.items = conf
    .items
    .into_iter()
    .filter(|it| it.id != data.id)
    .collect();

  plugins::tary_manage::_pause(app.clone());
  conf.select = config::GIfItem::new();
  let _ = match fs::remove_file(format!(
    "{}/{}.gif",
    utils::app_root().to_string_lossy(),
    &data.id
  )) {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string()),
  };
  conf.write();
}
