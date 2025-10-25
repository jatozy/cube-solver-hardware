use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct Controller {
    turntable_angle: u16,
}

impl Controller {
    pub fn new() -> Self {
        Controller { turntable_angle: 0 }
    }
}

impl Controller {
    pub fn rotate_turntable_clockwise(&mut self, angle: u16) -> std::sync::mpsc::Receiver<()> {
        self.turntable_angle = angle;
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(500));
            let _ = tx.send(()).unwrap();
        });

        rx
    }
}
