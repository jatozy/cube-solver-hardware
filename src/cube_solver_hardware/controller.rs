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
                        execute_rotate_turntable_90_degree_clockwise(
                            &mut hardware,
                            &mut turntable_angle,
                        );
                        cmd.result_back_channel.send(()).unwrap();
                    }
                    Commands::RotateTurntable90DegreeCounterClockwise => {
                        execute_rotate_turntable_90_degree_counter_clockwise(
                            &mut hardware,
                            &mut turntable_angle,
                        );
                        cmd.result_back_channel.send(()).unwrap();
                    }
                    Commands::MoveCarriageToFront => {
                        execute_move_carriage_to_front(&mut hardware);
                        cmd.result_back_channel.send(()).unwrap();
                    }
                    Commands::MoveCarriageToBack => {
                        execute_move_carriage_to_back(&mut hardware);
                        cmd.result_back_channel.send(()).unwrap();
                    }
                    Commands::RotateClawLeftPosition => {
                        execute_rotate_claw_to_left_position(&mut hardware);
                        cmd.result_back_channel.send(()).unwrap();
                    }
                    Commands::RotateClawMiddlePosition => {
                        execute_rotate_claw_to_middle_position(&mut hardware);
                        cmd.result_back_channel.send(()).unwrap();
                    }
                    Commands::RotateClawRightPosition => {
                        execute_rotate_claw_to_right_position(&mut hardware);
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

    pub fn move_carriage_to_front(&mut self) -> mpsc::Receiver<()> {
        let (result_sender, result_receiver) = mpsc::channel();
        self.command_sender
            .send(Command::new(Commands::MoveCarriageToFront, result_sender))
            .unwrap();
        result_receiver
    }

    pub fn move_carriage_to_back(&mut self) -> mpsc::Receiver<()> {
        let (result_sender, result_receiver) = mpsc::channel();
        self.command_sender
            .send(Command::new(Commands::MoveCarriageToBack, result_sender))
            .unwrap();
        result_receiver
    }

    pub fn rotate_claw_left_position(&mut self) -> mpsc::Receiver<()> {
        let (result_sender, result_receiver) = mpsc::channel();
        self.command_sender
            .send(Command::new(
                Commands::RotateClawLeftPosition,
                result_sender,
            ))
            .unwrap();
        result_receiver
    }

    pub fn rotate_claw_middle_position(&mut self) -> mpsc::Receiver<()> {
        let (result_sender, result_receiver) = mpsc::channel();
        self.command_sender
            .send(Command::new(
                Commands::RotateClawMiddlePosition,
                result_sender,
            ))
            .unwrap();
        result_receiver
    }

    pub fn rotate_claw_right_position(&mut self) -> mpsc::Receiver<()> {
        let (result_sender, result_receiver) = mpsc::channel();
        self.command_sender
            .send(Command::new(
                Commands::RotateClawRightPosition,
                result_sender,
            ))
            .unwrap();
        result_receiver
    }
}

fn execute_rotate_turntable_90_degree_clockwise(hardware: &mut Hardware, angle: &mut u16) {
    hardware.rotate_turntable_right();
    std::thread::sleep(std::time::Duration::from_millis(250));
    hardware.stop_turntable();
    *angle = (*angle + 90) % 360;
}

fn execute_rotate_turntable_90_degree_counter_clockwise(hardware: &mut Hardware, angle: &mut u16) {
    hardware.rotate_turntable_left();
    std::thread::sleep(std::time::Duration::from_millis(250));
    hardware.stop_turntable();
    *angle = (*angle + 270) % 360;
}

fn execute_move_carriage_to_front(hardware: &mut Hardware) {
    hardware.move_carriage_forward();
    loop {
        if hardware.is_carriage_at_front_position() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    hardware.stop_carriage();
}

fn execute_move_carriage_to_back(hardware: &mut Hardware) {
    hardware.move_carriage_backward();
    loop {
        if hardware.is_carriage_at_back_position() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    hardware.stop_carriage();
}

fn execute_rotate_claw_to_left_position(hardware: &mut Hardware) {
    hardware.rotate_claw_to_angle(150);
}

fn execute_rotate_claw_to_middle_position(hardware: &mut Hardware) {
    hardware.rotate_claw_to_angle(350);
}

fn execute_rotate_claw_to_right_position(hardware: &mut Hardware) {
    hardware.rotate_claw_to_angle(650);
}
