# E-Jet Systems v3 (E170/E175/E190/E195) for MSFS

## Overview

This is the third iteration of my custom-built systems for the Embraer E-Jet family (E170, E175, E190, and E195) in Microsoft Flight Simulator (MSFS). The goal is to create highly accurate and efficient aircraft systems using Rust.

## Why a Third Rewrite?

After two previous iterations in different languages, I've landed back on Rust for a few key reasons:

1. **Original Rust Version (v1)** - The first version was written in Rust, leveraging Tokio for async and multithreaded execution. However, over time, I found this approach to rely too heavily on half baked tools for launching the systems at the proper time. This caused errors where the aircraft would not have functional systems far too often as well as many antivirus issues from launching random unsigned exes on the user's pc

2. **C++ Rewrite (v2)** - The second version was written in C++, which allowed for lower-level control and performance optimization. While the system logic was solid, maintaining and understanding a large C++ codebase proved challenging for devs outside of my small friend group, mostly due to the bleeding edge C++ standard that was used. As a result of this, massive delays in progress were created when the small pool of developers (2) couldn't work on it consistently.

3. **Rust Rewrite (v3 - Current)** - This version returns to Rust, but with a more structured and maintainable approach. Dropping the heavy async/multithreading model used in v1 for full WASM and native support (like the C++ repo), this version focuses on clarity, safety, and ease of maintenance while still ensuring high performance.

## Features

- Accurate system simulations for the E-Jet series in MSFS

- Improved maintainability over the previous C++ version

- Better safety and memory management thanks to Rust

- Optimized performance without unnecessary complexity

- Scalable architecture to allow for future expansions and enhancements

- Written in an easy language this time

# Roadmap

- [ ] Complete refactor of the C++ repository
  - [x] Project setup, boilerplate for testing GUI
  - [x] Template classes refactored to traits & structs
  - [ ] System components refactored
  - [ ] Mass system logic & math
  - [ ] System structure & layout
  - [ ] Simulation
  - [ ] Communication
- [ ] Expand upon the C++ repo
  - [ ] Air conditioning system
  - [ ] FADEC & Enging Simulation
  - [ ] Pneumatic System
  - [ ] Random Systems that only serve to enhance frontend of aircraft (like cabin simulation)

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests with improvements and bug fixes.

## Contact

For discussions or questions, feel free to reach out via GitHub issues or [Discord](https://discord.gg/GhkQ9wrrbp).
