use std::cell::Cell;

/// A private function for spawning an actor.
///
/// Usage
/// do spawn_actor<S, T>(port, arg, factory ) |state : S, msg :T| {
///     ... on_receive
/// }
///
/// S is the state of the actor (wrapped by the actor object)
/// T is a type of message that the actor can receive
/// port is a port of messages to the actor
/// args is a list of arguments for init
/// init is a funktion that returns a state (of type S)
fn spawn_actor<S, T: Send, P: Send>(port: Port<T>, args: P, init: ~fn(P) -> S,
                      on_receive: ~fn(&mut S, T) -> bool) {

    let args_cell = Cell::new(args);

    do spawn || {
        let mut state = init(args_cell.take());
        loop  {
            match port.try_recv() {
                None => { break  }
                Some(msg) => { if !on_receive(&mut state, msg) { break  } }
            }
        }
    };
}

/// A private function for spawning an actor. This version assumes the actor to own a port.
///
/// Usage
/// do spawn_actor_with_port<S, T>(in_port, out_chan, arg, factory ) 
///                                 |state : S, msg :T1, chan: Chan<T2>| {
///     ... on_receive
/// }
///
/// S is the state of the actor (wrapped by the actor object)
/// T1 is a type of message that the actor can receive
/// T2 is a type of message that the actor can send
/// port is a port of messages to the actor
/// chan is a port of messages to the actor
/// args is a list of arguments for init
/// init is a funktion that returns a state (of type S)
fn spawn_actor_with_port<S, T1: Send, T2: Send, P: Send>(port: Port<T1>, chan: Chan<T2>, 
            args: P, init: ~fn(P) -> S, on_receive: ~fn(&mut S, &Chan<T2>, T1) -> bool) {

    let args_as_cell = Cell::new(args);
    let chan_as_cell = Cell::new(chan);

    do spawn || {
    
        let mut state = init(args_as_cell.take());
        let chan = chan_as_cell.take();
        
        loop  {
            match port.try_recv() {
                None => { break  }
                Some(msg) => { if !on_receive(&mut state, &chan, msg) { break  } }
            }
        }
    };
}



/// Actor is a wrapper around spawn_actor. Right now it is just an empty
/// struct, but it may get more functionality in the future
///
/// Usage
/// let actor  = Actor::new(args, init)  |state: S, msg: T| {
///     ... on_receive
/// }
///
/// S is the state of the actor (wrapped by the actor object)
/// T is a type of message that the actor can receive
/// port: port<T> is a port of messages to the actor
/// args: is a list of arguments for init
/// init: is a funktion that returns a state.
pub struct Actor;

impl Actor {

    pub fn new<S, T: Send, P:Send>(port: Port<T>, args: P, init: ~fn(P) -> S,
                      on_receive: ~fn(&mut S, T) -> bool) ->  Actor {

        spawn_actor(port, args, init, on_receive);
        Actor
    }
}

/// An actor that owns a channel and a port
///
/// Usage
/// let  actor ActorWithStream(args, init) |state: S, msg: T1, chan: Chan<T2>| {
///     ... on_receive
/// }
///
/// S is the state of the actor (wrapped by the actor object)
/// T1 is a type of message that the actor can receive
/// T2 is a type of message that the actor can send
/// args is a list of arguments for init
/// init is a funktion that returns a state (of type S)
pub struct  ActorWithStream<T1, T2>{
    chan: Chan<T1>,
    port: Port<T2>
}

impl<T1: Send, T2: Send> ActorWithStream<T1, T2> {

    pub fn new<S, P:Send>(args: P, init: ~fn(P) -> S,
                      on_receive: ~fn(&mut S, &Chan<T2>, T1) -> bool) -> ActorWithStream<T1, T2>{

        let (in_port, in_chan): (Port<T1>,Chan<T1>)= stream();
        let (out_port, out_chan): (Port<T2>,Chan<T2>)= stream();

        spawn_actor_with_port(in_port, out_chan, args, init, on_receive);
        ActorWithStream{port: out_port, chan: in_chan}
    }
}


/// An actor that owns a port
///
/// Usage
/// let  actor ActorWithPort(in_port, args, init) |state: S, msg: T1| {
///     ... on_receive
/// }
///
/// S is the state of the actor (wrapped by the actor object)
/// T1 is a type of message that the actor can receive
/// T2 is a type of message that the actor can send
/// in_port is a port of messages to the actor
/// args is a list of arguments for init
/// init is a funktion that returns a state (of type S)
pub struct  ActorWithPort<T2>{
    port: Port<T2>
}

impl<T2:  Send> ActorWithPort<T2> {

    pub fn new<S, T1: Send, P:Send>(in_port : Port<T1>, args: P, init: ~fn(P) -> S,
              on_receive: ~fn(&mut S, &Chan<T2>, T1) -> bool) -> ActorWithPort<T2> {

        let (out_port, out_chan): (Port<T2>,Chan<T2>)= stream();
        spawn_actor_with_port(in_port, out_chan, args, init, on_receive);
        ActorWithPort{port: out_port}
    }
}

/// An actor that owns a Channel
///
/// Usage
/// let  actor ActorWithChan(port<T1>, args, init) |state : S, msg :T1| {
///     ... on_receive
/// }
///
/// S is the state of the actor (wrapped by the actor object)
/// T is a type of message that the actor can receive
/// args is a list of arguments for init
/// init is a funktion that returns a state (of type S)
pub struct  ActorWithChan<T>{
    chan: Chan<T>
}

impl<T: Send> ActorWithChan<T> {

    pub fn new<S, P:Send>(args: P, init: ~fn(P) -> S,
                      on_receive: ~fn(&mut S, T) -> bool) ->  ActorWithChan<T >{

        let (in_port, in_chan) = stream();
        spawn_actor(in_port, args, init, on_receive);
        ActorWithChan{chan: in_chan}
    }
}

#[test]
fn test_actor() {

    struct MyState {
        chan: Chan<int>,
    }
    
    let (in_port, in_chan) = stream();
    let (out_port, out_chan) = stream();

    do Actor::new(in_port, out_chan, |c| MyState{chan: c}) |actor, msg| {
        actor.chan.send(msg);
        true
    };
    
    in_chan.send(1);
    in_chan.send(2);
    assert_eq!(1, out_port.recv());
    assert_eq!(2, out_port.recv());
}


#[test]
fn test_spawn_actor_with_stream() {

    struct MyState;

    let actor = do ActorWithStream::new((),
                |()| MyState) |_state, chan, msg| {
                    chan.send(msg);
                    true // True  for survival
        };
        
    actor.chan.send(1);
    actor.chan.send(2);
    assert_eq!(1, actor.port.recv());
    assert_eq!(2, actor.port.recv());
}


#[test]
fn test_actor_with_chan() {

    struct MyState {
        chan: Chan<int>,
    }
    let (out_port, out_chan) = stream();

    let actor = do ActorWithChan::new(out_chan, |c| MyState{chan: c}) |state, msg| {
            state.chan.send(msg);
            true
        };

    actor.chan.send(1);
    actor.chan.send(2);
    assert_eq!(1, out_port.recv());
    assert_eq!(2, out_port.recv());
}

#[test]
fn test_actor_with_port() {

    struct MyActor;

    let (in_port, in_chan) = stream();
    let actor = do ActorWithPort::new(in_port, 
            (), |()| MyActor) |_state, chan, msg| {
            chan.send(msg);
            true
        };

    in_chan.send(1);
    in_chan.send(2);
    assert_eq!(1, actor.port.recv());
    assert_eq!(2, actor.port.recv());
}

