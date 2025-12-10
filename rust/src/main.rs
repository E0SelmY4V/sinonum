#![windows_subsystem = "windows"]

use std::io::{self, Write};

use clap::{Parser, ValueEnum};
use encoding::{EncoderTrap, label::encoding_from_whatwg_label};
use sinonum::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum LiangOptionCmd {
    /// 禁用两
    Disable,
    /// 末尾带单位，即个位可以是两
    WithUnit,
    /// 纯数字表示，即个位不能用两
    JustNumber,
}
impl LiangOptionCmd {
    pub fn to(self) -> LiangOption {
        match self {
            Self::Disable => LiangOption::Disable,
            Self::WithUnit => LiangOption::WithUnit,
            Self::JustNumber => LiangOption::JustNumber,
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum AttMethod {
    /// 古代大单位表示法，亿亿为兆，兆兆为京
    Old,
    /// 现代表示法，最大是亿
    Std,
}

#[derive(Parser)]
#[command(name = "中国数字", version = "0.1", about = "把数字变成中国读法！", long_about = None)]
struct Args {
    /// 要转换的十进制阿拉伯数字
    num: String,
    /// 对两的用法
    #[arg(short, long, default_value_t = LiangOptionCmd::Disable, value_enum)]
    liang_option: LiangOptionCmd,
    /// 大单位的体系
    #[arg(short, long, default_value_t = AttMethod::Old, value_enum)]
    att_method: AttMethod,
    /// 显示的编码，详情请在 https://crates.io/crates/encoding 查阅
    #[arg(short, long, default_value_t = String::from("utf-8"))]
    encoding: String,
}

pub fn main() {
    let args = Args::parse();
    let text = (match args.att_method {
        AttMethod::Old => sinonumify::<OldAttUnits>,
        AttMethod::Std => sinonumify::<StdAttUnits>,
    })(&args.num, args.liang_option.to());
    let out_text = encoding_from_whatwg_label(&args.encoding)
        .unwrap_or_else(|| panic!("cannot find encoding {} !", args.encoding))
        .encode(&text, EncoderTrap::Strict)
        .expect("failed to encoding");
    io::stdout().write(&out_text).unwrap();
}
