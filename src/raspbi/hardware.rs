use rppal::gpio::Gpio;

const GPIO_TURNTABLE_MOTOR_LEFT: u8 = 5;
const GPIO_TURNTABLE_MOTOR_RIGHT: u8 = 6;
const GPIO_CARRIAGE_MOTOR_FORWARD: u8 = 17;
const GPIO_CARRIAGE_MOTOR_BACKWARD: u8 = 27;
const GPIO_CARRIAGE_FRONT_POSITION: u8 = 23;
const GPIO_CARRIAGE_BACK_POSITION: u8 = 24;

pub(crate) struct Hardware {
    turntable_motor_left_channel: rppal::gpio::OutputPin,
    turntable_motor_right_channel: rppal::gpio::OutputPin,
    carriage_motor_forward_channel: rppal::gpio::OutputPin,
    carriage_motor_backward_channel: rppal::gpio::OutputPin,
    carriage_front_position_channel: rppal::gpio::InputPin,
    carriage_back_position_channel: rppal::gpio::InputPin,
}

impl Hardware {
    pub fn new() -> Self {
        let gpio = Gpio::new().unwrap();

        Hardware {
            turntable_motor_left_channel: gpio
                .get(GPIO_TURNTABLE_MOTOR_LEFT)
                .unwrap()
                .into_output_low(),
            turntable_motor_right_channel: gpio
                .get(GPIO_TURNTABLE_MOTOR_RIGHT)
                .unwrap()
                .into_output_low(),
            carriage_motor_forward_channel: gpio
                .get(GPIO_CARRIAGE_MOTOR_FORWARD)
                .unwrap()
                .into_output_low(),
            carriage_motor_backward_channel: gpio
                .get(GPIO_CARRIAGE_MOTOR_BACKWARD)
                .unwrap()
                .into_output_low(),
            carriage_front_position_channel: gpio
                .get(GPIO_CARRIAGE_FRONT_POSITION)
                .unwrap()
                .into_input_pullup(),
            carriage_back_position_channel: gpio
                .get(GPIO_CARRIAGE_BACK_POSITION)
                .unwrap()
                .into_input_pullup(),
        }
    }

    pub fn rotate_turntable_left(&mut self) {
        self.turntable_motor_right_channel.set_low();
        self.wait_for_short_time();
        self.turntable_motor_left_channel.set_high();
    }

    pub fn rotate_turntable_right(&mut self) {
        self.turntable_motor_left_channel.set_low();
        self.wait_for_short_time();
        self.turntable_motor_right_channel.set_high();
    }

    pub fn stop_turntable(&mut self) {
        self.turntable_motor_left_channel.set_low();
        self.turntable_motor_right_channel.set_low();
    }

    pub fn move_carriage_forward(&mut self) {
        self.carriage_motor_backward_channel.set_low();
        self.wait_for_short_time();
        self.carriage_motor_forward_channel.set_high();
    }

    pub fn move_carriage_backward(&mut self) {
        self.carriage_motor_forward_channel.set_low();
        self.wait_for_short_time();
        self.carriage_motor_backward_channel.set_high();
    }

    pub fn stop_carriage(&mut self) {
        self.carriage_motor_forward_channel.set_low();
        self.carriage_motor_backward_channel.set_low();
    }

    pub fn is_carriage_at_front_position(&self) -> bool {
        self.carriage_front_position_channel.is_low()
    }

    pub fn is_carriage_at_back_position(&self) -> bool {
        self.carriage_back_position_channel.is_low()
    }

    fn wait_for_short_time(&self) {
        //Wait a short time to be shure, that the hardware has processed the changes.
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
