use crate::manager::ManagerProxy;
use crate::manager::ManagerProxyBlocking;
use crate::seat::SeatProxy;
use crate::seat::SeatProxyBlocking;
use futures_lite::future;

#[test]
fn timestamps() {
    let connection = zbus::blocking::Connection::system().unwrap();
    let manager = ManagerProxyBlocking::new(&connection).unwrap();
    let seats = manager.list_seats().unwrap();
    let seat = SeatProxyBlocking::builder(&connection)
        .path(seats[0].path())
        .unwrap()
        .build()
        .unwrap();

    assert!(seat.active_session().is_ok());
}

#[test]
fn properties() {
    let connection = zbus::blocking::Connection::system().unwrap();
    let manager = ManagerProxyBlocking::new(&connection).unwrap();
    let seats = manager.list_seats().unwrap();
    let seat = SeatProxyBlocking::builder(&connection)
        .path(seats[0].path())
        .unwrap()
        .build()
        .unwrap();

    assert!(seat.active_session().is_ok());
    assert!(seat.can_graphical().is_ok());
    assert!(seat.can_TTY().is_ok());
    assert!(seat.id().is_ok());
    assert!(seat.idle_hint().is_ok());
    assert!(seat.idle_since_hint().is_ok());
    assert!(seat.idle_since_hint_monotonic().is_ok());
    assert!(seat.sessions().is_ok());
}

#[test]
fn timestamps_async() {
    future::block_on(async {
        let connection = zbus::Connection::system().await.unwrap();
        let manager = ManagerProxy::new(&connection).await.unwrap();
        let seats = manager.list_seats().await.unwrap();
        let seat = SeatProxy::builder(&connection)
            .path(seats[0].path())
            .unwrap()
            .build()
            .await
            .unwrap();

        assert!(seat.active_session().await.is_ok());
    });
}

#[test]
fn properties_async() {
    future::block_on(async {
        let connection = zbus::Connection::system().await.unwrap();
        let manager = ManagerProxy::new(&connection).await.unwrap();
        let seats = manager.list_seats().await.unwrap();
        let seat = SeatProxy::builder(&connection)
            .path(seats[0].path())
            .unwrap()
            .build()
            .await
            .unwrap();

        assert!(seat.active_session().await.is_ok());
        assert!(seat.can_graphical().await.is_ok());
        assert!(seat.can_TTY().await.is_ok());
        assert!(seat.id().await.is_ok());
        assert!(seat.idle_hint().await.is_ok());
        assert!(seat.idle_since_hint().await.is_ok());
        assert!(seat.idle_since_hint_monotonic().await.is_ok());
        assert!(seat.sessions().await.is_ok());
    });
}
