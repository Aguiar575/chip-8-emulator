# Chip-8 Emulator

This is a simple Chip-8 emulator written in Rust. The emulator provides a basic framework for running Chip-8 programs and includes functionality such as display rendering, keyboard input, and timers.

## Attribution

The codebase of this emulator incorporates concepts, techniques, and structures outlined in the [Chip-8 Book](https://github.com/aquova/chip8-book). The original work by aquova has been a valuable reference in understanding and implementing key components of a Chip-8 emulator.

## Features

- **Screen:** Emulates a 64x32 monochrome display.
- **Memory:** 4KB of memory, including a fontset.
- **Registers:** 16 general-purpose registers (`V0` to `VF`).
- **Stack:** 16-level stack to handle subroutine calls.
- **Timers:** Emulation of delay and sound timers.
- **Input:** Emulates a 16-key hexadecimal keypad.
- **Instructions:** Supports a subset of Chip-8 instructions.

## Usage

To use the emulator, follow these steps:

1. **Install Rust:** Make sure you have Rust installed on your system. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

2. **Clone the Repository:**
   ```bash
   git clone https://github.com/your-username/chip8-emulator.git
   cd chip8-emulator
   ```

3. **Build and Run:**
   ```bash
   cargo run
   ```

   This command will build and run the emulator. You can then load Chip-8 programs into the emulator.

4. **Load a Chip-8 Program:**
   - The emulator supports loading Chip-8 programs using the `load` method. Modify the `main.rs` file or create a separate program file to load and run your Chip-8 programs.

## Controls

The emulator uses a hexadecimal keypad layout:

```
1 2 3 C
4 5 6 D
7 8 9 E
A 0 B F
```

These keys correspond to the original Chip-8 keypad layout. Customize your input mappings as needed.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. Feel free to use, modify, and distribute this code for your own projects.

## Contributing to the Chip-8 Book

If you find the Chip-8 Book helpful and wish to contribute or provide feedback, consider checking out the [original repository](https://github.com/aquova/chip8-book). Contributions and community involvement are valuable in enhancing educational resources like this one.
