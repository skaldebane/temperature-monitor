use notify_rust::{Hint, Notification, NotificationHandle, Urgency};
use std::thread::sleep;
use std::time::Duration;
use sysinfo::Components;

fn main() {
    let mut handle: notify_rust::error::Result<NotificationHandle> =
        Err(notify_rust::error::Error::from(""));
    let mut components = Components::new_with_refreshed_list();
    loop {
        let acpi_component = components
            .iter()
            .find(|it| it.label().starts_with("acpitz temp1"));
        if let Some(Some(temp)) = acpi_component.map(|it| it.temperature()) {
            if temp > 80.0 {
                println!("Temperature dangerously high: {}", temp);
                let body_text = format!(
                    "Your computer's temperature ({temp:.0}°C) exceeds safe limits. Please check the fan's ventilation."
                );
                if let Ok(ref mut h) = handle {
                    h.body(&body_text);
                    h.update();
                } else {
                    handle = Notification::new()
                        .appname("Temperature Monitor")
                        .summary("Temperature dangerously high")
                        .body(&body_text)
                        .icon("dialog-warning")
                        .urgency(Urgency::Normal)
                        .hint(Hint::Resident(true))
                        .timeout(0)
                        .show()
                        .inspect_err(|err| eprintln!("Error showing notification: {:?}", err));
                }
            } else {
                if let Ok(h) = handle {
                    h.close();
                    handle = Err(notify_rust::error::Error::from(""));
                }
            }
        }
        sleep(Duration::from_secs(3));
        components.refresh(false)
    }
}
