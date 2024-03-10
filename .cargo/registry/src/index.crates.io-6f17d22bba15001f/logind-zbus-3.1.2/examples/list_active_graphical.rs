use logind_zbus::{
    manager::ManagerProxyBlocking,
    session::{SessionClass, SessionProxyBlocking, SessionType},
};
use zbus::blocking::Connection;

// Type, Class, Active
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::system()?;
    let manager = ManagerProxyBlocking::new(&connection)?;
    let sessions = manager.list_sessions()?;

    for session in sessions {
        let session_proxy = SessionProxyBlocking::builder(&connection)
            .path(session.path())?
            .build()?;

        session_proxy.class().and_then(|class| {
            if class == SessionClass::User {
                session_proxy.type_().and_then(|typ| {
                    match typ {
                        SessionType::X11 | SessionType::Wayland | SessionType::MIR => {
                            session_proxy.active().map(|active| {
                                if active {
                                    println!("Active graphical session found");
                                } else {
                                    println!("Inactive graphical session found");
                                }
                            })?;
                        }
                        SessionType::TTY | SessionType::Unspecified => {}
                    }
                    Ok(())
                })?;
            }
            Ok(())
        })?;
    }
    Ok(())
}
