mod adjutant;
mod notifications;
mod printer;

use std::error::Error;

use async_std::{
    channel,
    sync::{Arc, RwLock},
};
use zbus::ConnectionBuilder;

use adjutant::Adjutant;
use notifications::{NotificationHandler, Notifications};
use printer::Printer;

async fn main_() -> Result<(), Box<dyn Error>> {
    let notifications = Notifications::default();
    let current: Arc<RwLock<Option<u32>>> = Default::default();
    let (tx, rx) = channel::unbounded::<()>();

    let handler = NotificationHandler::new(notifications.clone(), tx.clone());
    let adjutant = Adjutant::new(notifications.clone(), current.clone(), tx.clone());

    let mut printer = Printer::new(notifications.clone(), current.clone());

    let _notif_conn = ConnectionBuilder::session()?
        .name("org.freedesktop.Notifications")?
        .serve_at("/org/freedesktop/Notifications", handler)?
        .build()
        .await?;

    let _adj_conn = ConnectionBuilder::session()?
        .name("catthingy.Adjutant")?
        .serve_at("/catthingy/Adjutant", adjutant)?
        .build()
        .await?;

    loop {
        rx.recv().await?;
        printer.print().await;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    async_std::task::block_on(async { main_().await })
}
