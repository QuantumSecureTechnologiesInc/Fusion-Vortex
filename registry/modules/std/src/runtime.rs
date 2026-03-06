use crate::security::{get_context, verify_current_context, FusionSecurityError};
#[derive(Debug)]
pub enum FusionRuntimeError {
    MissingContext,
    InvalidContext,
    ChannelClosed,
}
impl From<FusionSecurityError> for FusionRuntimeError {
    fn from(err: FusionSecurityError) -> Self {
        match err {
            FusionSecurityError::InvalidContext => Self::InvalidContext,
            FusionSecurityError::MissingContext => Self::MissingContext,
        }
    }
}
/// Secure sender gated by the active context.
pub struct FusionSender<T> {
    tx: mpsc::Sender<T>,
}
/// Secure receiver gated by the active context.
pub struct FusionReceiver<T> {
    rx: mpsc::Receiver<T>,
}
pub fn fusion_channel<T>() -> (FusionSender<T>, FusionReceiver<T>) {
    let (tx, rx) = mpsc::channel();
    (FusionSender { tx }, FusionReceiver { rx })
}
impl<T> FusionSender<T> {
    pub fn send(&self, value: T) -> Result<(), FusionRuntimeError> {
        if !verify_current_context() {
            return Err(FusionRuntimeError::InvalidContext);
        }
        self.tx.send(value).map_err(|_| FusionRuntimeError::ChannelClosed)
    }
}
impl<T> FusionReceiver<T> {
    pub fn recv(&self) -> Result<T, FusionRuntimeError> {
        if !verify_current_context() {
            return Err(FusionRuntimeError::InvalidContext);
        }
        self.rx.recv().map_err(|_| FusionRuntimeError::ChannelClosed)
    }
}
/// Spawns a thread only when the active context is valid.
pub fn spawn_secure<F, T>(
    f: F,
) -> Result<thread::JoinHandle<Result<T, FusionRuntimeError>>, FusionRuntimeError>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let ctx = get_context().ok_or(FusionRuntimeError::MissingContext)?;
    if !ctx.verify() {
        return Err(FusionRuntimeError::InvalidContext);
    }
    Ok(
        thread::spawn(move || {
            if !ctx.verify() {
                return Err(FusionRuntimeError::InvalidContext);
            }
            Ok(f())
        }),
    )
}