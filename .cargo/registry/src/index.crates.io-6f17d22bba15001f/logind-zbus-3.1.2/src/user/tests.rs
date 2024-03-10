use crate::manager::ManagerProxy;
use crate::manager::ManagerProxyBlocking;
use crate::user::UserProxy;
use crate::user::UserProxyBlocking;
use futures_lite::future;

#[test]
fn timestamps() {
    let connection = zbus::blocking::Connection::system().unwrap();
    let manager = ManagerProxyBlocking::new(&connection).unwrap();
    let users = manager.list_users().unwrap();
    let user = UserProxyBlocking::builder(&connection)
        .path(users[0].path())
        .unwrap()
        .build()
        .unwrap();

    let time1 = user.timestamp().unwrap();
    assert!(time1.as_secs() > 0);

    let time2 = user.timestamp_monotonic().unwrap();
    assert!(time2.as_secs() > 0);
}

#[test]
fn properties() {
    let connection = zbus::blocking::Connection::system().unwrap();
    let manager = ManagerProxyBlocking::new(&connection).unwrap();
    let users = manager.list_users().unwrap();
    let user = UserProxyBlocking::builder(&connection)
        .path(users[0].path())
        .unwrap()
        .build()
        .unwrap();

    assert!(user.display().is_ok());
    // Special case. Exists only on users
    assert!(user.GID().is_ok());
    assert!(user.idle_hint().is_ok());
    assert!(user.idle_since_hint().is_ok());
    assert!(user.idle_since_hint_monotonic().is_ok());
    assert!(user.linger().is_ok());
    assert!(user.name().is_ok());
    assert!(user.runtime_path().is_ok());
    assert!(user.service().is_ok());
    assert!(user.slice().is_ok());
    assert!(user.sessions().is_ok());
    assert!(user.state().is_ok());
    assert!(user.timestamp().is_ok());
    assert!(user.timestamp_monotonic().is_ok());
    // Special case. Exists only on users
    assert!(user.UID().is_ok());
}

#[test]
fn timestamps_async() {
    future::block_on(async {
        let connection = zbus::Connection::system().await.unwrap();
        let manager = ManagerProxy::new(&connection).await.unwrap();
        let users = manager.list_users().await.unwrap();
        let user = UserProxy::builder(&connection)
            .path(users[0].path())
            .unwrap()
            .build()
            .await
            .unwrap();

        let time1 = user.timestamp().await.unwrap();
        assert!(time1.as_secs() > 0);

        let time2 = user.timestamp_monotonic().await.unwrap();
        assert!(time2.as_secs() > 0);
    });
}

#[test]
fn properties_async() {
    future::block_on(async {
        let connection = zbus::Connection::system().await.unwrap();
        let manager = ManagerProxy::new(&connection).await.unwrap();
        let users = manager.list_users().await.unwrap();
        let user = UserProxy::builder(&connection)
            .path(users[0].path())
            .unwrap()
            .build()
            .await
            .unwrap();

        assert!(user.display().await.is_ok());
        // Special case. Exists only on users
        assert!(user.GID().await.is_ok());
        assert!(user.idle_hint().await.is_ok());
        assert!(user.idle_since_hint().await.is_ok());
        assert!(user.idle_since_hint_monotonic().await.is_ok());
        assert!(user.linger().await.is_ok());
        assert!(user.name().await.is_ok());
        assert!(user.runtime_path().await.is_ok());
        assert!(user.service().await.is_ok());
        assert!(user.slice().await.is_ok());
        assert!(user.sessions().await.is_ok());
        assert!(user.state().await.is_ok());
        assert!(user.timestamp().await.is_ok());
        assert!(user.timestamp_monotonic().await.is_ok());
        // Special case. Exists only on users
        assert!(user.UID().await.is_ok());
    });
}
