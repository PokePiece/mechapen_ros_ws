cmake_minimum_required(VERSION 3.8) # Or newer if preferred

project(mechapen_description) # Project name from ros2 pkg create

# Find required ROS 2 packages
find_package(ament_cmake REQUIRED)
find_package(ament_lint_auto REQUIRED COMPONENTS xmllint)
find_package(ament_cmake_auto REQUIRED) # For general CMake infrastructure

# Add these lines to install your directories:
install(
  DIRECTORY launch urdf rviz
  DESTINATION share/${PROJECT_NAME}
)

ament_package() # Keep this at the end