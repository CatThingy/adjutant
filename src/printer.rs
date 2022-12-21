use async_std::sync::{Arc, RwLock};
use unicode_segmentation::UnicodeSegmentation;

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

            println!("app_id|string|{}", limit_graphemes(&notification.app_name, 400));
            println!("summary|string|{}", limit_graphemes(&notification.summary, 400));
            println!("body|string|{}", limit_graphemes(&notification.body, 400));
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

fn limit_graphemes(string: &str, length: usize) -> &str {
    match string.grapheme_indices(true).nth(length) {
        None => string,
        Some((idx, _)) => &string[..idx],
    }
}
