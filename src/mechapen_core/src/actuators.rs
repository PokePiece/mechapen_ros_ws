// src/actuators.rs

use std::collections::HashMap;

/// A trait defining the interface for controlling actuators (motors).
pub trait MotorController {
    fn set_motor_positions(&mut self, positions: &HashMap<String, f32>);
    // Add more specific motor control methods as needed
    fn set_motor_speed(&mut self, motor_id: &str, speed: f32);
}

/// A mock implementation of MotorController for testing on a host PC. 
///test 
pub struct MockMotorController {
    // In a real mock, you might store the last commanded positions
    // for verification in tests.
    pub last_commanded_positions: HashMap<String, f32>,
}

impl MockMotorController {
    pub fn new() -> Self {
        MockMotorController {
            last_commanded_positions: HashMap::new(),
        }
    }
}

impl MotorController for MockMotorController {
    fn set_motor_positions(&mut self, positions: &HashMap<String, f32>) {
        println!("(Mock) Setting motor positions: {:?}", positions);
        self.last_commanded_positions = positions.clone(); // Store for inspection if needed
    }

    fn set_motor_speed(&mut self, motor_id: &str, speed: f32) {
        println!("(Mock) Setting motor {} speed to {}", motor_id, speed);
    }
}