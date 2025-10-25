use std::sync::mpsc;
use std::thread;

pub struct Controller {
    tx: mpsc::Sender<ControllerCommand>,
}

enum ControllerCommand {
    RotateTurntable90DegreeClockwise,
}

impl Controller {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let mut hardware = super::rasbpi::Rasbpi::new();
        let mut turntable_angle = 0u16;

        thread::spawn(move || {
            while let Ok(cmd) = rx.recv() {
                match cmd {
                    ControllerCommand::RotateTurntable90DegreeClockwise => {
                        hardware.rotate_gpio_motor_right();
                        turntable_angle = (turntable_angle + 90) % 360;
                    }
                }
            }
        });

        Controller { tx }
    }

    pub fn rotate_turntable_90_degree_clockwise(&mut self) -> mpsc::Receiver<()> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            self.hardware.rotate_gpio_motor_right();
            self.turntable_angle = (self.turntable_angle + 90) % 360;
            tx.send(()).unwrap();
        });

        rx
    }
}
