use std::sync::Mutex;

use image::{RgbImage, buffer::ConvertBuffer};
use ndarray::Array3;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use video_rs::{Time, encode::Settings};

use crate::AppState;

pub fn export_mp4(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let Some(file_path) = app
            .dialog()
            .file()
            .add_filter("mp4", &["mp4"])
            .blocking_save_file()
        else {
            return;
        };

        let mut encoder = video_rs::Encoder::new(
            file_path.as_path().unwrap(),
            Settings::preset_h264_yuv420p(1920, 1080, false),
        )
        .unwrap();

        let layers = app
            .state::<Mutex<AppState>>()
            .lock()
            .unwrap()
            .layers
            .clone();

        let frame_duration = Time::from_nth_of_a_second(24);
        let mut time = Time::zero();
        for _i in 0..100 {
            let frame: RgbImage =
                iva_core::render(&layers, (time.as_secs_f64() * 1000.0) as u64).convert();
            let frame = Array3::<u8>::from_shape_vec((1080, 1920, 3), frame.to_vec()).unwrap();
            encoder.encode(&frame, time).unwrap();
            time = time.aligned_with(frame_duration).add();
        }

        encoder.finish().unwrap();
    });
}
