use logind_zbus::{manager::ManagerProxyBlocking, session::SessionProxyBlocking};
use zbus::blocking::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::system()?;
    let manager = ManagerProxyBlocking::new(&connection)?;
    let sessions = manager.list_sessions()?;

    let mut seat = 0;

    for (i, s) in sessions.iter().enumerate() {
        if s.uid() >= 1000 {
            seat = i;
            break;
        }
    }

    let session = SessionProxyBlocking::builder(&connection)
        .path(sessions[seat].path())?
        .build()?;

    if let Ok(mut sig_iter) = session.receive_unlock() {
        if sig_iter.next().is_some() {
            println!("Unlocked");
        }
    }
    Ok(())
}
