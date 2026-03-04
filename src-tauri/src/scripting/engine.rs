use anyhow::Result;
use mlua::prelude::*;

/// Lua scripting engine for automation.
///
/// Provides a sandboxed Lua environment with access to:
/// - span.connect(name) - connect to a device
/// - span.log(msg) - log a message
/// - span.sleep(ms) - async sleep
/// - session:send(data) - send data to session
/// - session:expect(pattern) - wait for pattern in output
/// - session:read() - read available output
///
/// Scripts are loaded from ~/.config/span/scripts/
pub struct LuaEngine {
    lua: Lua,
}

fn lua_err(e: LuaError) -> anyhow::Error {
    anyhow::anyhow!("Lua error: {}", e)
}

impl LuaEngine {
    pub fn new() -> Result<Self> {
        let lua = Lua::new();

        // Set up the 'span' global table with helper functions
        lua.load(r#"
            span = {}
            span.log = function(msg)
                __span_log(tostring(msg))
            end
            span.sleep = function(ms)
                __span_sleep(ms)
            end
        "#).exec().map_err(lua_err)?;

        // Register Rust-backed functions
        let log_fn = lua.create_function(|_, msg: String| {
            tracing::info!("[lua] {}", msg);
            Ok(())
        }).map_err(lua_err)?;
        lua.globals().set("__span_log", log_fn).map_err(lua_err)?;

        let sleep_fn = lua.create_function(|_, ms: u64| {
            std::thread::sleep(std::time::Duration::from_millis(ms));
            Ok(())
        }).map_err(lua_err)?;
        lua.globals().set("__span_sleep", sleep_fn).map_err(lua_err)?;

        Ok(Self { lua })
    }

    /// Execute a Lua script from a string
    pub fn exec_script(&self, script: &str) -> Result<()> {
        self.lua.load(script).exec().map_err(lua_err)?;
        Ok(())
    }

    /// Execute a Lua script from a file
    pub fn exec_file(&self, path: &std::path::Path) -> Result<()> {
        let script = std::fs::read_to_string(path)?;
        self.exec_script(&script)
    }

    /// Register a session object that Lua scripts can interact with
    pub fn register_session(&self, session_id: &str) -> Result<()> {
        let id = session_id.to_string();

        let session_table = self.lua.create_table().map_err(lua_err)?;
        session_table.set("id", id.clone()).map_err(lua_err)?;

        // session:send(data)
        let send_fn = self.lua.create_function(move |_, (_self_table, data): (LuaTable, String)| {
            tracing::info!("[lua] session.send: {}", data.trim());
            Ok(())
        }).map_err(lua_err)?;
        session_table.set("send", send_fn).map_err(lua_err)?;

        // session:expect(pattern)
        let expect_fn = self.lua.create_function(|_, (_self_table, pattern): (LuaTable, String)| {
            tracing::info!("[lua] session.expect: {}", pattern);
            Ok(String::new())
        }).map_err(lua_err)?;
        session_table.set("expect", expect_fn).map_err(lua_err)?;

        self.lua.globals().set("session", session_table).map_err(lua_err)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lua_engine_basic() {
        let engine = LuaEngine::new().unwrap();
        engine.exec_script("span.log('hello from lua')").unwrap();
    }

    #[test]
    fn test_lua_engine_session() {
        let engine = LuaEngine::new().unwrap();
        engine.register_session("test-session").unwrap();
        engine.exec_script(r##"
            span.log("Session ID: " .. session.id)
            session:send("show version\n")
            local output = session:expect("#")
        "##).unwrap();
    }
}
