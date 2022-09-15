use async_std::{
    channel::Sender,
    sync::{Arc, RwLock},
};
use zbus::dbus_interface;

use crate::notifications::Notifications;

pub struct Adjutant {
    notifications: Notifications,
    current: Arc<RwLock<Option<u32>>>,
    tx: Sender<()>,
}

impl Adjutant {
    pub fn new(
        notifications: Notifications,
        current: Arc<RwLock<Option<u32>>>,
        tx: Sender<()>,
    ) -> Self {
        Adjutant {
            notifications,
            current,
            tx,
        }
    }
}

#[dbus_interface(name = "catthingy.Adjutant")]
impl Adjutant {
    async fn close_current(&self) {}
    async fn expand_current(&self) {}
    async fn next(&self) {}
    async fn prev(&self) {}
}
