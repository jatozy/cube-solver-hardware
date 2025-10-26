use rppal::gpio::Gpio;

const GPIO_TURNTABLE_MOTOR_LEFT: u8 = 5;
const GPIO_TURNTABLE_MOTOR_RIGHT: u8 = 6;

pub(crate) struct Hardware {
    gpio_motor_left_channel: rppal::gpio::OutputPin,
    gpio_motor_right_channel: rppal::gpio::OutputPin,
}

impl Hardware {
    pub fn new() -> Self {
        let gpio = Gpio::new().unwrap();

        Hardware {
            gpio_motor_left_channel: gpio.get(GPIO_TURNTABLE_MOTOR_LEFT).unwrap().into_output(),
            gpio_motor_right_channel: gpio.get(GPIO_TURNTABLE_MOTOR_RIGHT).unwrap().into_output(),
        }
    }

    pub fn rotate_gpio_motor_left(&mut self) {
        self.gpio_motor_right_channel.set_low();
        self.wait_for_short_time();
        self.gpio_motor_left_channel.set_high();
    }

    pub fn rotate_gpio_motor_right(&mut self) {
        self.gpio_motor_left_channel.set_low();
        self.wait_for_short_time();
        self.gpio_motor_right_channel.set_high();
    }

    pub fn stop_gpio_motor(&mut self) {
        self.gpio_motor_left_channel.set_low();
        self.gpio_motor_right_channel.set_low();
    }

    fn wait_for_short_time(&self) {
        //Wait a short time to be shure, that the hardware has processed the changes.
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
