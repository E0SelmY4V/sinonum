use std::{iter::repeat_n, ops::Deref, usize};

const NUMS: [&str; 10] = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
const LIANG: &str = "两";
const FU: &str = "负";
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

pub fn sinonumify(num_str: &str) -> String {
    if num_str.is_empty() {
        return NUMS[0].to_string();
    }
    let negative = num_str.starts_with("-");
    let mut res = if negative { vec![FU] } else { vec![] };
    res.append(&mut sinonum_impl(&num_str[(negative as usize)..]));
    res.join("")
}

pub fn sinonum_impl(num_str: &str) -> Vec<&str> {
    let init_phase = (4 - (num_str.len() % 4)) * 2;
    let first_att_unit_power = num_str.len().next_multiple_of(4) / 4 - 1;
    let_flag!(a, had_zero_ptr, false);
    let_flag!(b, had_part_ptr, false);
    (0..2 * num_str.len())
        .zip(init_phase..)
        .filter_map(|(index, phase)| {
            if phase % 8 == 7 {
                let place = first_att_unit_power - index / 8;
                Some(Err(place))
            } else if phase % 2 == 0 {
                let index = index / 2;
                let pre_unit_place = (phase % 8) / 2;
                Some(Ok((index, pre_unit_place)))
            } else {
                None
            }
        }) // 按照相位得到单位
        .map(|r| {
            r.map(|(n, f)| {
                (
                    (num_str
                        .get(n..=n)
                        .unwrap_or_else(|| panic!("拿不到第 {n} 个字符"))
                        .chars()
                        .next()
                        .unwrap_or_else(|| panic!("拿不到！")) as u8)
                        .checked_sub('0' as u8)
                        .filter(|&n| n <= 9)
                        .unwrap_or_default(),
                    f,
                )
            })
        }) // Ok((数, 小单位)) Err(大单位)
        .filter_map(|now| {
            if let Ok((n, _n)) = now {
                *had_part_ptr |= n != 0;
            } else {
                if *had_part_ptr {
                    *had_part_ptr = false;
                } else {
                    return None;
                }
            }
            Some(now)
        }) // 合并大单位
        .flat_map(|r| match r {
            Ok((n, f)) => {
                if n == 0 {
                    *had_zero_ptr = true;
                    Vec::new()
                } else {
                    let mut res = Vec::with_capacity(2 + *had_zero_ptr as usize);
                    if *had_zero_ptr {
                        res.push(NUMS[0]);
                    }
                    *had_zero_ptr = false;
                    res.push(NUMS[n as usize]);
                    res.push(PRE_UNITS[f]);
                    res
                }
            }
            Err(place) => get_att_unit(place),
        }) // 翻译
        .collect()
}

/*

没有两十
除非是2，否则两不能在最低的个位

 */
