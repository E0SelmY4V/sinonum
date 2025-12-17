use crate::config::YishiOption;

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

/// 判断当前位是不是用两来代替二
///
/// ## 标准
///
/// - 启用两
/// - 当前小位不是十位（如十万，十兆亿）
/// - 当前小位是个位时，前面要么为空，要么为 `<一个字数大于等于 2 的单位>零`
pub fn is_liang(enable_liang: bool, pre_unit_place: usize, only_liang: bool) -> bool {
    enable_liang && pre_unit_place != 2 && (pre_unit_place != 3 || only_liang)
}
/// 判断当前的一十需不需要去掉一
///
/// ## 标准
///
/// - 小单位是十
/// - 当尽量避免一十时，最头上和零之后紧跟的一十会被省略
/// - 或者仅限头部
pub fn is_yishi(yishi: YishiOption, pre_unit_place: usize, had_zero: bool, is_head: bool) -> bool {
    pre_unit_place == 2
        && ((yishi == YishiOption::Avoid && (had_zero || is_head))
            || (yishi == YishiOption::ExceptHead && is_head))
}
