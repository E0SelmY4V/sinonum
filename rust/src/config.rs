//! # 配置相关

/// # 两相关的配置
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LiangOption {
    /// 禁用两
    Disable,
    /// 末尾带单位，即个位可以是两
    WithUnit,
    /// 纯数字表示，个位不能用两
    JustNumber,
}

/// # 一十相关的配置
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum YishiOption {
    /// 总是以“一十三”“一十四”表示
    Always,
    /// 除了开头是“一十”时，其他时候都不省略“一十”的表述
    ExceptHead,
    /// 尽量避免“一十”的表述
    Avoid,
}

/// # 配置表
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Config {
    pub liang: LiangOption,
    pub yishi: YishiOption,
}
