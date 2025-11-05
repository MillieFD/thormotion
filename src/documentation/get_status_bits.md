Request status bits for the specified motor channel.

For an explanation of status bits, see the Thorlabs APT Protocol, Issue 39, Page 126.

An expanded version of the status update message, containing additional position and velocity data,  can be 
requested using [`get_status`][1].

[1]: crate::devices::KDC101::get_status