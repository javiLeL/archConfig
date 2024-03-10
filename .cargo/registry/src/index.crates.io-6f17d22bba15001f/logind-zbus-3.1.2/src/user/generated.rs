//! # `DBus` interface proxy for: `org.freedesktop.login1.User`

#![allow(non_snake_case)]

use zbus::dbus_proxy;

use crate::{SomePath, TimeStamp};

use super::UserState;

#[dbus_proxy(
    interface = "org.freedesktop.login1.User",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
trait User {
    /// Kill method
    #[inline]
    fn kill(&self, signal_number: i32) -> zbus::Result<()>;

    /// Terminate method
    #[inline]
    fn terminate(&self) -> zbus::Result<()>;

    /// Display property
    #[dbus_proxy(property)]
    #[inline]
    fn display(&self) -> zbus::Result<SomePath>;

    /// GID property
    #[dbus_proxy(property)]
    #[inline]
    fn GID(&self) -> zbus::Result<u32>;

    /// IdleHint property
    #[dbus_proxy(property)]
    #[inline]
    fn idle_hint(&self) -> zbus::Result<bool>;

    /// IdleSinceHint property
    #[dbus_proxy(property)]
    #[inline]
    fn idle_since_hint(&self) -> zbus::Result<TimeStamp>;

    /// IdleSinceHintMonotonic property
    #[dbus_proxy(property)]
    #[inline]
    fn idle_since_hint_monotonic(&self) -> zbus::Result<TimeStamp>;

    /// Linger property
    #[dbus_proxy(property)]
    #[inline]
    fn linger(&self) -> zbus::Result<bool>;

    /// Name property
    #[dbus_proxy(property)]
    #[inline]
    fn name(&self) -> zbus::Result<String>;

    /// RuntimePath property
    #[dbus_proxy(property)]
    #[inline]
    fn runtime_path(&self) -> zbus::Result<String>;

    /// Service property
    #[dbus_proxy(property)]
    #[inline]
    fn service(&self) -> zbus::Result<String>;

    /// Sessions property
    #[dbus_proxy(property)]
    #[inline]
    fn sessions(&self) -> zbus::Result<Vec<(String, zbus::zvariant::OwnedObjectPath)>>;

    /// Slice property
    #[dbus_proxy(property)]
    #[inline]
    fn slice(&self) -> zbus::Result<String>;

    /// State property
    #[dbus_proxy(property)]
    #[inline]
    fn state(&self) -> zbus::Result<UserState>;

    /// Timestamp property
    #[dbus_proxy(property)]
    #[inline]
    fn timestamp(&self) -> zbus::Result<TimeStamp>;

    /// TimestampMonotonic property
    #[dbus_proxy(property)]
    #[inline]
    fn timestamp_monotonic(&self) -> zbus::Result<TimeStamp>;

    /// UID property
    #[dbus_proxy(property)]
    #[inline]
    fn UID(&self) -> zbus::Result<u32>;
}
