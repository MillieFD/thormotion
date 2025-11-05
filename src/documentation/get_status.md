Request a "one-off" status update for the specified motor channel.

Periodic (10Hz) update messages can be enabled using [`hw_stop_update_messages`][1].

A reduced version of the status update message, only containing the status bits without position and velocity data,
can be requested using [`get_status_bits`][2].

### Returns

- Current position (mm)
- Current velocity (mm/s)
- Status bits

For an explanation of status bits, see the Thorlabs APT Protocol, Issue 39, Page 126.

[1]: crate::devices::KDC101::start_update_messages
[2]: crate::devices::KDC101::get_status_bits