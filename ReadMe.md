# Modelica Rust GDExtension

Godot 4 GDExtension for integrating Modelica physics simulations into your games.

## Features

- 🔥 Real-time thermal simulation
- 🔌 Modbus TCP client for ICS integration
- 🚀 Zero-copy physics updates
- 🎮 Native Godot nodes for Modelica components

## Installation

### As a Git Submodule
```bash
cd your-godot-project
git submodule add https://github.com/bondlegend4/modelica-rust-gdext.git addons/modelica
cd addons/modelica
git submodule update --init --recursive
cargo build --release
```

### From Source
```bash
git clone https://github.com/bondlegend4/modelica-rust-gdext.git
cd modelica-rust-gdext
git submodule update --init --recursive
cargo build --release
```

## Usage

### In Your Godot Project

1. Copy `target/release/libmodelica_rust_gdext.*` to `res://addons/modelica/bin/`
2. Create `res://addons/modelica/modelica.gdextension`:
```ini
[configuration]
entry_symbol = "gdext_rust_init"

[libraries]
linux.debug.x86_64 = "res://addons/modelica/bin/libmodelica_rust_gdext.so"
linux.release.x86_64 = "res://addons/modelica/bin/libmodelica_rust_gdext.so"
```

3. Use in your scenes:
```gdscript
extends Node3D

@onready var thermal_sim = $ModelicaThermalNode

func _ready():
    thermal_sim.set_heater(true)

func _process(delta):
    print("Temperature: ", thermal_sim.get_temperature())
```

## Requirements

- Rust 1.87+
- Godot 4.2+
- OpenModelica (for building modelica-rust-ffi)

## Examples

See `examples/thermal_demo/` for a complete working example.

## License

MIT