Releases the claimed USB [`Interface`][1].

No action is taken if the device [`Status`][2] is already `Closed`.

This does not stop the device's current action. If you need to safely bring the device to a resting state, 
see [`abort`][3].

[1]: nusb::Interface
[2]: crate::devices::usb_primitive::status::Status
[3]: crate::ThorlabsDevice::abort