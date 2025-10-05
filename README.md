# Rust Ananicy - Auto Nice Daemon

A high-performance Rust implementation of Ananicy (Auto Nice Daemon) - a system service that automatically manages process priorities using nice levels, I/O scheduling, CPU scheduling, and cgroups.

## Features

- 🚀 **High Performance**: Built in Rust for maximum efficiency and low resource usage
- ⚡ **Auto Priority Management**: Automatically applies nice values to processes based on configurable rules
- 🎯 **Multiple Scheduling Classes**: Supports nice, ionice, CPU schedulers, and cgroup management
- 🔧 **Flexible Configuration**: JSON-based rule system with inheritance through types
- 📊 **Process Monitoring**: Continuous process scanning and rule application
- 🐳 **Cgroup Integration**: Automatic cgroup assignment with CPU quota management
- 🔄 **Hot-Reload Ready**: File watching support for configuration changes
- 🛠️ **Systemd Integration**: Native systemd service support with readiness notifications

## Quick Start

### Installation

```bash
# Build from source
cargo build --release

# Install system-wide
sudo cp target/release/rust-ananicy /usr/local/bin/
sudo mkdir -p /etc/ananicy.d/
```

### Configuration

Create rule files in `/etc/ananicy.d/`:

**Example rule** (`/etc/ananicy.d/00-default.rules`):
```json
{"name": "chrome", "nice": -5, "cgroup": "browsers"}
{"name": "firefox", "nice": -5, "cgroup": "browsers"}
{"name": "make", "nice": -15, "ioclass": "idle"}
```

**Cgroup definition** (`/etc/ananicy.d/cgroups.cgroups`):
```json
{"cgroup": "browsers", "CPUQuota": 80}
{"cgroup": "games", "CPUQuota": 90}
```

### Running

```bash
# Run directly
rust-ananicy start

# Or install as systemd service
sudo systemctl enable rust-ananicy
sudo systemctl start rust-ananicy
```

## Usage

```bash
# Start the daemon
rust-ananicy start

# Dump loaded rules
rust-ananicy dump rules

# Show current processes
rust-ananicy dump proc

# List available cgroups
rust-ananicy dump cgroups

# With custom config directory
rust-ananicy --config-dir /etc/my-ananicy/ start
```

## Configuration Structure

### Rule Properties
- `name`: Process name matching
- `type`: Inherit from type definition
- `nice`: Nice value (-20 to 19)
- `ioclass`: I/O scheduling class (none, realtime, best-effort, idle)
- `ionice`: I/O nice level (0-7)
- `sched`: CPU scheduler (other, rr, fifo, batch, iso, idle)
- `rtprio`: Real-time priority (1-99)
- `oom_score_adj`: OOM killer adjustment (-1000 to 1000)
- `cgroup`: Cgroup assignment
- `cmdlines`: Command line pattern matching

### Type Definitions
Create `.types` files for rule inheritance:

```json
{"type": "desktop-app", "nice": -5, "cgroup": "desktop"}
{"type": "background", "nice": 10, "ioclass": "idle"}
```

## Performance Benefits

- **Memory Efficient**: ~5-10MB RAM vs 50-100MB for Python version
- **Fast Startup**: Sub-second initialization
- **Low CPU**: Efficient async scanning with configurable intervals
- **Native Performance**: No interpreter overhead

## Requirements

- Linux kernel 4.15+
- systemd (optional)
- cgroup v2 support
- ionice and schedtool utilities

## Safety Features

- Memory safe implementation
- Proper error handling
- Process isolation
- Resource limits enforcement
- Graceful degradation

## Contributing

This is a Rust rewrite of the original [Ananicy](https://github.com/Nefelim4ag/Ananicy) project, aiming to provide better performance and reliability while maintaining compatibility with existing configurations.

## License

MIT License - see LICENSE file for details.

---

**Note**: This is a work in progress. Some advanced features from the original Ananicy may not be implemented yet.