// use async_std::future::pending;
mod adjutant;
mod notifications;
mod printer;

use std::{error::Error, time::Duration};

use async_std::sync::{Arc, RwLock};
use printer::Printer;
use zbus::ConnectionBuilder;

use notifications::{Notification, NotificationHandler};

use crate::adjutant::Adjutant;

async fn main_() -> Result<(), Box<dyn Error>> {
    let notifications = Arc::new(RwLock::new(Vec::<(u32, Notification)>::new()));
    let handler = NotificationHandler::new(notifications.clone());
    let mut printer = Printer::new(notifications);
    let _ = ConnectionBuilder::session()?
        .name("org.freedesktop.Notifications")?
        .serve_at("/org/freedesktop/Notifications", handler)?
        .build()
        .await?;

    let adjutant = Adjutant;

    let _ = ConnectionBuilder::session()?
        .name("catthingy.Adjutant")?
        .serve_at("/catthingy/Adjutant", adjutant)?
        .build()
        .await?;

    let mut now = std::time::Instant::now();

    loop {
        printer.update(now).await;
        now = std::time::Instant::now();
        async_std::task::sleep(Duration::from_secs(1)).await;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    async_std::task::block_on(async { main_().await })
}
