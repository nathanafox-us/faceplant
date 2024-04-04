use esp_idf_svc::hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_sys::{EspError, ESP_OK};

//SENSOR I2C ADDR
const SENSOR_ADDR:u8 = 0x68;

pub struct StemmaSoilSensor {
    capacitance:f64,
    temperature:f64,
}
impl StemmaSoilSensor {
    pub fn new() -> Self {
        Self {capacitance:0f64, temperature:0f64}
    }

    fn read_sensor(&self, i2c:&I2cDriver) -> i32 {
        self.get_capacitance(i2c);
        self.get_temperature(i2c);
        return ESP_OK;
    }
    fn get_capacitance(&self, i2c:&I2cDriver)->i32 {
        return ESP_OK;
    }
    fn get_temperature(&self, i2c:&I2cDriver)->i32 {
        return ESP_OK;
    }
}

fn main() -> Result<(), EspError> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    // 1. Instanciate the SDA and SCL pins, correct pins are in the training material.
    let sda = peripherals.pins.gpio11;
    let scl = peripherals.pins.gpio12;

    // 2. Instanciate the i2c peripheral
    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config)?;

    let mut index = 0;

    let soil_sensor = StemmaSoilSensor::new();

    // THIS PRINTS 6 times in one minute
    loop {
        soil_sensor.read_sensor(&i2c);

        println!("CAPACITANCE: {:?}", soil_sensor.capacitance);
        println!("TEMP: {:?}", soil_sensor.temperature);

        index += 1;
        if index > 6 {
            break;
        }
        FreeRtos::delay_ms(10000u32);
    };

    return Ok(());
}