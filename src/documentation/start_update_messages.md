Starts periodic update messages from the device every 100 milliseconds (10 Hz).

Automatic updates will continue until the [`hw_stop_update_messages`][1] function is called.

A 'one-off' status update can be requested using [`get_status`][2].

[1]: crate::devices::KDC101::hw_stop_update_messages
[2]: crate::devices::KDC101::get_status