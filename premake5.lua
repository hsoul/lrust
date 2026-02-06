---@diagnostic disable: undefined-global
local LOCAL_DIR = _SCRIPT_DIR
if os.host() == "windows" then
    LOCAL_DIR = path.translate(LOCAL_DIR)
end

-- Helper function to get platform-specific shared library name
function get_sharedlib_name(name)
    local prefix = ""
    local ext = ""
    
    if os.host() == "windows" then
        ext = ".dll"
    elseif os.host() == "macosx" then
        prefix = "lib"
        ext = ".dylib"
    else
        -- Linux and other Unix-like systems
        prefix = "lib"
        ext = ".so"
    end
    
    -- Remove "lib" prefix if already present in the name
    if name:sub(1, 3) == "lib" then
        name = name:sub(4)
    end
    
    return prefix .. name .. ext
end

workspace "lrust"
    configurations { "debug", "release" }

project "lrust"
    kind "Makefile"

    filter "configurations:release"
        buildcommands {
            -- cargo build --no-default-features --features="http,websocket"
            string.format("{CHDIR} %s && cargo build --release", LOCAL_DIR),
            -- ltask repo layout: put the cdylib under ltask root so package.cpath './?.so' can find it.
            string.format("{COPYFILE} %s/target/release/%s %s ", LOCAL_DIR, get_sharedlib_name("librust"), path.join(LOCAL_DIR, "../../luaclib", "rust.so")),
            -- copy lua helpers to game-server lualib (same level as librust.so)
            string.format("{COPY} %s/lualib/* %s ", LOCAL_DIR, path.join(LOCAL_DIR, "../..", "ext_lualib")),
        }

        cleancommands {
            string.format("{CHDIR} %s && cargo clean --release", LOCAL_DIR),
        }

    filter "configurations:debug"
        buildcommands {
            -- cargo build --no-default-features --features="http,websocket"
            string.format("{CHDIR} %s && cargo build", LOCAL_DIR),
            string.format("{COPYFILE} %s/target/debug/%s %s ", LOCAL_DIR, get_sharedlib_name("librust"), path.join(LOCAL_DIR, ".../../luaclib", "rust.so")),
            string.format("{COPY} %s/lualib/* %s ", LOCAL_DIR, path.join(LOCAL_DIR, "../..", "ext_lualib")),
        }

        cleancommands {
            string.format("{CHDIR} %s && cargo clean", LOCAL_DIR),
        }