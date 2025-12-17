pub const NUMS: [&str; 10] = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
pub const LIANG: &str = "两";
pub const FU: char = '负';
pub const PRE_UNITS: [&str; 4] = ["千", "百", "十", ""];

#[macro_export]
macro_rules! let_flag {
    ($name: ident, $name_ptr: ident, $value: expr) => {
        let mut $name = $value;
        let $name_ptr = &mut $name;
    };
}
