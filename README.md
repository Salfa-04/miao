# Rust Embedded Quickstart Template

A `cargo-generate` template for Rust embedded projects on STM32 chips. It includes Embassy, Defmt, and Probe-rs configurations.

## Features

- **Configuration**: Sets the target architecture based on the chip model.
- **Embassy**: Includes Embassy Executor and common components.
- **Defmt**: Includes `defmt` and `defmt-rtt` logging with RTT support.
- **Structure**: Organized into `controller`, `system`, and `tasks` modules.
- **Debugging**:
  - **Probe-rs**: Default runner.
  - **OpenOCD**: Optional OpenOCD configuration.
  - **RTT Forwarding**: Optional RTT log forwarding.

## Prerequisites

Before using this template, please ensure the following tools are installed:

1. **Rust Toolchain**

   ```bash
   pacman -S rustup
   rustup default stable
   ```

2. **cargo-generate**

   ```bash
   pacman -S cargo-generate
   ```

3. **probe-rs**

   ```bash
   pacman -S probe-rs
   ```

4. **OpenOCD** (Optional)

   ```bash
   pacman -S openocd
   ```

## Quick Start

Generate a new project using the following command:

```bash
cargo generate --git https://github.com/Salfa-04/quickstart --name my-project
```

### Interactive Configuration

During the generation process, you will need to answer the following questions:

1. **Target Chip**: Enter your target chip model (e.g., `stm32g473re`). The template will automatically identify the chip series and set the correct compilation target.
2. **Include RTT Forwarding Support?**: Whether to enable RTT log forwarding support.
   - If enabled, you need to enter the **RTT Server Address** (e.g., `127.0.0.1:1008`).
3. **Include OpenOCD Configuration Files?**: Whether to generate OpenOCD configuration files.
   - If enabled, you need to select the corresponding configuration file (e.g., `stm32g4x.cfg`).

## Project Structure

The generated project contains the following main parts:

```text
.
├── Cargo.toml              # Workspace configuration
├── .cargo/config.toml      # Build and run configuration (Auto-generated Target and Runner)
├── utils/                  # Common utility library (Initialization, macros, etc.)
└── <project-name>/         # Your application crate
    ├── Cargo.toml
    ├── Embed.toml          # probe-rs configuration
    ├── build.rs
    └── src/
        ├── main.rs         # Program entry point
        ├── controller/     # Control logic
        ├── system/         # Hardware abstraction and system configuration (GPIO, Clock, Interrupts, etc.)
        └── tasks/          # Async tasks (Blinky, Health check, etc.)
```

## Build & Run

### Using Probe-rs (Recommended)

The template configures `probe-rs` as the runner by default.

- **Run in Debug Mode**:

  ```bash
  cargo run
  # Or use the alias
  cargo r
  ```

- **Run in Release Mode**:

  ```bash
  cargo run --release
  # Or use the alias
  cargo rr
  ```

### Using OpenOCD

If you chose to generate OpenOCD configuration:

- Start the OpenOCD Service:

   ```bash
   openocd -f openocd.cfg
   ```

- Download and Run the Program:

   ```bash
   openocd -f <project-name>/openocd.cfg
   ```

   *(Note: You may need to modify the runner configuration in `.cargo/config.toml` to adapt to GDB/OpenOCD)*

## FAQ

- **Which chips are supported?**
  Currently, the script supports common STM32 series (F0, F1, F3, F4, F7, G0, G4, H7, L0, L4, U5, etc.). If you encounter an unsupported chip, please check `prehooks.rhai` or manually modify the generated `.cargo/config.toml`.
