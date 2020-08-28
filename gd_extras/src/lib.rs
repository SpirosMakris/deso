pub mod input;
pub mod node_ext;

#[macro_export]
macro_rules! gdp {
    ($($arg:tt)*) => ({
        let line = std::line!();
        let file = std::file!();
        let val: String = format!($($arg)*);
        gdnative::godot_print!("{}:{} {}", file, line, val);
    });
}

#[macro_export]
macro_rules! gd_err {
    ($($arg:tt)*) => ({
        let line = std::line!();
        let file = std::file!();
        let val: String = format!($($arg)*);
        gdnative::godot_error!("{}:{} {}", file, line, val);
    });
}
