use std::time::Instant;

use async_std::sync::{Arc, RwLock, RwLockUpgradableReadGuard};

use crate::notifications::Notification;

pub struct Printer {
    current: Option<usize>,
    notifications: Arc<RwLock<Vec<(u32, Notification)>>>,
}

impl Printer {
    pub fn new(notifications: Arc<RwLock<Vec<(u32, Notification)>>>) -> Self {
        Printer {
            current: None,
            notifications,
        }
    }
    pub async fn update(&mut self, now: Instant) {
        let notifications = self.notifications.upgradable_read().await;
        if notifications.len() > 0 {
            if let Some(index) = self.current {
                self.current = Some((index + 1) % notifications.len());
            } else {
                self.current = Some(0);
            }
        } else {
            self.current = None;
        }

        if let Some(index) = self.current {
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
    }
}
