pub use platform::*;

#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
mod platform;

#[cfg(all(not(target_os = "macos")))]
compile_error!("The platform you're compiling for is not supported");
