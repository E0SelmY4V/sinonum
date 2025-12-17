use crate::let_flag;
use std::{iter::repeat_n, usize};

pub trait AttUniter {
    const SIZE: usize;
    const NONTOPER_SIZE: usize = Self::SIZE - 1;
    const NONTOPER_MASK: usize =
        usize::MAX >> (0_usize.count_zeros() as usize - Self::NONTOPER_SIZE);
    /// 得到位列第 `place` 位的大单位
    fn get_att_unit(place: usize) -> Vec<&'static str>;
}
macro_rules! new_att_units {
    (
        $(
            #[doc = $doc:expr]
        )*
        $name: ident,
        $num: literal,
        $units: expr
    ) => {
        pub struct $name;
        impl $name {
            const UNITS: [&str; $num] = $units;
        }
        impl AttUniter for $name {
            const SIZE: usize = $num;
            $(
                #[doc = $doc]
            )*
            fn get_att_unit(place: usize) -> Vec<&'static str> {
                let toper_number = place >> Self::NONTOPER_SIZE;
                let nontoper_place = Self::NONTOPER_MASK & place;
                let nontoper_number = nontoper_place.count_ones() as usize;
                let mut result: Vec<&str> = Vec::with_capacity(toper_number + nontoper_number);
                let_flag!(tester, tester_ptr, 1_usize);
                result.extend(
                    Self::UNITS
                        .into_iter()
                        .filter_map(|att_unit| {
                            let filted = (nontoper_place & *tester_ptr != 0).then_some(att_unit);
                            *tester_ptr <<= 1;
                            filted
                        })
                        .chain(repeat_n(
                            *Self::UNITS.last().expect("一个大单位都没有"),
                            toper_number,
                        )),
                );
                result
            }
        }
    };
}
new_att_units!(
    /// ```rust
    /// use sinonum::att_uniter::{OldAttUnits, AttUniter};
    ///
    /// assert_eq!("万", OldAttUnits::get_att_unit(1).join(""));
    /// assert_eq!("亿", OldAttUnits::get_att_unit(2).join(""));
    /// assert_eq!("万兆", OldAttUnits::get_att_unit(5).join(""));
    /// assert_eq!("万亿兆京垓杼", OldAttUnits::get_att_unit(63).join(""));
    /// assert_eq!(
    ///     "兆京垓涧正载大数大数大数大数大数大数大数",
    ///     OldAttUnits::get_att_unit(919324).join(""),
    /// );
    /// ```
    OldAttUnits,
    18,
    [
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
    ]
);
new_att_units!(
    /// ```rust
    /// use sinonum::att_uniter::{StdAttUnits, AttUniter};
    ///
    /// assert_eq!("万", StdAttUnits::get_att_unit(1).join(""));
    /// assert_eq!("亿", StdAttUnits::get_att_unit(2).join(""));
    /// assert_eq!("万亿", StdAttUnits::get_att_unit(3).join(""));
    /// assert_eq!("万亿亿亿", StdAttUnits::get_att_unit(7).join(""));
    /// assert_eq!("万亿亿亿亿", StdAttUnits::get_att_unit(9).join(""));
    /// ```
    StdAttUnits,
    2,
    ["万", "亿"]
);
