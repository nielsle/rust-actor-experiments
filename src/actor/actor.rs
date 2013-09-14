
/// Message from actor to handler.
/// TODO: Idea pass a callback to the constructor instead 
pub enum SurviveOrDie { Survive, Die, }

/// An actor that has access to a default port
/// Implement this trait for your actor struct
pub trait Actor<T1: Send, T2: Send, C2: GenericChan<T2> + Send> {

    // React to a message.
    fn on_receive(&mut self, msg: T1, chan: &C2) -> SurviveOrDie;

    fn on_start(&mut self) { debug!("on start"); }

    fn on_stop(&mut self) { debug!("on stop"); }

    // Hook for sending a message when the port is gone.
    // This is often not an error. It just means that the sender 
    // will not send any more messages
    fn on_missing_port(&mut self) { debug!("Missing port. Stopping"); }
}

