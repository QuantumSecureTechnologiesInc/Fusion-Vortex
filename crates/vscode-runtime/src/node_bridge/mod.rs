//! Enhanced Node.js compatibility layer with module system

pub mod buffer;
pub mod events;
pub mod fs;
pub mod modules;
pub mod path;
pub mod process;
pub mod stream;

use anyhow::Result;
// use boa_engine::gc::{Finalize, Trace, Tracer};
// use boa_engine::object::{FunctionObjectBuilder, NativeObject};
use boa_engine::{Context, JsValue, Source};
// use boa_engine::property::Attribute;
// use boa_engine::gc::{Finalize, Trace, Tracer};
use fusion_policy::{Capability, PolicyEnforcer};
use parking_lot::RwLock;
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

/// Enhanced Node.js runtime bridge
pub struct NodeRuntime {
    context: Arc<RwLock<Context>>,
    module_loader: modules::ModuleLoader,
    enforcer: Option<Arc<RwLock<PolicyEnforcer>>>,
    capabilities: HashSet<Capability>,
}

impl NodeRuntime {
    pub fn new() -> Result<Self> {
        let mut context = Context::default();

        // Initialize all Node.js core modules
        Self::setup_globals(&mut context)?;
        Self::setup_core_modules(&mut context)?;

        Ok(Self {
            context: Arc::new(RwLock::new(context)),
            module_loader: modules::ModuleLoader::new(),
            enforcer: None,
            capabilities: HashSet::new(),
        })
    }

    /// Configure security policy for this runtime
    pub fn configure_security(
        &mut self,
        enforcer: Arc<RwLock<PolicyEnforcer>>,
        capabilities: Vec<Capability>,
    ) -> Result<()> {
        self.enforcer = Some(enforcer);
        self.capabilities = capabilities.into_iter().collect();
        self.setup_fs_bridge()?;
        Ok(())
    }

    /// Set up filesystem bridge with policy enforcement
    fn setup_fs_bridge(&self) -> Result<()> {
        let mut _context = self.context.write();

        // This is where we would normally register the native functions like __rust_fs_read
        // Since we are mocking the Node.js API structure in fs.rs without actual native bindings in this version,
        // we will implement checking logic that would be used if the bindings existed.

        if let Some(enforcer) = &self.enforcer {
            let _enforcer_guard = enforcer.read();
            let mut capabilities = std::collections::HashSet::new();
            capabilities.extend(self.capabilities.clone());
            let _allowed: Vec<Capability> = capabilities.into_iter().collect();

            // In a full implementation, we would register the host functions here:
            // register_native_fn(&mut context, "__rust_fs_read", move |args| {
            //      enforcer_guard.enforce(Capability::FilesystemRead, &allowed)?;
            //      // ... perform read
            // });
        }

        Ok(())
    }

    /// Set up Node.js global objects
    fn setup_globals(context: &mut Context) -> Result<()> {
        // Global object
        let global_obj = r#"
            var global = this;
            var GLOBAL = this;
            var root = this;
        "#;
        context
            .eval(Source::from_bytes(global_obj))
            .map_err(|e| anyhow::anyhow!("JS Error: {:?}", e))?;

        // Console
        // Self::setup_console(context)?;

        // Process
        Self::setup_process(context)?;

        // Buffer
        Self::setup_buffer(context)?;

        // Timers
        Self::setup_timers(context)?;

        Ok(())
    }

    /// Set up console object with proper methods
    #[allow(dead_code)]
    fn setup_console(_context: &mut Context) -> Result<()> {
        // Disabled due to Boa 0.19 API mismatch
        Ok(())
    }

    /// Set up process object
    fn setup_process(context: &mut Context) -> Result<()> {
        let process_code = r#"
            var process = {
                version: 'v18.0.0',
                versions: {
                    node: '18.0.0',
                    v8: '10.0.0'
                },
                platform: 'fusion',
                arch: 'x64',
                env: {},
                argv: ['node', 'extension.js'],
                cwd: function() { return '/'; },
                exit: function(code) { 
                    console.log('Process exit:', code); 
                },
                nextTick: function(callback) {
                    setTimeout(callback, 0);
                },
                on: function(event, handler) {
                    // Event listener stub
                },
                emit: function(event, ...args) {
                    // Event emitter stub
                }
            };
        "#;
        context
            .eval(Source::from_bytes(process_code))
            .map_err(|e| anyhow::anyhow!("JS Error: {:?}", e))?;
        Ok(())
    }

    /// Set up Buffer implementation
    fn setup_buffer(context: &mut Context) -> Result<()> {
        let buffer_code = r#"
            class Buffer extends Uint8Array {
                static from(data, encoding) {
                    if (typeof data === 'string') {
                        return new TextEncoder().encode(data);
                    }
                    return new Uint8Array(data);
                }
                
                static alloc(size) {
                    return new Uint8Array(size);
                }
                
                toString(encoding) {
                    return new TextDecoder(encoding || 'utf8').decode(this);
                }
            }
            
            global.Buffer = Buffer;
        "#;
        context
            .eval(Source::from_bytes(buffer_code))
            .map_err(|e| anyhow::anyhow!("JS Error: {:?}", e))?;
        Ok(())
    }

    /// Set up timer functions
    fn setup_timers(context: &mut Context) -> Result<()> {
        let timers_code = r#"
             var timers = {};
             var timerIdCounter = 1;
             
             function setTimeout(callback, delay, ...args) {
                 var id = timerIdCounter++;
                 timers[id] = { callback, args, delay, type: 'timeout' };
                 return id;
             }
             
             function clearTimeout(id) {
                 delete timers[id];
             }
             
             function setInterval(callback, delay, ...args) {
                 var id = timerIdCounter++;
                 timers[id] = { callback, args, delay, type: 'interval' };
                 return id;
             }
             
             function clearInterval(id) {
                 delete timers[id];
             }
             
             function setImmediate(callback, ...args) {
                 return setTimeout(callback, 0, ...args);
             }
             
             function clearImmediate(id) {
                 clearTimeout(id);
             }
             
             global.setTimeout = setTimeout;
             global.clearTimeout = clearTimeout;
             global.setInterval = setInterval;
             global.clearInterval = clearInterval;
             global.setImmediate = setImmediate;
             global.clearImmediate = clearImmediate;
        "#;
        context
            .eval(Source::from_bytes(timers_code))
            .map_err(|e| anyhow::anyhow!("JS Error: {:?}", e))?;
        Ok(())
    }

    /// Set up core Node.js modules
    fn setup_core_modules(context: &mut Context) -> Result<()> {
        // These will be lazy-loaded when required
        let module_stubs = r#"
            var _nodeModules = {
                'fs': null,
                'path': null,
                'events': null,
                'stream': null,
                'util': null,
                'os': null,
                'crypto': null,
                'child_process': null,
            };
        "#;
        context
            .eval(Source::from_bytes(module_stubs))
            .map_err(|e| anyhow::anyhow!("JS Error: {:?}", e))?;
        Ok(())
    }

    /// Execute JavaScript code
    pub fn execute(&self, code: &str) -> Result<JsValue> {
        let mut context = self.context.write();
        let result = context
            .eval(Source::from_bytes(code))
            .map_err(|e| anyhow::anyhow!("JS Error: {:?}", e))?;
        Ok(result)
    }

    /// Load and execute a module
    pub fn require(&self, module_path: &str) -> Result<JsValue> {
        // Check if core module
        if module_path.starts_with("node:")
            || !module_path.contains('/') && !module_path.contains('\\')
        {
            return self.load_core_module(module_path);
        }

        // Load from file system
        self.module_loader.load_module(module_path, &self.context)
    }

    /// Load a core Node.js module
    fn load_core_module(&self, name: &str) -> Result<JsValue> {
        let module_name = name.strip_prefix("node:").unwrap_or(name);

        let module_code = match module_name {
            "fs" | "fs/promises" => fs::get_fs_module(),
            "path" => path::get_path_module(),
            "events" => events::get_events_module(),
            "stream" => stream::get_stream_module(),
            "util" => Self::get_util_module(),
            "os" => Self::get_os_module(),
            _ => return Err(anyhow::anyhow!("Unknown core module: {}", module_name)),
        };

        self.execute(&module_code)
    }

    fn get_util_module() -> String {
        r#"
        (function() {
            return {
                promisify: function(fn) {
                    return function(...args) {
                        return new Promise((resolve, reject) => {
                            fn(...args, (err, result) => {
                                if (err) reject(err);
                                else resolve(result);
                            });
                        });
                    };
                },
                inspect: function(obj) {
                    return JSON.stringify(obj);
                }
            };
        })()
        "#
        .to_string()
    }

    fn get_os_module() -> String {
        r#"
        (function() {
            return {
                platform: function() { return process.platform; },
                arch: function() { return process.arch; },
                homedir: function() { return process.env.HOME || '/'; },
                tmpdir: function() { return '/tmp'; },
                hostname: function() { return 'fusion-host'; },
                cpus: function() { return []; },
                freemem: function() { return 0; },
                totalmem: function() { return 0; },
            };
        })()
        "#
        .to_string()
    }

    /// Load JavaScript module from file
    pub async fn load_module_from_file(&self, path: &Path) -> Result<JsValue> {
        let code = tokio::fs::read_to_string(path).await?;
        self.execute(&code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        let runtime = NodeRuntime::new();
        assert!(runtime.is_ok());
    }

    #[test]
    fn test_console_log() {
        let runtime = NodeRuntime::new().unwrap();
        let result = runtime.execute("console.log('test'); 'ok'");
        assert!(result.is_ok());
    }

    #[test]
    fn test_buffer() {
        let runtime = NodeRuntime::new().unwrap();
        let result = runtime.execute("Buffer.from('hello').toString()");
        assert!(result.is_ok());
    }
}
