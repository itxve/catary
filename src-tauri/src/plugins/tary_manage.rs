use image::{imageops, ImageBuffer, Rgba};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
  sync::{Arc, Mutex},
  thread,
  time::Duration,
};
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use tauri::async_runtime::JoinHandle;
use tauri::{command, AppHandle, Icon, Manager, State};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GifData {
  File(String),
  Raw(Vec<u8>),
}

pub struct TaryItem {
  pub speed: Duration,
  pub data: GifData,
}

fn find_max_dimensions(dimensions: &[(usize, usize)]) -> (usize, usize) {
  let mut max_w: usize = 0;
  let mut max_h: usize = 0;
  for ele in dimensions {
    if ele.0 > max_w {
      max_w = ele.0
    }
    if ele.1 > max_h {
      max_h = ele.1
    }
  }
  (max_w, max_h)
}
fn resize_image(
  image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
  scale: f32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
  // 计算新的尺寸
  let (width, height) = image.dimensions();
  let new_width = (width as f32 * scale).round() as u32;
  let new_height = (height as f32 * scale).round() as u32;
  imageops::resize(image, new_width, new_height, imageops::Nearest)
}

pub struct DTaryState {
  pub running: Arc<AtomicBool>,
  pub tary_item: Arc<Mutex<TaryItem>>,
  pub join_handle: Option<JoinHandle<u8>>,
}

impl DTaryState {
  pub fn from_file(path: String) -> Self {
    Self {
      running: Arc::new(AtomicBool::new(false)),
      tary_item: Arc::new(Mutex::new(TaryItem {
        speed: Duration::from_millis(cpu_usage()),
        data: GifData::File(path),
      })),
      join_handle: None,
    }
  }

  pub fn from_raw(data: Vec<u8>) -> Self {
    Self {
      running: Arc::new(AtomicBool::new(false)),
      tary_item: Arc::new(Mutex::new(TaryItem {
        speed: Duration::from_millis(cpu_usage()),
        data: GifData::Raw(data),
      })),
      join_handle: None,
    }
  }

  pub fn run(&mut self, app: AppHandle) {
    self.running.store(true, Ordering::SeqCst);
    let canlcel = Arc::clone(&self.running);
    let app = app.clone();
    let tary_item = Arc::clone(&self.tary_item);
    if let Some(_) = &self.join_handle {
      return;
    };
    self.join_handle = Some(tauri::async_runtime::spawn(async move {
      loop {
        if !canlcel.load(Ordering::SeqCst) {
          // 暂停时，释放cpu占用
          thread::sleep(Duration::from_millis(50));
          continue;
        }

        let item = match *tary_item.lock().unwrap() {
          TaryItem { speed, ref data } => match data {
            GifData::File(path) => {
              let f_data = tauri::api::file::read_binary(Path::new(path)).unwrap();
              (speed, f_data)
            }
            GifData::Raw(data) => (speed, data.clone()),
          },
        };

        // Configure the decoder such that it will expand the image to RGBA.
        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);
        // 找到最大的宽和高
        let mut wh: Vec<(usize, usize)> = vec![];
        let decoder = options
          .clone()
          .read_info(Cursor::new(item.1.clone()))
          .unwrap();
        for (_i, frame) in decoder.into_iter().enumerate() {
          let frame = frame.unwrap();
          wh.push((frame.width as usize, frame.height as usize))
        }
        let dimensions = find_max_dimensions(&wh);
        let width: u32 = dimensions.0 as u32;
        let height: u32 = dimensions.1 as u32;
        // 读取图片帧
        let decoder = options.read_info(Cursor::new(item.1)).unwrap();
        for (_i, frame) in decoder.into_iter().enumerate() {
          let frame = frame.unwrap();

          // 创建一个`ImageBuffer`
          let mut image_buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
            frame.width as u32,
            frame.height as u32,
            frame.buffer.to_vec(),
          )
          .unwrap();
          // 调整gif 帧图片大小,解决抖动问题
          if frame.width as u32 != width || frame.height as u32 != height {
            // 创建一个透明的背景图片
            let mut new_img = ImageBuffer::from_fn(width, height, |_, _| Rgba([0, 0, 0, 0]));
            // 计算位置，这里以居中为例
            let x_offset = (width - frame.width as u32) / 2;
            let y_offset = (height - frame.height as u32) / 2;
            // 将原始帧的内容复制到新 ImageBuffer 的适当位置
            for (x, y, pixel) in image_buffer.enumerate_pixels() {
              new_img.put_pixel(x + x_offset, y + y_offset, *pixel);
            }
            // 使用新的 ImageBuffer 替换原始帧
            image_buffer = new_img;
            // todo 适当剪裁以放大图片
          }
          let app = app.tray_handle();
          let _ = app.set_icon(Icon::Rgba {
            rgba: image_buffer.clone().into_vec(),
            width: image_buffer.width(),
            height: image_buffer.height(),
          });
          thread::sleep(item.0);
        }
      }
    }));
  }

  pub fn set_speed(&mut self, spped: Duration) {
    self.tary_item.lock().unwrap().speed = spped;
  }

  pub fn set_gif(&mut self, gif_data: GifData) {
    self.tary_item.lock().unwrap().data = gif_data
  }

  pub fn set_run(&mut self, run: bool) {
    self.running.store(run, Ordering::SeqCst);
  }
}

pub fn cpu_usage() -> u64 {
  let s = System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
  let prec = (s.global_cpu_info().cpu_usage() * s.cpus().len() as f32).floor();
  prec as u64
}

pub fn _play(app: AppHandle) {
  let app_run = app.clone();
  let dt_state: State<Arc<Mutex<DTaryState>>> = app.state();
  dt_state.lock().unwrap().run(app_run);
}

pub fn _pause(app: AppHandle) {
  let dt_state: State<Arc<Mutex<DTaryState>>> = app.state();
  dt_state.lock().unwrap().set_run(false);
}

pub fn _set_speed(app: AppHandle, speed: Duration) {
  let dt_state: State<Arc<Mutex<DTaryState>>> = app.state();
  dt_state.lock().unwrap().set_speed(speed);
}

pub fn _change_tary(app: AppHandle, buffer: GifData) {
  let dt_state: State<Arc<Mutex<DTaryState>>> = app.state();
  let _ = dt_state.lock().unwrap().set_gif(buffer);
  _play(app.clone());
}

/** commands */
#[command]
pub fn play(app: AppHandle) {
  _play(app)
}

#[command]
pub fn pause(app: AppHandle) {
  _pause(app)
}

#[command]
pub fn set_speed_millis(app: AppHandle, speed: u64) {
  _set_speed(app, Duration::from_millis(speed))
}

#[command]
pub fn set_speed_secs(app: AppHandle, speed: u64) {
  _set_speed(app, Duration::from_secs(speed))
}

#[command]
pub fn change_tary(app: AppHandle, buffer: GifData) {
  _change_tary(app, buffer);
}

#[command]
pub fn set_template_icon(app: AppHandle, template: bool) {
  #[cfg(target_os = "macos")]
  let _ = app.tray_handle().set_icon_as_template(template);
}
