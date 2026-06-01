/// 校验字符串是否符合小写蛇形机器标识规则
///
/// 规则如下：
/// - 首字符必须是小写英文字母
/// - 后续字符只允许小写英文字母、数字或下划线
/// - 不负责去除首尾空白
pub fn is_snake_lower_alphanumeric(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    if !first.is_ascii_lowercase() {
        return false;
    }

    chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
}
