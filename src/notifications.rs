use std::{collections::HashMap, time::Duration};

use async_std::{
    channel::Sender,
    sync::{Arc, RwLock},
};
use zbus::{dbus_interface, SignalContext};

#[derive(Debug, Clone)]
pub struct Notification {
    pub app_name: String,
    pub summary: String,
    pub timer: u32,
}

pub type Notifications = Arc<RwLock<Vec<(u32, Notification)>>>;

pub struct NotificationHandler {
    pub notifications: Notifications,
    pub next_id: u32,
    pub tx: Sender<()>,
}

impl NotificationHandler {
    pub fn new(notifications: Notifications, tx: Sender<()>) -> NotificationHandler {
        NotificationHandler {
            notifications,
            next_id: 1,
            tx,
        }
    }
}

#[dbus_interface(interface = "org.freedesktop.Notifications")]
impl NotificationHandler {
    /// CloseNotification method
    async fn close_notification(
        &mut self,
        id: u32,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
    ) {
        let mut notifications = self.notifications.write().await;
        if let Some(index) = notifications
            .iter()
            .position(|(notif_id, _)| notif_id == &id)
        {
            notifications.remove(index);
            Self::notification_closed(&ctxt, id, 3).await.unwrap();
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
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
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
                    timer: 0,
                },
            );
        } else {
            notifications.push((
                new_id,
                Notification {
                    app_name: app_name.to_string(),
                    summary: summary.to_string(),
                    timer: 0,
                },
            ));
        }

        let task_ctxt = ctxt.into_owned();
        let task_notifications = self.notifications.clone();
        let task_tx = self.tx.clone();

        async_std::task::spawn(async move {
            let timeout = if expire_timeout == -1 {
                5000
            } else if expire_timeout > 0 {
                expire_timeout as u64
            } else {
                return;
            };
            async_std::task::sleep(Duration::from_millis(timeout)).await;
            let mut notifications = task_notifications.write().await;
            if let Some(index) = notifications
                .iter()
                .position(|(notif_id, _)| notif_id == &new_id)
            {
                notifications.remove(index);
                Self::notification_closed(&task_ctxt, new_id, 3)
                    .await
                    .unwrap();
                task_tx.send(()).await.unwrap();
            }
        });

        self.tx.send(()).await.unwrap();

        new_id
    }

    // /// ActionInvoked signal
    // #[dbus_interface(signal)]
    // pub async fn action_invoked(
    //     ctxt: &SignalContext<'_>,
    //     id: u32,
    //     activation_token: &str,
    // ) -> Result<(), zbus::Error>;

    /// NotificationClosed signal
    #[dbus_interface(signal)]
    pub async fn notification_closed(
        ctxt: &SignalContext<'_>,
        id: u32,
        reason: u32,
    ) -> Result<(), zbus::Error>;
}
