Releases the claimed USB [`Interface`][1].

No action is taken if the device `Status` is already `Closed`.

This does not stop the device's current action. If you need to safely bring the device to a resting state, 
see [`abort`][2].

[1]: nusb::Interface
[2]: ThorlabsDevice::abort