use async_std::sync::{Arc, RwLock};

use crate::notifications::Notifications;

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

            println!("app_id|string|{}", notification.app_name,);
            println!("summary|string|{}", notification.summary,);
            println!("index|int|{}", index + 1,);
            println!("len|int|{}\n", notifications.len());
            println!("has|bool|true");
        } else {
            drop(current);
            println!(
                "app_id|string|{}\nsummary|string|{}\nindex|int|{}\nlen|int|{}\nhas|bool|false\n\n",
                "", "", 0, 0
            )
        }
    }
}
