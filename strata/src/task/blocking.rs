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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn runs_closure_and_returns_result() {
        let handle = spawn_blocking_with_current_span(|| 21 * 2);
        assert_eq!(handle.await.unwrap(), 42);
    }

    #[tokio::test]
    async fn propagates_current_span_to_blocking_thread() {
        tracing_subscriber::fmt().with_test_writer().try_init().ok();

        let span = tracing::info_span!("blocking-task");

        let (caller_span_id, handle) = span.in_scope(|| {
            let id = tracing::Span::current()
                .id()
                .expect("span should have an id with an active subscriber");
            let handle = spawn_blocking_with_current_span(|| tracing::Span::current().id());
            (id, handle)
        });

        let blocking_span_id = handle
            .await
            .unwrap()
            .expect("blocking thread should inherit the caller span");

        assert_eq!(blocking_span_id, caller_span_id);
    }
}
