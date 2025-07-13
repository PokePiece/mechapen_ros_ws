// src/sensors.rs

/// A trait defining the interface for reading sensor data.
pub trait SensorReader {
    fn read_all_sensors(&self) -> SensorData;
    // Add more specific sensor reading methods as needed
    fn read_pen_tip_force(&self) -> f32;
    fn read_imu_data(&self) -> (f32, f32, f32); // (pitch, roll, yaw)
}

/// A struct to hold all sensor data.
#[derive(Debug)]
pub struct SensorData {
    pub pen_tip_force: f32,
    pub imu_orientation: (f32, f32, f32),
    // Add more sensor fields as your design evolves
}

/// A mock implementation of SensorReader for testing on a host PC.
pub struct MockSensorReader;

impl MockSensorReader {
    pub fn new() -> Self {
        MockSensorReader
    }
}

impl SensorReader for MockSensorReader {
    fn read_all_sensors(&self) -> SensorData {
        // Return some dummy, changing data for simulation
        // In a real mock, you might feed it data from a file or another source.
        SensorData {
            pen_tip_force: 0.5, // Dummy value
            imu_orientation: (0.1, 0.2, 0.3), // Dummy values
        }
    }

    fn read_pen_tip_force(&self) -> f32 {
        println!("(Mock) Reading pen tip force...");
        0.5 // Dummy value
    }

    fn read_imu_data(&self) -> (f32, f32, f32) {
        println!("(Mock) Reading IMU data...");
        (0.1, 0.2, 0.3) // Dummy values
    }
}