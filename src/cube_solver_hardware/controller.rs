use super::command::Command;
use super::command::Commands;
use crate::raspbi::hardware::Hardware;
use std::sync::mpsc;
use std::thread;

pub struct Controller {
    command_sender: mpsc::Sender<Command>,
}

impl Controller {
    pub fn new() -> Self {
        let (command_sender, command_receiver) = mpsc::channel::<Command>();
        let mut hardware = Hardware::new();
        let mut turntable_angle = 0u16;

        thread::spawn(move || {
            while let Ok(cmd) = command_receiver.recv() {
                match cmd.command {
                    Commands::RotateTurntable90DegreeClockwise => {
                        hardware.rotate_gpio_motor_right();
                        std::thread::sleep(std::time::Duration::from_millis(250));
                        hardware.stop_gpio_motor();
                        turntable_angle = (turntable_angle + 90) % 360;
                        cmd.result_back_channel.send(()).unwrap();
                    }
                    Commands::RotateTurntable90DegreeCounterClockwise => {
                        hardware.rotate_gpio_motor_left();
                        std::thread::sleep(std::time::Duration::from_millis(250));
                        hardware.stop_gpio_motor();
                        turntable_angle = (turntable_angle + 270) % 360;
                        cmd.result_back_channel.send(()).unwrap();
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
            .send(Command::new(
                Commands::RotateTurntable90DegreeClockwise,
                result_sender,
            ))
            .unwrap();
        result_receiver
    }

    pub fn rotate_turntable_90_degree_counter_clockwise(&mut self) -> mpsc::Receiver<()> {
        let (result_sender, result_receiver) = mpsc::channel();
        self.command_sender
            .send(Command::new(
                Commands::RotateTurntable90DegreeCounterClockwise,
                result_sender,
            ))
            .unwrap();
        result_receiver
    }
}
