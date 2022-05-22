use async_std::future::pending;
use std::error::Error;
use zbus::ConnectionBuilder;

mod notifications;
use notifications::Notifications;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let notifications = Notifications::new();
    let _ = ConnectionBuilder::session()?
        .name("org.freedesktop.Notifications")?
        .serve_at("/org/freedesktop/Notifications", notifications)?
        .build()
        .await?;

    // // Do other things or wait forever
    pending::<()>().await;

    Ok(())
}
