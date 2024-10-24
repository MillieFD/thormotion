use crate::error::Error;
use pyo3::prelude::*;

mod enumerate;
mod error;

#[pymodule]
fn thormotion(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_test, m)?)?;
    Ok(())
}

#[pyfunction]
fn py_test(serial_number: u32) -> PyResult<()> {
    let serial_string = serial_number.to_string();
    let mut serial = enumerate::open_serial_port(serial_string)?;
    let chan_enable: [u8; 6] = [0x10, 0x02, 0x01, 0x01, 0x50, 0x01];
    let home: [u8; 6] = [0x43, 0x04, 0x01, 0x00, 0x50, 0x01];
    serial.flush()?;
    serial.write_all(&chan_enable)?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    serial.write_all(&home)?;
    Ok(())
}

fn test_send(serial_number: u32) -> Result<(), Error> {
    let serial_string = serial_number.to_string();
    let mut serial = enumerate::open_serial_port(serial_string)?;
    let chan_enable: [u8; 6] = [0x10, 0x02, 0x01, 0x01, 0x50, 0x01];
    let home: [u8; 6] = [0x43, 0x04, 0x01, 0x00, 0x50, 0x01];
    serial.flush()?;
    serial.write_all(&chan_enable)?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    serial.write_all(&home)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_send;

    #[test]
    fn it_works() {
        if let Err(err) = test_send(27264344) {
            eprintln!("Error: \n{}", err.to_string());
        }
    }
}
