use std::sync::mpsc;
use std::thread;

pub struct Controller {
    command_sender: mpsc::Sender<ControllerCommand>,
}

impl Controller {
    pub fn new() -> Self {
        let (command_sender, command_receiver) = mpsc::channel();
        let mut hardware = super::rasbpi::Rasbpi::new();
        let mut turntable_angle = 0u16;

        thread::spawn(move || {
            while let Ok(cmd) = command_receiver.recv() {
                match cmd {
                    ControllerCommand::RotateTurntable90DegreeClockwise => {
                        hardware.rotate_gpio_motor_right();
                        turntable_angle = (turntable_angle + 90) % 360;
                    }
                }
            }
        });

        Controller {
            command_sender: command_sender,
        }
    }

    pub fn rotate_turntable_90_degree_clockwise(&mut self) -> mpsc::Receiver<()> {
        let (result_sender, result_receiver) = mpsc::channel();
        self.command_sender
            .send({ControllerCommand::RotateTurntable90DegreeClockwise, result_sender)
            .unwrap();
        result_receiver
    }
}
