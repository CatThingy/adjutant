mod adjutant;
mod notifications;
mod printer;

use std::{convert::TryInto, error::Error};

use async_std::{
    channel,
    sync::{Arc, RwLock},
};
use zbus::{ConnectionBuilder, SignalContext};

use adjutant::Adjutant;
use notifications::{NotificationHandler, Notifications};
use printer::{Print, Printer};

async fn main_() -> Result<(), Box<dyn Error>> {
    let notifications = Notifications::default();
    let current: Arc<RwLock<Option<usize>>> = Default::default();
    let (tx, rx) = channel::unbounded::<Print>();

    let handler = NotificationHandler::new(notifications.clone(), tx.clone(), current.clone());
    let notif_conn = ConnectionBuilder::session()?
        .name("org.freedesktop.Notifications")?
        .serve_at("/org/freedesktop/Notifications", handler)?
        .build()
        .await?;

    let mut printer = Printer::new(notifications.clone(), current.clone());

    let adjutant = Adjutant::new(
        notifications.clone(),
        current.clone(),
        tx.clone(),
        SignalContext::from_parts(
            notif_conn,
            "/org/freedesktop/Notifications".try_into().unwrap(),
        ),
    );

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
