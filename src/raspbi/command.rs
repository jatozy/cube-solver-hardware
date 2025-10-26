use std::sync::mpsc;

pub(crate) struct Command {
    pub result_back_channel: mpsc::Sender<()>,
    pub command: Commands,
}

pub(crate) enum Commands {
    RotateTurntable90DegreeClockwise,
}
