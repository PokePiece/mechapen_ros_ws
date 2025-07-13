// src/ai_core.rs
use crate::sensors::SensorData; // Import SensorData from sensors module

#[derive(Debug, PartialEq)] // Derive PartialEq for comparison in main.rs
pub enum AiCommand {
    Idle,
    Walk,
    Write,
    AdjustPosition,
    // Add more commands as needed
}

#[derive(Debug)]
pub struct MechaPenAI {
    // AI state, learning models, etc.
}

impl MechaPenAI {
    pub fn new() -> Self {
        MechaPenAI {}
    }

    pub fn process_sensors(&mut self, sensor_data: &SensorData) -> AiCommand {
        println!("(AI) Processing sensor data: {:?}", sensor_data);
        // Simple AI logic: if pen tip pressed, switch to write mode every few steps
        if sensor_data.pen_tip_force > 0.4 {
            AiCommand::Write
        } else {
            AiCommand::Walk // Default to walking
        }
        // This is where your complex AI logic would reside
    }
}