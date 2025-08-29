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
            if let Some(temp) = component.temperature() {
                println!(" - {}: {:.1}°C", component.label(), temp);
            }
        }

        // Priority 1: Look for CPU package temperature (whole CPU)
        for component in &self.components {
            let label = component.label().to_lowercase();
            if label.contains("package") && label.contains("cpu") ||
               label.contains("package id") {
                if let Some(temp) = component.temperature() {
                    println!("🎯 Using CPU package temperature: {:.1}°C", temp);
                    return Some(temp);
                }
            }
        }

        // Priority 2: Look for other CPU-related temperature sensors
        for component in &self.components {
            let label = component.label().to_lowercase();
            if label.contains("cpu") ||
               label.contains("processor") {
                if let Some(temp) = component.temperature() {
                    println!("🎯 Using CPU sensor temperature: {:.1}°C", temp);
                    return Some(temp);
                }
            }
        }

        // Priority 3: Average of all core temperatures if no package temp found
        let mut core_temps = Vec::new();
        for component in &self.components {
            let label = component.label().to_lowercase();
            if label.contains("core") {
                if let Some(temp) = component.temperature() {
                    core_temps.push(temp);
                }
            }
        }

        if !core_temps.is_empty() {
            let avg_temp = core_temps.iter().sum::<f32>() / core_temps.len() as f32;
            println!("🎯 Using average core temperature: {:.1}°C", avg_temp);
            return Some(avg_temp);
        }

        // Priority 4: If no CPU-specific sensor found, return the first available temperature
        for component in &self.components {
            if let Some(temp) = component.temperature() {
                println!("🎯 Using first available temperature: {:.1}°C", temp);
                return Some(temp);
            }
        }

        None
    }
}
