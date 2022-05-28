// use async_std::future::pending;
use std::{error::Error, time::Duration};

use async_std::sync::{Arc, RwLock, RwLockUpgradableReadGuard};
use zbus::ConnectionBuilder;

mod notifications;
use notifications::{Notification, NotificationHandler};

pub struct Printer {
    current: Option<usize>,
    notifications: Arc<RwLock<Vec<(u32, Notification)>>>,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let handler = NotificationHandler::new();
    let mut printer = Printer {
        current: None,
        notifications: handler.notifications.clone(),
    };

    let _ = ConnectionBuilder::session()?
        .name("org.freedesktop.Notifications")?
        .serve_at("/org/freedesktop/Notifications", handler)?
        .build()
        .await?;

    let mut now = std::time::Instant::now();

    loop {
        let notifications = printer.notifications.upgradable_read().await;
        if notifications.len() > 0 {
            if let Some(index) = printer.current {
                printer.current = Some((index + 1) % notifications.len());
            } else {
                printer.current = Some(0);
            }
        } else {
            printer.current = None;
        }

        if let Some(index) = printer.current {
            let (_, notification) = &notifications[index];

            println!(
                "app_id|string|{}\nsummary|string|{}\nindex|int|{}\nlen|int|{}\nhas|bool|true\n\n",
                notification.app_name,
                notification.summary,
                index + 1,
                notifications.len()
            );
        } else {
            println!(
                "app_id|string|{}\nsummary|string|{}\nindex|int|{}\nlen|int|{}\nhas|bool|false\n\n",
                "", "", 0, 0
            )
        }

        let mut notifications = RwLockUpgradableReadGuard::upgrade(notifications).await;
        let elapsed = now.elapsed().as_millis() as u32;

        let mut removal_indices = vec![];

        for (index, (_, n)) in notifications.iter_mut().enumerate().rev() {
            if let Some(timeout) = n.expire_timeout {
                n.timer += elapsed;

                if n.timer > timeout {
                    removal_indices.push(index);
                }
            }
        }

        for index in removal_indices.drain(..) {
            notifications.remove(index);
        }

        drop(notifications);
        now = std::time::Instant::now();
        async_std::task::sleep(Duration::from_secs(1)).await;
    }
}
