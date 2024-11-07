/*
Project: thormotion
GitHub: https://github.com/MillieFD/thormotion
Author: Amelia Fraser-Dale
License: BSD 3-Clause "New" or "Revised"
Filename: todo
Description: todo
---------------------------------------------------------------------------------------------------
Notes:
*/

use crate::devices::KDC101::KDC101;
use crate::enumeration::usb_device_primitive::UsbDevicePrimitive;
use crate::env::{TIMEOUT, VENDOR_ID};
use crate::errors::error_types::Error;
use crate::traits::thorlabs_device::ThorlabsDevice;
use rusb::{Device, DeviceDescriptor, DeviceHandle, DeviceList, GlobalContext, Language};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};

static ALL_DEVICES: LazyLock<RwLock<HashMap<String, Arc<Box<dyn ThorlabsDevice>>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

async fn get_devices() -> Result<(), Error> {
    for device in DeviceList::new()?.iter().filter_map(|d| filter_devices(d)) {
        let dev: Arc<Box<dyn ThorlabsDevice>> = match &device.serial_number[..2] {
            // "20" => Arc::new(Box::new(BSC001::new(device).await?)),
            // "21" => Arc::new(Box::new(BPC001::new(device).await?)),
            // "22" => Arc::new(Box::new(BNT001::new(device).await?)),
            // "25" => Arc::new(Box::new(BMS001::new(device).await?)),
            // "26" => Arc::new(Box::new(KST101::new(device).await?)),
            "27" => Arc::new(Box::new(KDC101::new(device).await?)),
            // "28" => Arc::new(Box::new(KBD101::new(device).await?)),
            // "29" => Arc::new(Box::new(KPZ101::new(device).await?)),
            // "30" => Arc::new(Box::new(BSC002::new(device).await?)),
            // "31" => Arc::new(Box::new(BPC002::new(device).await?)),
            // "33" => Arc::new(Box::new(BDC101::new(device).await?)),
            // "35" => Arc::new(Box::new(BMS002::new(device).await?)),
            // "37" => Arc::new(Box::new(MFF10X::new(device).await?)),
            // "40" => Arc::new(Box::new(BSC101::new(device).await?)),
            // "41" => Arc::new(Box::new(BPC101::new(device).await?)),
            // "43" => Arc::new(Box::new(BDC101::new(device).await?)),
            // "44" => Arc::new(Box::new(PPC001::new(device).await?)),
            // "45" => Arc::new(Box::new(LTS150/LTS300::new(device).await?)),
            // "49" => Arc::new(Box::new(MLJ050/MLJ150::new(device).await?)),
            // "50" => Arc::new(Box::new(MST601/MST602::new(device).await?)),
            // "51" => Arc::new(Box::new(MPZ601::new(device).await?)),
            // "52" => Arc::new(Box::new(MNA601/IR::new(device).await?)),
            // "55" => Arc::new(Box::new(K10CR1::new(device).await?)),
            // "56" => Arc::new(Box::new(KLS101::new(device).await?)),
            // "57" => Arc::new(Box::new(KNA101::new(device).await?)),
            // "59" => Arc::new(Box::new(KSG101::new(device).await?)),
            // "60" => Arc::new(Box::new(OST001::new(device).await?)),
            // "63" => Arc::new(Box::new(ODC001::new(device).await?)),
            // "64" => Arc::new(Box::new(TLD001::new(device).await?)),
            // "65" => Arc::new(Box::new(TIM001::new(device).await?)),
            // "67" => Arc::new(Box::new(TBD001::new(device).await?)),
            // "68" => Arc::new(Box::new(KSC101::new(device).await?)),
            // "69" => Arc::new(Box::new(KPA101::new(device).await?)),
            // "70" => Arc::new(Box::new(BSC103/BSC203::new(device).await?)),
            // "71" => Arc::new(Box::new(BPC103/203/303::new(device).await?)),
            // "72" => Arc::new(Box::new(BPS103::new(device).await?)),
            // "73" => Arc::new(Box::new(BBD103::new(device).await?)),
            // "80" => Arc::new(Box::new(TST001::new(device).await?)),
            // "81" => Arc::new(Box::new(TPZ001::new(device).await?)),
            // "82" => Arc::new(Box::new(TNA001::new(device).await?)),
            // "83" => Arc::new(Box::new(TDC001::new(device).await?)),
            // "84" => Arc::new(Box::new(TSG001::new(device).await?)),
            // "85" => Arc::new(Box::new(TSC001::new(device).await?)),
            // "86" => Arc::new(Box::new(TLS001::new(device).await?)),
            // "87" => Arc::new(Box::new(TTC001::new(device).await?)),
            // "89" => Arc::new(Box::new(TQD001::new(device).await?)),
            // "90" => Arc::new(Box::new(SCC101::new(device).await?)),
            // "91" => Arc::new(Box::new(PCC101::new(device).await?)),
            // "93" => Arc::new(Box::new(DCC101::new(device).await?)),
            // "94" => Arc::new(Box::new(BCC101::new(device).await?)),
            // "95" => Arc::new(Box::new(PPC102::new(device).await?)),
            // "96" => Arc::new(Box::new(PCC102::new(device).await?)),
            _ => return Err(Error::DeviceNotSupported(device.serial_number)),
        };
        ALL_DEVICES.write()?.insert(dev.serial_number.clone(), dev);
    }
    Ok(())
}

fn filter_devices(dev: Device<GlobalContext>) -> Option<UsbDevicePrimitive> {
    let descriptor = dev.device_descriptor().ok()?;
    if descriptor.vendor_id() != VENDOR_ID {
        return None;
    }
    let handle = dev.open().ok()?;
    let language = get_language(&handle)?;
    let serial_number = get_serial_number(&descriptor, &handle, language)?;
    let device = UsbDevicePrimitive::new(handle, descriptor, language, serial_number).ok()?;
    Some(device)
}

fn get_language(handle: &DeviceHandle<GlobalContext>) -> Option<Language> {
    handle.read_languages(TIMEOUT).ok()?.get(0).copied()
}

fn get_serial_number(
    descriptor: &DeviceDescriptor,
    handle: &DeviceHandle<GlobalContext>,
    language: Language,
) -> Option<String> {
    Some(
        handle
            .read_serial_number_string(language, &descriptor, TIMEOUT)
            .ok()?,
    )
}

pub(crate) async fn get_device(serial_number: &str) -> Result<Arc<Box<dyn ThorlabsDevice>>, Error> {
    if let Some(device) = ALL_DEVICES.read()?.get(serial_number) {
        return Ok(device.clone());
    }
    get_devices().await?;
    Ok(ALL_DEVICES
        .read()?
        .get(serial_number)
        .ok_or(Error::DeviceNotFound(serial_number.to_string()))?
        .clone())
}
