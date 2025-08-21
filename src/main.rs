use raylib::{ffi::GetRenderWidth, prelude::*};
use std::sync::Mutex;

static GLOBAL_FRAMES: Mutex<[u32; 1024]> = Mutex::new([0; 1024]);
static GLOBAL_FRAMES_COUNT: Mutex<usize> = Mutex::new(0);

fn callback(buffer_data: &[u32], mut frames: usize) {
    if frames > 1024 {
        frames = 1024;
    }

    let mut global_frames = GLOBAL_FRAMES.lock().unwrap();
    let mut count = GLOBAL_FRAMES_COUNT.lock().unwrap();

    global_frames[..frames].copy_from_slice(&buffer_data[..frames]);
    *count = frames;
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Music Analyzer")
        .build();
    let audio_device = RaylibAudio::init_audio_device().expect("Failed to init audio");
    let ready: bool = audio_device.is_audio_device_ready();

    let music = audio_device
        .new_music("resources/Mild High Club - Homage.mp3")
        .expect("Failed to Load Music");

    music.play_stream();

    println!("Audio device ready? {}", ready);

    while !rl.window_should_close() {
        music.update_stream();

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if music.is_stream_playing() {
                music.pause_stream();
            } else {
                music.play_stream();
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_circle_gradient(320, 200, 220.5, Color::DARKRED, Color::BLACK);

        d.draw_text("Press SPACE to toggle music!", 170, 200, 20, Color::WHITE);
        d.draw_ellipse(321, 200, 120.0, 60.0, Color::PINK);
    }
}
