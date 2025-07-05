use notify_rust::{Hint, Notification, NotificationHandle, Urgency};
use std::thread::sleep;
use std::time::Duration;
use sysinfo::Components;

fn main() {
    let mut handle: Option<NotificationHandle> = None;
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
                if let Some(ref mut h) = handle {
                    h.body(&body_text);
                    h.update();
                } else {
                    handle = Notification::new()
                        .appname("Temperature Monitor")
                        .summary("Temperature dangerously high")
                        .body(&body_text)
                        .icon("dialog-warning")
                        .urgency(Urgency::Critical)
                        .hint(Hint::Resident(true))
                        .timeout(0)
                        .show()
                        .inspect_err(|err| eprintln!("Error showing notification: {:?}", err))
                        .ok();
                }
            } else {
                if let Some(h) = handle {
                    h.close();
                    handle = None;
                }
            }
        }
        sleep(Duration::from_secs(3));
        components.refresh(false)
    }
}
