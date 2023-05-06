#[cfg(debug_assertions)]
pub const STATIC_BASE_PATH: &str = "REPLACE_THIS_STRING!";

#[cfg(not(debug_assertions))]
pub const STATIC_BASE_PATH: &str = env!("DIST_PATH");
