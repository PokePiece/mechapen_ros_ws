# 🖊️ MechaPen ROS Workspace: Micro-Level Mechatronic Pen Software & Simulation

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![ROS](https://img.shields.io/badge/ROS-220050?style=for-the-badge&logo=ros&logoColor=white)
![Gazebo](https://img.shields.io/badge/Gazebo-0066B3?style=for-the-badge&logo=gazebo&logoColor=white)
![RViz](https://img.shields.io/badge/RViz-0066B3?style=for-for-the-badge&logo=ros&logoColor=white)
![Mechatronics](https://img.shields.io/badge/Mechatronics-E74C3C?style=for-the-badge&logoColor=white)

## ✨ Project Overview

This repository contains the foundational control software and simulation environment for a **micro-level mechatronic self-writing pen**. As a Mechatronics Software Engineer in 2025, I architected this system to be the robust core software integrated directly onto a physical device, where long-term stability, ultimate control, and clever data handling are paramount. I continuously develop this system for deployment and full motor articulation.

The pen's core software is initiated in **Rust**, constructing a modular, high-performance core. It features an engineered **2D forward kinematics solver** for precise leg movement, integrated with a basic **gait control system** to simulate dynamic locomotion. A powerful Rust-based sandbox has been established, enabling rapid iteration and rigorous testing of complex algorithms independent of hardware constraints. This foundational work is preparing for advanced AI integration and future visualization within Unity.

## 🚀 Features

* **Rust-Powered Core:** Developed in Rust for its unparalleled memory safety, performance, and concurrency features, ensuring the robustness required for embedded mechatronic systems.
* **Core Kinematic Function & Articulation:** Implements the fundamental software responsible for the precise movement and articulation of the mechatronic pen's legs.
* **2D Forward Kinematics Solver:** A custom-engineered solver for accurate calculation of leg end-effector positions based on joint angles, crucial for precise pen placement.
* **Basic Gait Control System:** Simulates dynamic locomotion, providing foundational control over the pen's walking or crawling movements.
* **ROS (Robot Operating System) Integration:** Leverages the ROS framework for modularity, inter-process communication, and access to a vast ecosystem of robotics tools.
* **RViz Simulation:** Utilizes RViz for real-time visualization of the pen's articular and motor functions, allowing for detailed inspection of joint states and movements.
* **Gazebo Simulation:** Provides a comprehensive 3D simulation environment for live kinematic display and rigorous testing of the pen's physical interactions within a virtual world.
* **Rust-Based Sandbox:** A dedicated environment for rapid iteration and rigorous testing of complex control algorithms, decoupled from hardware dependencies.

## ⚙️ Technical Details

This project is structured as a standard **ROS workspace** with **Rust** as the primary development language for ROS nodes.

* **Language:** Rust
* **Robotics Framework:** ROS (Robot Operating System)
* **Simulation Environments:**
    * **RViz:** For visualizing robot models (URDF), sensor data, and planning outputs.
    * **Gazebo:** For high-fidelity physics simulation and testing of robot kinematics and dynamics in a simulated environment.
* **Kinematics:** Focus on 2D forward kinematics for leg movement.
* **Control:** Basic gait control system.

## 📦 Getting Started

To set up and run the MechaPen software and simulation, you will need a Linux environment with ROS installed.

### Prerequisites

* **Ubuntu (20.04 LTS or newer recommended)**
* **ROS Noetic or ROS 2 (Foxy/Galactic/Humble recommended):** Ensure you have a full ROS installation.
* **Rust Toolchain:** Install `rustup` and the Rust toolchain.
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
    ```
* **`rosdep`:** Initialize and update `rosdep`.
    ```bash
    sudo rosdep init
    rosdep update
    ```
* **`colcon`:** The ROS build tool.
    ```bash
    sudo apt install python3-colcon-common-extensions
    ```
* **Gazebo & RViz:** These should come with a full ROS installation.

### Installation & Build

1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/PokePiece/mechapen_ros_ws.git](https://github.com/PokePiece/mechapen_ros_ws.git)
    cd mechapen_ros_ws
    ```
2.  **Initialize ROS dependencies:**
    ```bash
    rosdep install --from-paths src --ignore-src --rosdistro <your_ros_distro> -y
    # Replace <your_ros_distro> with 'noetic', 'foxy', 'humble', etc.
    ```
3.  **Build the ROS workspace:**
    ```bash
    colcon build
    ```
    This will compile your Rust packages within the ROS workspace.

### Running the Simulation

1.  **Source the workspace:**
    ```bash
    source install/setup.bash
    ```
    (Add this to your `~/.bashrc` for convenience: `echo "source ~/mechapen_ros_ws/install/setup.bash" >> ~/.bashrc && source ~/.bashrc`)

2.  **Launch Gazebo and RViz (example launch file):**
    You would typically have a ROS launch file to bring up your simulation. Assuming a launch file like `mechapen_gazebo.launch` in one of your packages:
    ```bash
    roslaunch mechapen_pkg mechapen_gazebo.launch
    ```
    (Replace `mechapen_pkg` with the actual name of your ROS package containing the launch file.)

    This command will:
    * Start the Gazebo simulation environment, loading the mechatronic pen model.
    * Start RViz, displaying the robot's state and potentially sensor data.
    * Launch your Rust-based ROS nodes responsible for kinematics and gait control.

3.  **Interact with the pen:**
    Depending on your implementation, you might interact via ROS topics/services (e.g., sending velocity commands via `rostopic pub`) or a simple GUI if one is developed.

## 📈 Roadmap & Future Plans

The `mechapen_ros_ws` is a living project with ambitious goals:

* **Advanced Gait Control:** Developing more sophisticated and adaptive gait algorithms for varied terrains and maneuvers.
* **Inverse Kinematics:** Implementing inverse kinematics to control the pen's end-effector position directly, simplifying writing tasks.
* **AI Integration:** Incorporating machine learning models for intelligent path planning, obstacle avoidance, and autonomous writing generation.
* **Sensor Fusion:** Integrating data from multiple simulated (and eventually real) sensors for robust state estimation.
* **Unity Visualization:** Expanding the simulation to include high-fidelity visualization and interaction within a Unity environment, leveraging its rendering capabilities.
* **Real-world Hardware Deployment:** The ultimate goal is to deploy this software onto a physical mechatronic pen device.

## 🤝 Contributing

Contributions are highly encouraged! If you're interested in robotics, Rust, ROS, or mechatronics, feel free to open issues, suggest features, or submit pull requests.

## 📄 License

This project is licensed under the MIT License - see the `LICENSE` file for details.

---

Explore the code and join the journey of building a self-writing mechatronic pen!
