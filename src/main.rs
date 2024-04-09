use esp_idf_svc::hal::{
    delay::{FreeRtos, BLOCK},
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};
use esp_idf_sys::{EspError, ESP_OK};

//SENSOR I2C ADDR
const SENSOR_ADDR:u8 = 0x36;
const SOIL_ADDR_1:u8 = 0x0F;
const SOIL_ADDR_2:u8 = 0x10;
const TEMP_ADDR_1:u8 = 0x00;
const TEMP_ADDR_2:u8 = 0x04;


pub struct StemmaSoilSensor {
    capacitance:u16,
    temperature:u16,
}
impl StemmaSoilSensor {
    pub fn new() -> Self {
        Self {capacitance: 0, temperature: 0}
    }

    fn read_sensor(&mut self, i2c:&mut I2cDriver) -> i32 {
        let _ = self.get_capacitance(i2c);
        let _ = self.get_temperature(i2c);
        return ESP_OK;
    }
    fn get_capacitance(&mut self, i2c:&mut I2cDriver)-> Result<i32, EspError> {
        let location_bytes = [SOIL_ADDR_1, SOIL_ADDR_2];
        let mut capacitance_bytes: [u8; 2] = [0, 0];

        i2c.write(SENSOR_ADDR, &location_bytes, BLOCK)?;

        FreeRtos::delay_ms(500);

        let read_err = i2c.read(SENSOR_ADDR, &mut capacitance_bytes, BLOCK);
        match read_err {
            Ok(()) => {
                self.capacitance = (capacitance_bytes[0] as u16) << 8 | capacitance_bytes[1] as u16;
                return Ok(ESP_OK);
            },
            Err(esp_err) => {
                println!("{}", esp_err);
                return Err(esp_err);
            }
        }
    }
    fn get_temperature(&mut self, i2c:&mut I2cDriver)->Result<i32, EspError>  {
        let location_bytes = [TEMP_ADDR_1, TEMP_ADDR_2];
        let mut temp_bytes: [u8; 2] = [0, 0];

        i2c.write(SENSOR_ADDR, &location_bytes, BLOCK)?;

        FreeRtos::delay_ms(500);

        let read_error: Result<(), EspError> = i2c.read(SENSOR_ADDR, &mut temp_bytes, BLOCK);

        match read_error {
            Ok(()) => {
                self.temperature = (temp_bytes[0] as u16) << 8 | temp_bytes[1] as u16;
                return Ok(ESP_OK);
            },
            Err(esp_err) => {
                println!("{}", esp_err);
                return Err(esp_err);
            }
        }
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
    let mut i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config)?;

    let mut index = 0;

    let mut soil_sensor = StemmaSoilSensor::new();

    // THIS PRINTS once every second and a half (half a second in between reads)
    loop {
        soil_sensor.read_sensor(&mut i2c);

        println!("CAPACITANCE: {:?}", soil_sensor.capacitance);
        println!("TEMP: {:?}", soil_sensor.temperature);

        index += 1;
        if index >= 60 {
            break;
        }
        FreeRtos::delay_ms(1000u32);
    };

    return Ok(());
}