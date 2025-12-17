#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LiangOption {
    /// 禁用两
    Disable,
    /// 末尾带单位，即个位可以是两
    WithUnit,
    /// 纯数字表示，个位不能用两
    JustNumber,
}

pub struct Config {
    pub liang: LiangOption,
}
