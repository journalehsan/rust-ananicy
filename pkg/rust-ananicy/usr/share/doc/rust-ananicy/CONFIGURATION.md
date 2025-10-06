# Configuration Guide

## Overview

Rust Ananicy comes with comprehensive default configuration covering 200+ popular applications across different categories. The configuration system is designed to be both powerful and easy to customize.

## Configuration Structure

```
/etc/ananicy.d/
├── ananicy.conf           # Main daemon configuration
├── 00-default.cgroups     # CPU quota cgroups definitions  
├── 00-types.types         # Process type definitions
├── 10-desktop.rules       # Desktop environment rules
├── 20-browsers.rules      # Web browser optimizations
├── 30-development.rules   # IDEs, compilers, build tools
├── 40-applications.rules  # General applications
├── 50-gaming.rules        # Games and launchers
└── 80-background.rules    # Background services
```

## Rule Priority

Rules are processed in alphabetical order by filename. Use prefixes to control priority:
- `00-` - System essentials and types
- `10-` - Desktop environment
- `20-` - Interactive applications
- `80-` - Background processes
- `90-` - Custom overrides

## Included Application Categories

### Desktop Environments
- **KDE Plasma**: Full desktop stack optimization
- **GNOME**: Shell, session, and services
- **XFCE**: Lightweight desktop components
- **Audio**: PipeWire, PulseAudio with real-time priority

### Web Browsers  
- **Chromium-based**: Chrome, Edge, Brave, Opera, Vivaldi
- **Firefox**: Main process and content isolation
- **Renderer processes**: Lower priority to prevent UI blocking

### Development Tools
- **IDEs**: VS Code, JetBrains suite, Atom, Sublime Text
- **Compilers**: GCC, Clang, Rust, Go with build tool optimization
- **Version Control**: Git, SVN with appropriate priorities

### Applications
- **Office**: LibreOffice, OnlyOffice suite
- **Media**: VLC, MPV, Spotify, video editors
- **Communication**: Discord, Telegram, Slack, email clients
- **Graphics**: GIMP, Krita, Blender

### Gaming
- **Launchers**: Steam, Lutris, Heroic optimized
- **Wine/Proton**: Windows compatibility layers
- **Emulators**: Dolphin, PCSX2, RetroArch

### Background Services
- **Package Managers**: APT, DNF, Pacman, Flatpak
- **Backup**: rsync, Borg, Restic
- **Cloud Sync**: Dropbox, Nextcloud
- **P2P**: BitTorrent clients with traffic limiting

## Customization Examples

### Custom Application Rule
```json
{"name": "my-app", "nice": -5, "ioclass": "best-effort", "ionice": 2, "cgroup": "desktop"}
```

### Command Line Pattern Matching
```json
{"name": "python", "cmdlines": ["machine-learning"], "nice": 5, "cgroup": "cpu80"}
```

### Gaming Priority Boost
```json
{"name": "my-game.exe", "nice": -10, "ioclass": "realtime", "oom_score_adj": -100}
```

### Development Workflow
```json
{"name": "webpack", "nice": 10, "ioclass": "idle", "cgroup": "cpu50"}
{"name": "node", "cmdlines": ["--inspect"], "nice": -2}
```

## Process Type System

Types allow rule inheritance and easier maintenance:

```json
// Define a type
{"type": "my-workflow", "nice": -3, "ioclass": "best-effort", "cgroup": "desktop"}

// Use the type
{"name": "my-app", "type": "my-workflow"}
```

## Cgroup CPU Limiting

Predefined cgroups for different CPU quotas:
- `cpu95` - High priority (games, media)
- `cpu80` - Normal desktop apps
- `cpu50` - Background tasks
- `cpu20` - Heavy downloads, torrents

## Priority Guidelines

### Nice Values (-20 to +19)
- `-15` to `-10`: Critical system processes
- `-10` to `-5`: Desktop environment, audio
- `-5` to `0`: Interactive applications
- `0` to `5`: Background applications  
- `5` to `15`: Batch/non-interactive tasks

### I/O Classes
- `realtime`: Audio servers, critical system processes
- `best-effort`: Normal applications (levels 0-7)
- `idle`: Background tasks, downloads

### OOM Score Adjustment (-1000 to +1000)
- `-500`: Critical desktop components
- `-200`: Audio/media servers
- `+100`: Web browsers
- `+300`: Browser renderer processes

## Monitoring and Tuning

### Check Applied Rules
```bash
# See all loaded rules
rust-ananicy dump rules

# Check specific process
rust-ananicy dump proc | grep firefox

# Monitor service logs
journalctl -u rust-ananicy -f
```

### Performance Monitoring
```bash
# System overview
htop -t    # Tree view
btop       # Modern alternative

# Process priorities
ps -eo pid,ni,pri,pcpu,comm --sort=-pcpu

# I/O priorities  
iotop -a

# Memory usage
free -h && ps_mem
```

### Custom Tuning Tips

1. **Identify Resource Hogs**
   ```bash
   # CPU usage
   ps -eo pid,pcpu,comm --sort=-pcpu | head -20
   
   # Memory usage  
   ps -eo pid,pmem,comm --sort=-pmem | head -20
   ```

2. **Gaming Optimization**
   - Lower nice values for games (-8 to -10)
   - Higher nice for background apps during gaming
   - Use `cpu95` cgroup for demanding games

3. **Development Workflow**
   - Balance IDE responsiveness vs build speed
   - Use `idle` I/O class for file indexing
   - Limit CPU quota for resource-heavy builds

4. **Media Workstation**
   - Real-time priority for audio servers
   - High priority for video editors
   - Background rendering with CPU limits

## Troubleshooting

### Rules Not Applied
- Check file permissions: `ls -la /etc/ananicy.d/`
- Verify JSON syntax: `rust-ananicy dump rules`
- Monitor logs: `journalctl -u rust-ananicy`

### Performance Issues
- Reduce scan frequency in `ananicy.conf`
- Disable unused rule categories
- Monitor daemon resource usage

### Permission Errors
- Ensure cgroup filesystem is mounted
- Check systemd service permissions
- Verify ionice/schedtool availability

This configuration provides an excellent starting point for most systems while remaining easily customizable for specific needs.