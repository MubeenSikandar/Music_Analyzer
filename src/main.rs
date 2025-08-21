use raylib::prelude::*;

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
    }
}
