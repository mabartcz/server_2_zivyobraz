use sysinfo::{Components, System};

pub struct SystemMonitor {
    system: System,
    components: Components,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            components: Components::new_with_refreshed_list(),
        }
    }

    pub fn get_cpu_temperature(&mut self) -> Option<f32> {
        // First we update all information of our `System` struct.
        self.system.refresh_all();

        // Refresh the components to get latest temperature readings
        self.components.refresh(false);

        println!("Available components:");
        for component in &self.components {
            println!(" - {}: {:.1}°C", component.label(), component.temperature()?);
        }

        // Look for CPU-related temperature sensors
        for component in &self.components {
            let label = component.label().to_lowercase();

            // Check for common CPU temperature sensor names
            if label.contains("cpu") ||
               label.contains("core") ||
               label.contains("processor") ||
               label.contains("package") {
                return Some(component.temperature()?);
            }
        }

        // If no CPU-specific sensor found, return the first available temperature
        self.components.iter().next().map(|component| component.temperature())?
    }
}
