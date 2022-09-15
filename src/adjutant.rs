use zbus::dbus_interface;

pub struct Adjutant;
#[dbus_interface(name = "catthingy.Adjutant")]
impl Adjutant {
    async fn say_hello(&self, name: &str) -> String {
        format!("Hello {}!", name)
    }
}
