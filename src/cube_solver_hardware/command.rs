use std::sync::mpsc;

pub(crate) struct Command {
    pub result_back_channel: mpsc::Sender<()>,
    pub command: Commands,
}

pub(crate) enum Commands {
    RotateTurntable90DegreeClockwise,
    RotateTurntable90DegreeCounterClockwise,
    MoveCarriageToFront,
    MoveCarriageToBack,
    RotateClawLeftPosition,
    RotateClawMiddlePosition,
    RotateClawRightPosition,
}

impl Command {
    pub fn new(command: Commands, result_back_channel: mpsc::Sender<()>) -> Self {
        Command {
            command,
            result_back_channel,
        }
    }
}
