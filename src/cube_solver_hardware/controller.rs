use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct Controller {
    turntable_angle: u16,
    hardware: super::rasbpi::Rasbpi,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            turntable_angle: 0,
            hardware: super::rasbpi::Rasbpi::new(),
        }
    }
}

impl Controller {
    pub fn rotate_turntable_90_degree_clockwise(&mut self) -> std::sync::mpsc::Receiver<()> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            self.hardware.rotate_gpio_motor_right();
            self.turntable_angle = (self.turntable_angle + 90) % 360;
            tx.send(()).unwrap();
        });

        rx
    }
}
