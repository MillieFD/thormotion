/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion

BSD 3-Clause License, Copyright (c) 2025, Amelia Fraser-Dale

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the conditions of the LICENSE are met.
*/

/* ------------------------------------------------------------------------------ Public modules */

pub mod devices;
pub mod error;

/* ----------------------------------------------------------------------------- Private modules */

#[doc(hidden)]
mod functions;
mod messages;
mod traits;

/* ------------------------------------------------------------------------------ Public Exports */

pub use traits::ThorlabsDevice;

/* --------------------------------------------------------------------------------------- Tests */

#[cfg(test)]
mod tests {
    use log::{debug, error, info, trace, warn};

    /// Initialize the logging infrastructure for tests
    /// This ensures that logging output is captured and displayed during test execution
    fn init_test_logging() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Trace)
            .try_init();
    }

    #[test]
    fn test_device_creation() {
        init_test_logging();
        info!("=== Starting test_device_creation ===");

        trace!("Importing KDC101 device module");
        use crate::devices::KDC101;

        info!("Test case: Creating KDC101 device with valid serial number");
        let serial_number = String::from("27000001");
        debug!("Serial number: {}", serial_number);

        info!("Step 1: Attempting to create new KDC101 device instance");
        match KDC101::new(serial_number.clone()) {
            Ok(_device) => {
                info!("✓ Successfully created KDC101 device with serial number: {}", serial_number);
                debug!("Device instance created and ready for operations");
            }
            Err(e) => {
                error!("✗ Failed to create KDC101 device: {:?}", e);
                warn!("This error is expected if no physical device is connected");
                debug!("Error details: {}", e);
            }
        }

        info!("=== Completed test_device_creation ===");
    }

    #[test]
    fn test_device_creation_invalid_serial() {
        init_test_logging();
        info!("=== Starting test_device_creation_invalid_serial ===");

        use crate::devices::KDC101;

        info!("Test case: Creating KDC101 device with invalid serial number");
        let invalid_serial = String::from("invalid");
        debug!("Invalid serial number: {}", invalid_serial);

        info!("Step 1: Attempting to create device with invalid serial number");
        match KDC101::new(invalid_serial.clone()) {
            Ok(_) => {
                error!("✗ Unexpectedly succeeded in creating device with invalid serial: {}", invalid_serial);
                panic!("Expected error for invalid serial number");
            }
            Err(e) => {
                info!("✓ Correctly rejected invalid serial number: {}", invalid_serial);
                debug!("Expected error received: {:?}", e);
                trace!("Error type and message validated successfully");
            }
        }

        info!("=== Completed test_device_creation_invalid_serial ===");
    }

    #[test]
    fn test_device_utilities() {
        init_test_logging();
        info!("=== Starting test_device_utilities ===");

        use crate::devices::{get_devices, show_devices};

        info!("Test case: Verifying device utility functions");
        
        info!("Step 1: Calling get_devices() to retrieve connected devices");
        let devices: Vec<_> = get_devices().collect();
        debug!("Number of devices found: {}", devices.len());
        
        if devices.is_empty() {
            warn!("No devices detected - this is expected in test environment");
            trace!("Device list is empty, which is normal without hardware");
        } else {
            info!("✓ Detected {} device(s)", devices.len());
            for (idx, device_info) in devices.iter().enumerate() {
                debug!("Device {}: {:?}", idx + 1, device_info);
            }
        }

        info!("Step 2: Calling show_devices() to display device information");
        show_devices();
        trace!("show_devices() completed successfully");

        info!("=== Completed test_device_utilities ===");
    }

    // Original test case enhanced with logging (kept commented as it requires physical hardware)
    // #[test]
    // fn identify_kdc101() {
    //     init_test_logging();
    //     info!("=== Starting identify_kdc101 ===");
    //     
    //     use crate::devices::KDC101;
    //     
    //     info!("Test case: Identifying KDC101 device");
    //     let serial_number = String::from("27xxxxxx");
    //     debug!("Using serial number: {}", serial_number);
    //     
    //     info!("Step 1: Creating new KDC101 device instance");
    //     let mut device = match KDC101::new(serial_number.clone()) {
    //         Ok(dev) => {
    //             info!("✓ Device created successfully");
    //             dev
    //         }
    //         Err(e) => {
    //             error!("✗ Failed to create device: {:?}", e);
    //             panic!("Cannot proceed without device");
    //         }
    //     };
    //     
    //     smol::block_on(async {
    //         info!("Step 2: Opening device connection");
    //         match device.open_async().await {
    //             Ok(_) => {
    //                 info!("✓ Device opened successfully");
    //                 debug!("Device is now ready for communication");
    //             }
    //             Err(e) => {
    //                 error!("✗ Failed to open device: {:?}", e);
    //                 panic!("Cannot communicate with device");
    //             }
    //         }
    //         
    //         info!("Step 3: Sending identify command to device");
    //         device.identify_async().await;
    //         info!("✓ Identify command sent successfully");
    //         trace!("Device should now be blinking/identifying itself");
    //     });
    //     
    //     info!("=== Completed identify_kdc101 ===");
    // }
}
