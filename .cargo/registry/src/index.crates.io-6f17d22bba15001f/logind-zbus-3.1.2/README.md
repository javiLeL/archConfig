# logind-zbus

A wrapper around the dbus interfaces provided by `systemd-logind`.

`logind-zbus` aims to provide a convenient API abstraction of the dbus interface
of logind in rust, where possible parsing responses to concrete structs and enums.
