use crate::env::*;
use log::*;
use wasmer_runtime::{Array, Ctx, WasmPtr};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[repr(u32)]
#[derive(Debug, FromPrimitive)]
pub enum LogLevel {
    Error = 1,
    Warning = 3,
    Info = 6,
}

pub fn log_write(ctx: &mut Ctx, level: u32, ptr: WasmPtr<u8, Array>, len: u32) {
  let (memory, env) = OlinEnv::get_memory_and_environment(ctx, 0);
  let string = ptr.get_utf8_string(memory, len).unwrap();
    match FromPrimitive::from_u32(level) {
        Some(LogLevel::Info) => info!("{}: {}", env.host_name, string),
        Some(LogLevel::Warning) => warn!("{}: {}", env.host_name, string),
        Some(LogLevel::Error) => error!("{}: {}", env.host_name, string),
        None => { panic!("invalid log level {}", level); }
    }

    env.log_call("log_write".to_string());
}

pub fn runtime_exit(ctx: &mut Ctx, code: u32) {
    let (_, env) = OlinEnv::get_memory_and_environment(ctx, 0);
    env.log_call("runtime_exit".to_string());
}
