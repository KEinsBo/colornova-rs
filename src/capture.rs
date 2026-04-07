use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;

pub struct ScreenCapture {
    capturer: Capturer,
    last_frame: Vec<u8>,
}

impl ScreenCapture {
    pub fn new() -> Self {
        let display = Display::primary().expect("Kein primärer Bildschirm gefunden");
        let capturer = Capturer::new(display).expect("Fehler beim Erstellen des Capturers");
        Self {
            capturer,
            last_frame: Vec::new(),
        }
    }

    pub fn get_frame(&mut self) -> &[u8] {
        match self.capturer.frame() {
            Ok(frame) => {
                self.last_frame = frame.to_vec();
                &self.last_frame
            }
            Err(err) if err.kind() == WouldBlock => &self.last_frame,
            Err(err) => panic!("Capture-Fehler: {}", err),
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.capturer.width(), self.capturer.height())
    }
}
