use crate::manager::{InhibitType, IsSupported, ManagerProxy, ManagerProxyBlocking, Mode};
use futures_lite::future;

#[test]
fn timestamps() {
    let connection = zbus::blocking::Connection::system().unwrap();
    let manager = ManagerProxyBlocking::new(&connection).unwrap();

    assert!(manager.can_suspend().is_ok());
}

#[test]
fn properties() {
    let connection = zbus::blocking::Connection::system().unwrap();
    let manager = ManagerProxyBlocking::new(&connection).unwrap();

    assert!(manager.block_inhibited().is_ok());
    assert!(manager.boot_loader_entries().is_ok());
    assert!(manager.delay_inhibited().is_ok());
    assert!(manager.docked().is_ok());
    assert!(manager.handle_hibernate_key().is_ok());
    assert!(manager.handle_lid_switch().is_ok());
    assert!(manager.handle_lid_switch_docked().is_ok());
    assert!(manager.handle_lid_switch_external_power().is_ok());
    assert!(manager.handle_power_key().is_ok());
    assert!(manager.handle_suspend_key().is_ok());
    assert!(manager.holdoff_timeout_USec().is_ok());
    assert!(manager.idle_action().is_ok());
    assert!(manager.idle_action_USec().is_ok());
    assert!(manager.idle_hint().is_ok());
    assert!(manager.idle_since_hint().is_ok());
    assert!(manager.idle_since_hint_monotonic().is_ok());
    assert!(manager.inhibit_delay_max_USec().is_ok());
    assert!(manager.inhibitors_max().is_ok());
    assert!(manager.kill_exclude_users().is_ok());
    assert!(manager.kill_only_users().is_ok());
    assert!(manager.kill_user_processes().is_ok());
    assert!(manager.lid_closed().is_ok());
    assert!(manager.NAuto_VTs().is_ok());
    assert!(manager.NCurrent_inhibitors().is_ok());
    assert!(manager.NCurrent_sessions().is_ok());
    assert!(manager.on_external_power().is_ok());
    assert!(manager.preparing_for_shutdown().is_ok());
    assert!(manager.preparing_for_sleep().is_ok());
    assert!(manager.reboot_parameter().is_ok());
    assert!(manager.reboot_to_boot_loader_entry().is_ok());
    assert!(manager.reboot_to_boot_loader_menu().is_ok());
    assert!(manager.reboot_to_firmware_setup().is_ok());
    assert!(manager.remove_IPC().is_ok());
    assert!(manager.runtime_directory_inodes_max().is_ok());
    assert!(manager.runtime_directory_size().is_ok());
    assert!(manager.scheduled_shutdown().is_ok());
    assert!(manager.get_seat("seat0").is_ok());

    // Requires a valid session ID
    // assert!(manager.get_session("c4").is_ok());
    // Requires knowing a process ID owned by a session
    // assert!(manager.get_session_by_pid(5455).is_ok());

    assert!(manager.sessions_max().is_ok());
    // Requires a valid logged-in user ID
    // assert!(manager.get_user(1000).is_ok());
    // Requires knowing a process ID
    // assert!(manager.get_user_by_pid(25813).is_ok());
    assert!(manager.user_stop_delay_USec().is_ok());
    assert!(manager.wall_message().is_ok());
}

#[test]
fn timestamps_async() {
    future::block_on(async {
        let connection = zbus::Connection::system().await.unwrap();
        let manager = ManagerProxy::new(&connection).await.unwrap();

        assert!(manager.can_suspend().await.is_ok());
    });
}

#[test]
fn properties_async() {
    future::block_on(async {
        let connection = zbus::Connection::system().await.unwrap();
        let manager = ManagerProxy::new(&connection).await.unwrap();

        assert!(manager.block_inhibited().await.is_ok());
        assert!(manager.boot_loader_entries().await.is_ok());
        assert!(manager.delay_inhibited().await.is_ok());
        assert!(manager.docked().await.is_ok());
        assert!(manager.handle_hibernate_key().await.is_ok());
        assert!(manager.handle_lid_switch().await.is_ok());
        assert!(manager.handle_lid_switch_docked().await.is_ok());
        assert!(manager.handle_lid_switch_external_power().await.is_ok());
        assert!(manager.handle_power_key().await.is_ok());
        assert!(manager.handle_suspend_key().await.is_ok());
        assert!(manager.holdoff_timeout_USec().await.is_ok());
        assert!(manager.idle_action().await.is_ok());
        assert!(manager.idle_action_USec().await.is_ok());
        assert!(manager.idle_hint().await.is_ok());
        assert!(manager.idle_since_hint().await.is_ok());
        assert!(manager.idle_since_hint_monotonic().await.is_ok());
        assert!(manager.inhibit_delay_max_USec().await.is_ok());
        assert!(manager.inhibitors_max().await.is_ok());
        assert!(manager.kill_exclude_users().await.is_ok());
        assert!(manager.kill_only_users().await.is_ok());
        assert!(manager.kill_user_processes().await.is_ok());
        assert!(manager.lid_closed().await.is_ok());
        assert!(manager.NAuto_VTs().await.is_ok());
        assert!(manager.NCurrent_inhibitors().await.is_ok());
        assert!(manager.NCurrent_sessions().await.is_ok());
        assert!(manager.on_external_power().await.is_ok());
        assert!(manager.preparing_for_shutdown().await.is_ok());
        assert!(manager.preparing_for_sleep().await.is_ok());
        assert!(manager.reboot_parameter().await.is_ok());
        assert!(manager.reboot_to_boot_loader_entry().await.is_ok());
        assert!(manager.reboot_to_boot_loader_menu().await.is_ok());
        assert!(manager.reboot_to_firmware_setup().await.is_ok());
        assert!(manager.remove_IPC().await.is_ok());
        assert!(manager.runtime_directory_inodes_max().await.is_ok());
        assert!(manager.runtime_directory_size().await.is_ok());
        assert!(manager.scheduled_shutdown().await.is_ok());
        assert!(manager.get_seat("seat0").await.is_ok());

        // Requires a valid session ID
        // assert!(manager.get_session("c4").await.is_ok());
        // Requires knowing a process ID owned by a session
        // assert!(manager.get_session_by_pid(5455).await.is_ok());

        assert!(manager.sessions_max().await.is_ok());
        // Requires a valid logged-in user ID
        // assert!(manager.get_user(1000).await.is_ok());
        // Requires knowing a process ID
        // assert!(manager.get_user_by_pid(25813).await.is_ok());
        assert!(manager.user_stop_delay_USec().await.is_ok());
        assert!(manager.wall_message().await.is_ok());
    });
}

#[test]
fn inhibitors() {
    let connection = zbus::blocking::Connection::system().unwrap();
    let manager = ManagerProxyBlocking::new(&connection).unwrap();

    assert!(manager.can_suspend().is_ok());

    let inhibitors = manager.list_inhibitors();
    let n_inhibitors = manager.NCurrent_inhibitors().unwrap();
    assert!(inhibitors.is_ok());
    assert_eq!(n_inhibitors, inhibitors.unwrap().len() as u64);

    let res = manager.inhibit(
        InhibitType::HandleHibernateKey,
        "inhibit test",
        "inhibit test",
        <&str>::from(Mode::Delay),
    );

    assert!(res.is_err());
    res.map_err(|e| {
        if let zbus::Error::MethodError(_, data, _) = e {
            assert_eq!(
                data,
                Some("Delay inhibitors only supported for shutdown and sleep".to_owned())
            );
        }
    })
    .ok();
}

#[test]
fn can_do() {
    let connection = zbus::blocking::Connection::system().unwrap();
    let manager = ManagerProxyBlocking::new(&connection).unwrap();

    let res = manager.can_suspend();
    assert!(res.is_ok());
    assert_eq!(res, zbus::Result::Ok(IsSupported::Yes));

    let res = manager.can_hybrid_sleep();
    assert!(res.is_ok());
    assert_eq!(res, zbus::Result::Ok(IsSupported::Yes));

    let res = manager.can_hibernate();
    assert!(res.is_ok());
    assert_eq!(res, zbus::Result::Ok(IsSupported::Yes));
}
