use std::collections::HashMap;

use async_std::sync::{Arc, RwLock};
use zbus::dbus_interface;

#[derive(Debug, Clone)]
pub struct Notification {
    pub app_name: String,
    pub summary: String,
    pub expire_timeout: i32,
    pub timer: u32,
}

#[derive(Debug)]
pub struct NotificationHandler {
    pub notifications: Arc<RwLock<Vec<(u32, Notification)>>>,
    next_id: u32,
}

impl NotificationHandler {
    pub fn new() -> NotificationHandler {
        NotificationHandler {
            notifications: Arc::new(RwLock::<Vec<(u32, Notification)>>::new(vec![])),
            next_id: 1,
        }
    }
}

#[dbus_interface(interface = "org.freedesktop.Notifications")]
impl NotificationHandler {
    /// CloseNotification method
    async fn close_notification(&mut self, id: u32) {
        let mut notifications = self.notifications.write().await;
        if let Some(index) = notifications
            .iter()
            .position(|(notif_id, _)| notif_id == &id)
        {
            notifications.remove(index);
        }
    }

    /// GetCapabilities method
    async fn get_capabilities(&self) -> Vec<String> {
        vec![]
    }

    /// GetServerInformation method
    async fn get_server_information(&self) -> (&str, &str, &str, &str) {
        ("notext", "CatThingy", "0.1.0", "1.2")
    }

    /// Notify method
    async fn notify(
        &mut self,
        app_name: &str,
        replaces_id: u32,
        _app_icon: &str,
        summary: &str,
        _body: &str,
        _actions: Vec<&str>,
        _hints: HashMap<&str, zbus::zvariant::Value<'_>>,
        expire_timeout: i32,
    ) -> u32 {
        dbg!(
            app_name,
            replaces_id,
            _app_icon,
            summary,
            _body,
            _actions,
            _hints,
            expire_timeout
        );

        let new_id = if replaces_id == 0 {
            let next_id = self.next_id;
            self.next_id = u32::wrapping_add(next_id, 1);

            next_id
        } else {
            replaces_id
        };

        let mut notifications = self.notifications.write().await;

        if let Some(index) = notifications
            .iter()
            .position(|(notif_id, _)| notif_id == &new_id)
        {
            notifications[index] = (
                new_id,
                Notification {
                    app_name: app_name.to_string(),
                    summary: summary.to_string(),
                    expire_timeout,
                    timer: 0,
                },
            );
        } else {
            notifications.push((
                new_id,
                Notification {
                    app_name: app_name.to_string(),
                    summary: summary.to_string(),
                    expire_timeout,
                    timer: 0,
                },
            ));
        }

        // dbg!(&self.notifications);

        new_id
    }

    // /// ActionInvoked signal
    // #[dbus_interface(signal)]
    // async fn action_invoked(&self, id: u32, activation_token: &str) {}

    // /// NotificationClosed signal
    // #[dbus_interface(signal)]
    // async fn notification_closed(&self, id: u32, reason: u32) {}
}
