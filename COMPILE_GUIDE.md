# Compilation and Usage Guide

## Prerequisites

1. **Rust Environment**: Install Rust from [https://www.rust-lang.org/](https://www.rust-lang.org/)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Premake5**: Install Premake5 (if using premake build system)
   - Download from [https://premake.github.io/](https://premake.github.io/)
   - Or use package manager: `sudo apt-get install premake5` (Linux)

## Compilation Methods

### Method 1: Using Cargo Directly (Recommended)

This is the simplest way to compile the project:

#### Debug Build
```bash
cd /home/game/open-source/game-server/ltask/3rd/lrust
cargo build
```

The compiled library will be at:
- Linux: `target/debug/librust.so`
- Windows: `target/debug/rust.dll`
- macOS: `target/debug/librust.dylib`

#### Release Build
```bash
cargo build --release
```

The compiled library will be at:
- Linux: `target/release/librust.so`
- Windows: `target/release/rust.dll`
- macOS: `target/release/librust.dylib`

#### Build with Specific Features
```bash
# Build with only specific features
cargo build --release --no-default-features --features="http,websocket"

# Build with all default features (default)
cargo build --release
```

### Method 2: Using Premake5

If you're integrating with a premake-based build system:

```bash
cd /home/game/open-source/game-server/ltask/3rd/lrust
premake5 gmake2  # Generate Makefiles
make config=release  # Build release version
# or
make config=debug    # Build debug version
```

Or if using Visual Studio:
```bash
premake5 vs2022  # Generate Visual Studio 2022 project
```

## Installation

After compilation, you need to:

1. **Copy the shared library** to where your Lua runtime can find it:
   ```bash
   # For Linux
   cp target/release/librust.so /path/to/your/lua/runtime/
   
   # For Windows
   cp target/release/rust.dll /path/to/your/lua/runtime/
   
   # For macOS
   cp target/release/librust.dylib /path/to/your/lua/runtime/
   ```

2. **Copy Lua helper files** from `lualib/` directory:
   ```bash
   cp lualib/*.lua /path/to/your/lua/runtime/lualib/
   ```

## Usage in Lua

### Loading the Library

The library provides various modules that can be loaded in Lua:

```lua
-- HTTP Client
local httpc = require("ext.httpc")  -- or require("rust.httpc")

-- SQL Database (SQLx)
local sqlx = require("ext.sqlx")    -- or require("rust.sqlx")

-- MongoDB
local mongodb = require("ext.mongodb")  -- or require("rust.mongodb")

-- Excel Reader
local excel = require("rust.excel")

-- Crypto
local crypto = require("rust.crypto")

-- WebSocket
local websocket = require("ext.websocket")  -- or require("rust.websocket")
```

### Example: HTTP Client

```lua
local httpc = require("ext.httpc")
local moon = require("moon")

moon.async(function()
    local response = httpc.get("https://bing.com")
    print(response.status_code == 200)
end)
```

### Example: SQLx Database

```lua
local moon = require("moon")
local sqlx = require("ext.sqlx")

moon.async(function()
    local db = sqlx.connect("postgres://user:pass@localhost/dbname", "connection_name")
    if db.kind then
        print("connect failed", db.message)
        return
    end
    
    local result = db:query("SELECT * FROM users WHERE id = $1", 123)
    print_r(result)
    
    -- Wait for all queries to complete before shutdown
    while true do
        local done = true
        local stats = sqlx.stats()
        for k, v in pairs(stats) do
            if v > 0 then
                done = false
                break
            end
        end
        if done then
            break
        end
        moon.sleep(100)
    end
end)
```

## Available Features

The library supports optional features that can be enabled/disabled:

- `excel`: Excel file reading
- `sqlx`: SQL database support (MySQL, PostgreSQL, SQLite)
- `mongodb`: MongoDB support
- `websocket`: WebSocket client
- `http`: HTTP client
- `json`: JSON processing
- `tiberius`: SQL Server support

Default features: `["excel", "sqlx", "mongodb", "websocket", "http", "json"]`

To enable SQL Server support, add `tiberius` to the default features in `crates/libs/lib-lualib/Cargo.toml`:

```toml
[features]
default = ["excel", "sqlx", "mongodb", "websocket", "http", "json", "tiberius"]
```

## Troubleshooting

### Common Issues

1. **Library not found**: Make sure the compiled library is in a directory that's in your Lua `package.cpath`
2. **Missing dependencies**: Some features require system libraries (e.g., OpenSSL for HTTPS)
3. **Build errors**: Check that you have the correct Rust toolchain installed: `rustc --version`

### Clean Build

If you encounter build issues, try a clean build:

```bash
cargo clean
cargo build --release
```

## Testing

Test scripts are available in the `test/` directory:

```bash
# Run a test script (example)
lua test/test_mysql_types.lua
```
