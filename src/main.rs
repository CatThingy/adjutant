// use async_std::future::pending;
use std::{error::Error, time::Duration};

use async_std::sync::{Arc, RwLock, RwLockUpgradableReadGuard};
use zbus::ConnectionBuilder;

mod notifications;
use notifications::{Notification, NotificationHandler};

pub struct Notitext {
    current: Option<usize>,
    notifications: Arc<RwLock<Vec<(u32, Notification)>>>,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let handler = NotificationHandler::new();
    let mut printer = Notitext {
        current: None,
        notifications: handler.notifications.clone(),
    };

    let _ = ConnectionBuilder::session()?
        .name("org.freedesktop.Notifications")?
        .serve_at("/org/freedesktop/Notifications", handler)?
        .build()
        .await?;

    // pending::<()>().await;

    loop {
        let notifications = printer.notifications.upgradable_read().await;
        if printer.current.is_none() && notifications.len() > 0 {
            printer.current = Some(0);
        } else if let Some(index) = printer.current {
            printer.current = Some((index + 1) % notifications.len());
        }

        if let Some(index) = printer.current {
            let (_, notification) = &notifications[index];

            println!(
                "app_id|string|{}\nsummary|string|{}\n\n",
                notification.app_name, notification.summary
            );
        }


        let notifications = RwLockUpgradableReadGuard::upgrade(notifications);

        drop(notifications);
        async_std::task::sleep(Duration::from_secs(1)).await;
    }
}
