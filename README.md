# Sys Info Lib

Rust library to fetch system information
Need to interact with the operating system to fetch the required information. This can be done using various crates or by directly interfacing with system APIs. For example, to get CPU usage, you might read from `/proc/stat` on Linux

```
sysinfo_lib/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── cpu.rs
│   ├── memory.rs
│   ├── disk.rs
│   └── utils.rs
├── tests/
│   ├── cpu_tests.rs
│   ├── memory_tests.rs
│   └── disk_tests.rs
└── examples/
    └── main.rs
```

## Breakdown of the Structure
- Cargo.toml: Your project configuration file where you specify dependencies like Tokio.
- src/: The source directory containing your library code.
- lib.rs: The main entry point for your library. It will re-export modules and provide the main API.
- cpu.rs: Module for fetching CPU-related information.
- memory.rs: Module for fetching memory-related information.
- disk.rs: Module for fetching disk-related information.
- utils.rs: Utility functions that can be shared across modules.
- tests/: Directory for integration tests.
- cpu_tests.rs: Tests for CPU-related functions.
- memory_tests.rs: Tests for memory-related functions.
- disk_tests.rs: Tests for disk-related functions.
- examples/: Directory for example usage of your library.
- main.rs: An example application demonstrating how to use your library.
