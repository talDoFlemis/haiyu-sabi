use std::future::Future;

use std::any::Any;

use crate::error::ActorError;

pub trait Message: Any + Send + Sized + 'static {}

/// Represents the state of an actor. Must be safe
/// to send between threads (same bounds as a [Message])
pub trait State: std::any::Any + Send + 'static {}
impl<T: std::any::Any + Send + 'static> State for T {}

pub trait Actor: Sized + Send + Sync + 'static {
    /// The kind of message this mtfk can process. AKA It's interface of thing it can accept
    type Msg: Message;

    /// The actual state of a actor at any given time
    type State: State;

    /// A turn is defined as the processing of a
    /// single message by an actor. In other words, a turn defines
    /// the process of an actor taking a message from its inbox and
    /// processing that message to completion.
    fn turn(
        &self,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> impl Future<Output = Result<(), ActorError>> + Sync;
}
