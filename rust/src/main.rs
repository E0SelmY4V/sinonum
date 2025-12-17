#![windows_subsystem = "windows"]

use clap::Parser;
use std::io::{self, Write};

mod args {
    use clap::{Parser, ValueEnum};
    use encoding::{EncoderTrap, label::encoding_from_whatwg_label};
    use sinonum::{
        att_uniter::{OldAttUnits, StdAttUnits},
        config::{Config, LiangOption, YishiOption},
        sinonumify,
    };

    macro_rules! cmdify_option {
        (
            $cmd_id: ident,
            $ori_id: ident,
            $(
                $(
                    #[doc = $doc:expr]
                )*
                $enum: ident,
            )*
        ) => {
            #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
            enum $cmd_id {
                $(
                    $(
                        #[doc = $doc]
                    )*
                    $enum,
                )*
            }
            impl From<$cmd_id> for $ori_id {
                fn from(liang_option_cmd: $cmd_id) -> Self {
                    match liang_option_cmd {
                        $(
                            $cmd_id::$enum => Self::$enum,
                        )*
                    }
                }
            }
        };
    }
    cmdify_option!(
        LiangOptionCmd,
        LiangOption,
        /// 禁用两
        Disable,
        /// 末尾带单位，即个位可以是两
        WithUnit,
        /// 纯数字表示，个位不能用两
        JustNumber,
    );
    cmdify_option!(
        YishiOptionCmd,
        YishiOption,
        /// 总是以“一十三”“一十四”表示
        Always,
        /// 除了开头是“一十”时，其他时候都不省略“一十”的表述
        ExceptHead,
        /// 尽量避免“一十”的表述
        Avoid,
    );
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
    pub enum AttMethod {
        /// 古代大单位表示法，亿亿为兆，兆兆为京
        Old,
        /// 现代表示法，最大是亿
        Std,
    }

    #[derive(Parser, Clone)]
    #[command(name = "中国数字", version = "0.1", about = "把数字变成中国读法！", long_about = None)]
    pub struct Args {
        /// 要转换的十进制阿拉伯数字
        num: Vec<String>,
        /// 对两的用法
        #[arg(short, long, default_value_t = LiangOptionCmd::Disable, value_enum)]
        liang: LiangOptionCmd,
        /// 对一十的用法
        #[arg(short, long, default_value_t = YishiOptionCmd::Always, value_enum)]
        yishi: YishiOptionCmd,
        /// 大单位的体系
        #[arg(short, long, default_value_t = AttMethod::Old, value_enum)]
        att_method: AttMethod,
        /// 显示的编码，详情请在 https://crates.io/crates/encoding 查阅
        #[arg(short, long, default_value_t = String::from("utf-8"))]
        pub encoding: String,
    }
    impl Args {
        pub fn run(&self) -> Vec<String> {
            let f = match self.att_method {
                AttMethod::Old => sinonumify::<OldAttUnits>,
                AttMethod::Std => sinonumify::<StdAttUnits>,
            };
            let config = self.clone().into();
            self.num.iter().map(|num| f(num, config)).collect()
        }
        pub fn encode(&self, text: &str) -> Vec<u8> {
            encoding_from_whatwg_label(&self.encoding)
                .unwrap_or_else(|| panic!("cannot find encoding {} !", self.encoding))
                .encode(&text, EncoderTrap::Strict)
                .expect("failed to encoding")
        }
    }
    impl From<Args> for Config {
        fn from(value: Args) -> Self {
            Self {
                liang: value.liang.into(),
                yishi: value.yishi.into(),
            }
        }
    }
}

pub fn main() {
    let args = args::Args::parse();
    let text = args.run();
    io::stdout().write(&args.encode(&text.join("\n"))).unwrap();
}
