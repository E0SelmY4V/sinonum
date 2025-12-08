use std::{iter::repeat_n, usize};

const NUMS: [&str; 10] = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
const LIANG: &str = "两";
const FU: char = '负';
const PRE_UNITS: [&str; 4] = ["千", "百", "十", ""];
const ATT_UNITS: [&str; 18] = [
    "万",
    "亿",
    "兆",
    "京",
    "垓",
    "杼",
    "穰",
    "沟",
    "涧",
    "正",
    "载",
    "极",
    "恒河沙",
    "阿僧口",
    "那由它",
    "不可思议",
    "无量",
    "大数",
];

macro_rules! let_flag {
    ($name: ident, $name_ptr: ident, $value: expr) => {
        let mut $name = $value;
        let $name_ptr = &mut $name;
    };
}

pub mod get_att_unit {
    use crate::*;
    pub(super) const NONTOPER_SIZE: usize = ATT_UNITS.len() - 1;
    pub(super) const NONTOPER_MASK: usize =
        usize::MAX >> (0_usize.count_zeros() as usize - NONTOPER_SIZE);
}

/// 得到位列第 `place` 位的大单位
///
/// ## Example
///
/// ```rust
/// use sinonum::get_att_unit;
///
/// assert_eq!("万", get_att_unit(1).join(""));
/// assert_eq!("亿", get_att_unit(2).join(""));
/// assert_eq!("万兆", get_att_unit(5).join(""));
/// assert_eq!("万亿兆京垓杼", get_att_unit(63).join(""));
/// assert_eq!(
///     "兆京垓涧正载大数大数大数大数大数大数大数大数",
///     get_att_unit(919324).join(""),
/// );
/// ```
pub fn get_att_unit(place: usize) -> Vec<&'static str> {
    let toper_number = place >> get_att_unit::NONTOPER_SIZE;
    let nontoper_number = (get_att_unit::NONTOPER_MASK & place).count_ones() as usize;
    let mut result = Vec::with_capacity(toper_number + nontoper_number);
    let_flag!(tester, tester_ptr, 1_usize);
    result.extend(
        ATT_UNITS
            .into_iter()
            .filter_map(|att_unit| {
                let filted = (place & *tester_ptr != 0).then_some(att_unit);
                *tester_ptr <<= 1;
                filted
            })
            .chain(repeat_n(
                *ATT_UNITS.last().expect("一个大单位都没有"),
                toper_number,
            )),
    );
    result
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LiangOption {
    /// 禁用两
    Disable,
    /// 末尾带单位，即个位可以是两
    WithUnit,
    /// 纯数字表示，个位不能用两
    JustNumber,
}

pub fn sinonumify(num_str: &str, liang_option: LiangOption) -> String {
    if num_str.is_empty() {
        return NUMS[0].to_string();
    }
    let negative = num_str.starts_with("-");
    let mut res: String = sinonum_impl(
        &num_str.trim()[(negative as usize)..],
        liang_option != LiangOption::Disable,
    );
    if negative {
        res.insert(0, FU);
    }
    if liang_option == LiangOption::JustNumber && res.ends_with(LIANG) {
        res.pop();
        res.push_str(NUMS[2]);
    }
    res
}

pub fn sinonum_impl<T: FromIterator<&'static str>>(num_str: &str, enable_liang: bool) -> T {
    let init_phase = 4 - (num_str.len() % 4);
    let first_att_unit_power = num_str.len().next_multiple_of(4) / 4 - 1;
    let_flag!(a, had_zero_ptr, false);
    let_flag!(b, had_part_ptr, false);
    let_flag!(c, last_unit_num_ptr, 5);
    num_str
        .chars()
        .into_iter()
        .filter_map(|num_str| {
            u8::try_from(num_str)
                .ok()
                .and_then(|n| n.checked_sub('0' as u8))
                .filter(|&n| n <= 9)
        }) // 变成 u8
        .zip(
            (0..)
                .map(|i| first_att_unit_power - i)
                .flat_map(|n| [n; 4])
                .zip((init_phase..).map(|n| n % 4)),
        ) // 带上位信息
        .flat_map(|(num_char, (att_unit_place, phase))| {
            [Ok((num_char, phase)), Err((phase, att_unit_place))]
        }) // 变成流
        .filter_map(|block| match block {
            Ok(t) => Some(Ok(t)),
            Err((phase, att_unit_place)) if phase == 3 => Some(Err(att_unit_place)),
            _ => None,
        }) // 每 4 位数，一个大单位标记
        .filter_map(|block| {
            if let Ok((n, _)) = block {
                *had_part_ptr |= n != 0;
            } else {
                if *had_part_ptr {
                    *had_part_ptr = false;
                } else {
                    return None;
                }
            }
            Some(block)
        }) // 合并大单位
        .flat_map(|block| match block {
            Ok((n, pre_unit_place)) => {
                if n == 0 {
                    *had_zero_ptr = true;
                    Vec::new()
                } else {
                    let mut res = Vec::with_capacity(2 + *had_zero_ptr as usize);
                    if *had_zero_ptr {
                        res.push(NUMS[0]);
                    }
                    *had_zero_ptr = false;
                    res.push(
                        if is_liang(enable_liang, n, pre_unit_place, *last_unit_num_ptr > 1) {
                            LIANG
                        } else {
                            NUMS[n as usize]
                        },
                    );
                    res.push(PRE_UNITS[pre_unit_place]);
                    *last_unit_num_ptr = (pre_unit_place != 3) as usize;
                    res
                }
            }
            Err(place) => {
                let att_unit = get_att_unit(place);
                *last_unit_num_ptr += att_unit.len();
                att_unit
            }
        }) // 翻译
        .collect()
}

/// 判断当前位是不是用两来代替二
///
/// ## 标准
///
/// - 启用两
/// - 当前数是 2
/// - 当前小位不是十位（如十万，十兆亿）
/// - 当前小位是个位时，前面要么为空，要么为 `<一个字数大于等于 2 的单位>零`
fn is_liang(enable_liang: bool, n: u8, pre_unit_place: usize, only_liang: bool) -> bool {
    enable_liang && n == 2 && pre_unit_place != 2 && (pre_unit_place != 3 || only_liang)
}
