use eframe::egui;
use egui::Ui;
use libmpv2::Mpv;
use std::sync::{Arc, Mutex};

struct VideoPlayerApp {
    mpv: Arc<Mutex<Mpv>>,
    is_playing: bool,
    video_path: String,
    status_message: String,
}

impl VideoPlayerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Initialize libmpv
        let mpv = Mpv::new().expect("Failed to initialize libmpv instance");

        // Ensure audio is available on macOS, even when the app is embedded.
        let _ = mpv.set_property("ao", "coreaudio");
        // let _ = mpv.set_property("audio-device", "default");
        let _ = mpv.set_property("mute", false);
        let _ = mpv.set_property("volume", 100.0);

        Self {
            mpv: Arc::new(Mutex::new(mpv)),
            is_playing: false,
            video_path: String::from("video.mp4"), // Default file path
            status_message: String::from("Ready"),
        }
    }

    fn start_playback(&mut self) {
        if let Ok(mpv) = self.mpv.lock() {
            match mpv.command("loadfile", &[&self.video_path]) {
                Ok(_) => {
                    let _ = mpv.set_property("pause", false);
                    self.is_playing = true;
                    self.status_message = format!("Playing: {}", self.video_path);
                }
                Err(err) => {
                    self.status_message = format!("Error loading video: {}", err);
                }
            }
        }
    }

    fn stop_playback(&mut self) {
        if let Ok(mpv) = self.mpv.lock() {
            let _ = mpv.set_property("pause", true);
            let _ = mpv.command("stop", &[]);
            self.is_playing = false;
            self.status_message = String::from("Stopped");
        }
    }
}

impl eframe::App for VideoPlayerApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        // Request continuous UI updates when playing to keep state synchronized
        if self.is_playing {
            ui.request_repaint();
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Rust Video Player");
            ui.add_space(10.0);

            // 1. Video Path Input
            ui.horizontal(|ui| {
                ui.label("Video File:");
                ui.text_edit_singleline(&mut self.video_path);
            });

            ui.add_space(15.0);

            // 2. Playback Control Buttons
            ui.horizontal(|ui| {
                // START / PLAY Button
                if ui
                    .add_enabled(!self.is_playing, egui::Button::new("▶ Play"))
                    .clicked()
                {
                    self.start_playback();
                }

                // STOP Button
                if ui
                    .add_enabled(self.is_playing, egui::Button::new("⏹ Stop"))
                    .clicked()
                {
                    self.stop_playback();
                }
            });

            ui.add_space(15.0);
            ui.separator();

            // 3. Status Readout
            ui.label(format!("Status: {}", self.status_message));

            // Placeholder area where external rendering or overlay canvas sits
            ui.add_space(20.0);
            let (rect, _response) = ui.allocate_at_least(
                egui::vec2(ui.available_width(), 300.0),
                egui::Sense::hover(),
            );
            ui.painter()
                .rect_filled(rect, 0.0, egui::Color32::from_gray(30));
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Video Viewport Area",
                egui::FontId::proportional(16.0),
                egui::Color32::WHITE,
            );
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 480.0])
            .with_title("Rust Video Player with egui"),
        ..Default::default()
    };

    eframe::run_native(
        "Video Player App",
        options,
        Box::new(|cc| Ok(Box::new(VideoPlayerApp::new(cc)))),
    )
}
