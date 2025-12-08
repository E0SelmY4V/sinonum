use clap::{Parser, ValueEnum};
use sinonum::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum LiangOptionCmd {
    Disable,
    WithUnit,
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

#[derive(Parser)]
#[command(name = "中国数字", version = "0.1", about = "把数字变成中国读法！", long_about = None)]
struct Args {
    /// 要转换的十进制阿拉伯数字
    num: String,
    /// 对两的用法
    #[arg(short, long, default_value_t = LiangOptionCmd::Disable, value_enum)]
    liang_option: LiangOptionCmd,
}

pub fn main() {
    let args = Args::parse();
    print!("{}", sinonumify(&args.num, args.liang_option.to()));
}
