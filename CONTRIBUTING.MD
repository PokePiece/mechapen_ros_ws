# 🤝 Contributing to MechaPen ROS Workspace

We're thrilled you're interested in contributing to the MechaPen project! Your contributions, big or small, are incredibly valuable and help us on our journey to building a self-writing mechatronic pen. This guide will help you get started and understand our contribution process.

## 🌟 Welcome!

The MechaPen ROS Workspace is a passion project focused on micro-level mechatronics, robotics, and robust software engineering with Rust and ROS. We believe in collaborative development and welcome anyone passionate about these fields.

## 📝 Code of Conduct

To ensure a welcoming and inclusive environment for everyone, we adhere to a [Code of Conduct](https://www.google.com/search?q=CODE_OF_CONDUCT.md "null") (if you don't have one, consider adding a standard one like the Contributor Covenant). Please review it before participating.

## 🤔 How Can I Contribute?

There are many ways to contribute, not just by writing code!

### 🐛 Reporting Bugs

If you find a bug in the code or simulation, please help us by opening an issue.

- **Search Existing Issues:** Before opening a new issue, please check if a similar one already exists.
    
- **Provide Details:** When creating a bug report, include:
    
    - A clear and concise description of the bug.
        
    - Steps to reproduce the behavior.
        
    - Expected behavior.
        
    - Actual behavior.
        
    - Your operating system, ROS version, and Rust toolchain version.
        
    - Any relevant error messages or logs.
        
    - Screenshots or GIFs if applicable.
        

### ✨ Suggesting Features

Have an idea for a new feature or improvement? We'd love to hear it!

- **Open an Issue:** Describe your proposed feature in detail.
    
- **Explain the Use Case:** Why is this feature needed? How will it benefit the project or users?
    
- **Consider Implementation:** If you have thoughts on how it could be implemented, feel free to share them.
    

### 💻 Submitting Code Changes (Pull Requests)

If you're ready to dive into the code, here's our recommended workflow for submitting pull requests:

#### 1\. Development Setup

Before you start, ensure your development environment is set up. Refer to the **"📦 Getting Started"** section in the main [README.md](# "null") for detailed prerequisites and installation instructions.

In short, you'll need:

- Ubuntu (20.04 LTS or newer)
    
- ROS Noetic or ROS 2 (Foxy/Galactic/Humble)
    
- Rust Toolchain (`rustup`)
    
- `rosdep` and `colcon`
    
- Gazebo & RViz
    

After cloning the repo, remember to:

```
cd mechapen_ros_ws
rosdep install --from-paths src --ignore-src --rosdistro <your_ros_distro> -y
colcon build
source install/setup.bash
```

#### 2\. Create a New Branch

- **Fork the Repository:** Start by forking the `mechatronic_pen_ros_wsl` repository to your own GitHub account.
    
- **Clone Your Fork:**
    
    ```
    git clone https://github.com/YOUR_USERNAME/mechatronic_pen_ros_wsl.git
    cd mechatronic_pen_ros_wsl
    ```
    
- **Create a New Branch:** Create a new branch from `main` for your changes. Use a descriptive name (e.g., `fix/bug-description`, `feat/new-gait-algorithm`).
    
    ```
    git checkout main
    git pull origin main # Ensure your main branch is up-to-date
    git checkout -b your-branch-name
    ```
    

#### 3\. Make Your Changes

- Implement your bug fix or feature.
    
- **Coding Style:**
    
    - For Rust code, please run `cargo fmt` and `cargo clippy` to adhere to standard Rust formatting and catch common lints.
        
    - Follow existing code conventions within the project.
        
- **Testing:**
    
    - If you're adding new functionality, please include relevant tests.
        
    - Ensure all existing tests pass (`colcon test`).
        
    - Verify your changes in the Gazebo/RViz simulation if applicable.
        

#### 4\. Commit Your Changes

- **Atomic Commits:** Try to make small, focused commits that address a single logical change.
    
- **Clear Commit Messages:** Write clear, concise, and descriptive commit messages. A good commit message explains *what* the change is and *why* it was made.
    
    - **Subject Line:** Keep it concise (50-72 characters), use imperative mood (e.g., "Fix: Prevent crash on startup" instead of "Fixed crash on startup").
        
    - **Body (Optional):** Provide more detailed explanation if needed.
        

#### 5\. Push to Your Fork

```
git push origin your-branch-name
```

#### 6\. Open a Pull Request (PR)

- Go to your forked repository on GitHub.
    
- You should see a "Compare & pull request" button or a prompt to create a new pull request from your recently pushed branch.
    
- **Fill out the PR Template:** Our repository might have a PR template. Please fill it out completely, providing:
    
    - A clear title.
        
    - A detailed description of your changes.
        
    - References to any related issues (e.g., `Closes #123`, `Fixes #456`).
        
    - Screenshots or GIFs if your changes are visual.
        
- **Allow Edits from Maintainers:** Please check the box that allows maintainers to make edits to your PR branch. This helps us quickly make minor adjustments or rebase if necessary.
    

## 🔍 Review Process

- Once you open a PR, project maintainers will review your code.
    
- We may ask questions, suggest changes, or request additional information.
    
- Please be responsive to comments and be prepared to iterate on your changes.
    
- We aim to review PRs in a timely manner, but please understand that this is a volunteer effort. Your patience is appreciated!
    

## 📄 License

By contributing to the MechaPen ROS Workspace, you agree that your contributions will be licensed under the [MIT License](https://www.google.com/search?q=LICENSE "null").

Thank you for helping us build the MechaPen! We look forward to your contributions.
