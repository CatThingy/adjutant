use async_std::sync::{Arc, RwLock};

use crate::notifications::Notifications;

pub struct Print;

pub struct Printer {
    current: Arc<RwLock<Option<usize>>>,
    notifications: Notifications,
}

impl Printer {
    pub fn new(notifications: Notifications, current: Arc<RwLock<Option<usize>>>) -> Self {
        Printer {
            current,
            notifications,
        }
    }
    pub async fn print(&mut self) {
        let notifications = self.notifications.read().await;

        let current = self.current.read().await;

        if let Some(index) = *current {
            drop(current);
            let (_, notification) = &notifications[index];

            println!("app_id|string|{}", notification.app_name);
            println!("summary|string|{}", notification.summary);
            println!("body|string|{}", notification.body);
            println!("index|int|{}", index + 1);
            println!("len|int|{}", notifications.len());
            println!("has|bool|true\n");
        } else {
            drop(current);
            println!("app_id|string|");
            println!("summary|string|");
            println!("body|string|");
            println!("index|int|0");
            println!("len|int|0");
            println!("has|bool|false\n");
        }
    }
}
