use att_uniter::AttUniter;
pub use config::Config;
use config::LiangOption;
use config::YishiOption;
use util::is_liang;
use util::is_yishi;
use util::{FU, LIANG, NUMS, PRE_UNITS};

pub mod att_uniter;
pub mod config;
mod util;

pub fn sinonumify<U: AttUniter>(num_str: &str, config: Config) -> String {
    let negative = num_str.starts_with("-");
    let mut res = sinonum_impl::<U, String>(
        &num_str.trim()[(negative as usize)..],
        config.liang != LiangOption::Disable,
        config.yishi,
    );
    if res.is_empty() {
        res = String::from(NUMS[0]);
    }
    if negative {
        res.insert(0, FU);
    }
    if config.liang == LiangOption::JustNumber && res.ends_with(LIANG) {
        res.pop();
        res.push_str(NUMS[2]);
    }
    res
}

pub fn sinonum_impl<U: AttUniter, T: FromIterator<&'static str>>(
    num_str: &str,
    enable_liang: bool,
    yishi: YishiOption,
) -> T {
    let init_phase = 4 - (num_str.len() % 4);
    let first_att_unit_power = num_str.len().next_multiple_of(4) / 4 - 1;
    let_flag!(a, had_zero_ptr, false);
    let_flag!(b, had_part_ptr, false);
    let_flag!(c, last_unit_num_ptr, usize::MAX);
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
                    res.push(match n {
                        1 if is_yishi(
                            yishi,
                            pre_unit_place,
                            *had_zero_ptr,
                            *last_unit_num_ptr == usize::MAX,
                        ) =>
                        {
                            ""
                        }
                        2 if is_liang(enable_liang, pre_unit_place, *last_unit_num_ptr > 1) => {
                            LIANG
                        }
                        n => NUMS[n as usize],
                    });
                    res.push(PRE_UNITS[pre_unit_place]);
                    *had_zero_ptr = false;
                    *last_unit_num_ptr = (pre_unit_place != 3) as usize;
                    res
                }
            }
            Err(place) => {
                let att_unit = U::get_att_unit(place);
                *last_unit_num_ptr += att_unit.len();
                att_unit
            }
        }) // 翻译
        .collect()
}
