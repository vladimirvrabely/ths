use std::error::Error;
use tokio_modbus::{client::{rtu::attach_slave, Reader}, slave::Slave};

pub async fn run() -> Result<(), Box<dyn Error>>{
    println!("yep");
    let tty_path = "/dev/ttyUSB0";
    let baud_rate = 9600;
    let builder = tokio_serial::new(tty_path, baud_rate);
    let serial_stream = tokio_serial::SerialStream::open(&builder).unwrap();
    let slave = Slave(1);
    let mut client = attach_slave(serial_stream, slave);
    println!("Created client");

    let ir = client.read_input_registers(1, 1).await.unwrap().unwrap();
    println!("{:?}", ir);

    let ir = client.read_input_registers(1, 2).await.unwrap().unwrap();
    println!("{:?}", ir);

    Ok(())
}