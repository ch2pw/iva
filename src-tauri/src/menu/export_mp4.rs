use ndarray::Array3;
use tauri_plugin_dialog::DialogExt;
use video_rs::{Time, encode::Settings};

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

        let duration = Time::from_nth_of_a_second(24);
        let mut position = Time::zero();
        for _i in 0..100 {
            let frame = Array3::<u8>::zeros((1080, 1920, 3));
            encoder.encode(&frame, position).unwrap();
            position = position.aligned_with(duration).add();
        }

        encoder.finish().unwrap();
    });
}
