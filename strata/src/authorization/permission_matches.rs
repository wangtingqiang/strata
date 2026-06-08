/// 判断已授予的权限码是否匹配请求的权限码。
///
/// 支持通配符：`*` 匹配所有，`{prefix}.*` 匹配以该前缀开头的权限。
pub fn permission_matches(granted: &str, code: &str) -> bool {
    if granted == "*" {
        return true;
    }
    if granted == code {
        return true;
    }
    if let Some(prefix) = granted.strip_suffix(".*") {
        return code.starts_with(&format!("{prefix}."));
    }
    false
}
