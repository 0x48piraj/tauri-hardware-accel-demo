pub(crate) mod envelope;
pub(crate) mod transport {
    pub(crate) mod message;
}
pub(crate) mod browser_state;
pub(crate) mod renderer_state;
pub(crate) mod pending;
pub(crate) mod rpc;
pub(crate) mod binary_buffer;
pub(crate) mod utils;
pub(crate) mod responder;
pub(crate) mod event;
pub(crate) mod stream;
pub(crate) mod request_response;
pub(crate) mod router;
pub(crate) mod browser;
pub(crate) mod renderer;
pub(crate) mod handle_cell;

// Public exports for the rest of the application
pub use browser::handle_ipc_message;
pub use renderer::IpcRenderProcessHandler;
pub use browser_state::{IpcResult, IpcError, IpcContext};
pub use router::IpcRouter;
pub use request_response::{RequestResponseSubsystem, SyncHandler, AsyncHandler, BinaryResponder};
pub use responder::Responder;
pub use event::EventSubsystem;
pub use stream::{StreamSubsystem, StreamHandler, StreamFactory, StreamResponder};
pub use handle_cell::AppCell;
