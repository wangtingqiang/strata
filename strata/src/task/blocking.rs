/// 在当前 tracing span 上下文中执行 `spawn_blocking`。
///
/// `tokio::task::spawn_blocking` 在独立 OS 线程池上运行，
/// 默认不会继承调用方的 tracing span 上下文，导致 span 链断裂。
/// 本函数先捕获调用方的 span，再传入阻塞线程执行，确保 span 正确挂接到父 trace。
pub fn spawn_blocking_with_current_span<F, R>(f: F) -> tokio::task::JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}
