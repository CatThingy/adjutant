use async_std::{
    channel::Sender,
    sync::{Arc, RwLock, RwLockUpgradableReadGuard, RwLockWriteGuard},
};
use zbus::{dbus_interface, SignalContext};

use crate::{
    notifications::{NotificationHandler, Notifications},
    printer::Print,
};

pub struct Adjutant<'a> {
    notifications: Notifications,
    current: Arc<RwLock<Option<usize>>>,
    tx: Sender<Print>,
    notification_signal_ctx: SignalContext<'a>,
}

impl<'a> Adjutant<'a> {
    pub fn new(
        notifications: Notifications,
        current: Arc<RwLock<Option<usize>>>,
        tx: Sender<Print>,
        notification_signal_ctx: SignalContext<'a>,
    ) -> Self {
        Adjutant {
            notifications,
            current,
            tx,
            notification_signal_ctx,
        }
    }
}

#[dbus_interface(name = "catthingy.Adjutant")]
impl Adjutant<'static> {
    async fn close_current(&mut self) {
        let current = self.current.upgradable_read().await;
        if let Some(index) = *current {
            let mut notifications = self.notifications.write().await;

            let id = notifications.remove(index).0;
            NotificationHandler::notification_closed(&self.notification_signal_ctx, id, 2)
                .await
                .unwrap();
            let notifications = RwLockWriteGuard::downgrade(notifications);
            if notifications.len() == 0 {
                let mut current = RwLockUpgradableReadGuard::upgrade(current).await;
                *current = None;
            } else if index >= notifications.len() {
                let mut current = RwLockUpgradableReadGuard::upgrade(current).await;
                *current = Some(notifications.len() - 1);
            }

            self.tx.send(Print).await.unwrap();
        }
    }

    async fn next(&self) {
        let mut current = self.current.write().await;
        if let Some(index) = *current {
            let notifications = self.notifications.read().await;
            *current = Some((index + 1) % notifications.len());
            self.tx.send(Print).await.unwrap();
        }
    }

    async fn prev(&self) {
        let mut current = self.current.write().await;
        if let Some(mut index) = *current {
            let notifications = self.notifications.read().await;
            if index == 0 {
                index = notifications.len();
            }
            *current = Some(index - 1);
            self.tx.send(Print).await.unwrap();
        }
    }

    async fn invoke(&self) {
        let current = self.current.read().await;
        if let Some(index) = *current {
            let notifications = self.notifications.read().await;
            let (notif_id, _) = notifications[index];
            NotificationHandler::action_invoked(&self.notification_signal_ctx, notif_id, "default")
                .await
                .unwrap();
        }
    }
}
