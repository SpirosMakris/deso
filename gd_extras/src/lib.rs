pub mod input;


#[macro_export]
macro_rules! gdp {
    ($($arg:tt)*) => ({
        let line = std::line!();
        let file = std::file!();
        let val: String = format!($($arg)*);
        gdnative::godot_print!("{}:{} {}", file, line, val);
    });
}