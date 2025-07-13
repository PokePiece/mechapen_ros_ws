// src/main.rs

// Declare modules for different aspects of the MechaPen's software.
mod kinematics;      // For calculating joint positions and inverse kinematics
mod gait_control;    // For managing walking patterns
mod writing_engine;  // For converting text/drawing into pen movements
mod sensors;         // For abstracting sensor inputs (e.g., pen tip force, IMU)
mod actuators;       // For abstracting motor/actuator outputs
mod ai_core;         // For high-level decision making and integrated AI

use crate::sensors::SensorReader; // Add this line
use crate::actuators::MotorController; // Add this line

// The main entry point of our MechaPen software.
fn main() {
    println!("MechaPen Core Software booting up...");

    // --- 1. Initialize Mock Hardware Interfaces ---
    // For now, these are just "dummy" implementations that print messages
    // instead of talking to real hardware.
    let mut mock_motor_controller = actuators::MockMotorController::new();
    let mock_sensor_reader = sensors::MockSensorReader::new();

    // --- 2. Initialize Core Components ---
    let mut pen_kinematics = kinematics::PenKinematics::new();
    let mut pen_gait_controller = gait_control::GaitController::new();
    let mut pen_writing_engine = writing_engine::WritingEngine::new();
    let mut pen_ai = ai_core::MechaPenAI::new();

    println!("MechaPen Core components initialized.");

    // --- 3. Main Control Loop (Simulated) ---
    // In a real embedded system, this would be a tight loop or an RTOS task.
    // For now, we'll run a few simulated steps.
    for step in 0..10 {
        println!("\n--- Simulation Step {} ---", step);

        // Simulate reading sensor data
        let sensor_data = mock_sensor_reader.read_all_sensors();
        println!("Sensor data: {:?}", sensor_data);

        // AI processes sensor data and decides next high-level action
        let ai_command = pen_ai.process_sensors(&sensor_data);
        println!("AI command: {:?}", ai_command);

        // Gait controller calculates leg movements based on AI command
        let leg_joint_targets = pen_gait_controller.calculate_gait(&ai_command, step);
        println!("Leg joint targets: {:?}", leg_joint_targets);

        // Kinematics converts joint targets to actual motor positions
        let motor_positions = pen_kinematics.calculate_motor_positions(&leg_joint_targets);
        println!("Motor positions: {:?}", motor_positions);

        // Actuators send commands to motors
        mock_motor_controller.set_motor_positions(&motor_positions);

        // Simulate writing action (if AI commanded it)
        if ai_command == ai_core::AiCommand::Write {
            pen_writing_engine.write_character('A'); // Example: write a character
        }

        // In a real system, you'd have a delay or time management here.
        // For simulation, we just advance the 'step' counter.
    }

    println!("\nMechaPen Core Software simulation finished.");
}