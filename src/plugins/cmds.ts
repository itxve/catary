import { invoke } from "@tauri-apps/api/tauri";
import * as fs from "@tauri-apps/api/fs";

export type GifItem = {
  id: string;
  desc: string;
};

export function refresh_tray_menu() {
  invoke("refresh_tray_menu");
}

export function play() {
  invoke("play");
}
export async function pause() {
  await invoke("pause");
}

export function set_speed_millis(speed: number = 200) {
  invoke("set_speed_millis", { speed });
}
export function set_speed_secs(speed: number = 200) {
  invoke("set_speed_secs", { speed });
}

export function change_tary(data: Array<number> | string) {
  if (Array.isArray(data)) {
    invoke("change_tary", { buffer: { Raw: data } });
  } else {
    invoke("change_tary", { buffer: { File: data } });
  }
}

export async function app_info() {
  return invoke<{
    resource_dir: string;
    home_dir: string;
    current_gif: string;
  }>("app_info");
}

export async function writeBinaryFile2Home(path: string, file: ArrayBuffer) {
  return fs.writeBinaryFile(path, file);
}

export async function readBinaryFile2Home(path: string) {
  return fs.readBinaryFile(path);
}

export async function user_gifs() {
  return invoke<GifItem>("user_gifs");
}

export async function set_template_icon(template: boolean) {
  return invoke("set_template_icon", { template });
}

export async function add_image(item: GifItem) {
  item.desc = item.desc || "🚀";
  return invoke("add_image", { item });
}

export async function set_current_gif(item: GifItem) {
  item.desc = item.desc || "🚀";
  return invoke("set_current_gif", { data: item });
}

export async function del_current_gif(item: GifItem) {
  item.desc = item.desc || "🚀";
  return invoke("del_current_gif", { data: item });
}

export default {
  app_info,
  refresh_tray_menu,
  play,
  pause,
  set_speed_millis,
  set_speed_secs,
  change_tary,
  writeBinaryFile2Home,
  readBinaryFile2Home,
  user_gifs,
  add_image,
  set_template_icon,
  set_current_gif,
  del_current_gif,
};
