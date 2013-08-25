use std::cell::Cell;

/// Spawns an actor. Returns nothing
///
/// Usage
/// do spawn_actor(in_port, arguments, factory ) |actor, msg | {
///     ... on_receive
/// }
///
/// factory is a function that constructs an actor
/// arguments are arguments for the factory
/// in_port is a port for sending data to the actor
pub fn spawn_actor<T: Send, P: Send,
                   A>(in_port: Port<T>, args: P, new_actor: ~fn(P) -> A,
                      on_receive: ~fn(&mut A, T) -> bool) {

    let args_cell = Cell::new(args);

    do spawn || {
        let mut actor = new_actor(args_cell.take());
        loop  {
            match in_port.try_recv() {
                None => { break  }
                Some(msg) => { if !on_receive(&mut actor, msg) { break  } }
            }
        }
    };
}

/// Spawns an actor. Returns a port and a channel
///
/// Usage
/// let (port, chan) = do spawn_actor_to_stream(arguments, factory)  |actor, msg | {
///     ... on_receive
/// }
///
/// factory is a function that constructs an actor
/// arguments are arguments for the factory
pub fn spawn_actor_to_stream<T1: Send, T2: Send, P: Send,
                             A>(args: P, new_actor: ~fn((P, Chan<T2>)) -> A,
                                on_receive: ~fn(&mut A, T1) -> bool) ->
 (Port<T2>, Chan<T1>) {
    let (in_port, in_chan) = stream();
    let (out_port, out_chan) = stream();
    spawn_actor(in_port, (args, out_chan), new_actor, on_receive);
    (out_port, in_chan)
}


/// Spawns an actor. Returns port of output stream
///
/// Usage
/// let port = do spawn_actor_to_port(arguments, factory) |actor, msg| {
///     ... on receive
/// }
///
/// factory is a function that constructs an actor
/// arguments are arguments for the factory
pub fn spawn_actor_to_port<T1: Send, T2: Send, P: Send,
                           A>(in_port: Port<T1>, args: P,
                              new_actor: ~fn((P, Chan<T2>)) -> A,
                              on_recieve: ~fn(&mut A, T1) -> bool) ->
 Port<T2> {

    let (out_port, out_chan) = stream();
    spawn_actor(in_port, (args, out_chan), new_actor, on_recieve);
    out_port
}


/// Spawns an actor. Returns channel of input stream
///
/// Usage
/// let chan = do spawn_actor_to_chan(in_port, arguments, factory) | actor, msg| {
///     ... on receive
/// }
///
/// factory is a function that constructs an actor
/// arguments are arguments for the factory
/// in_port is a port for sending data to the actor
pub fn spawn_actor_to_chan<T: Send, P: Send,
                           A>(args: P, new_actor: ~fn(P) -> A,
                              on_recieve: ~fn(&mut A, T) -> bool) -> Chan<T> {

    let (in_port, in_chan) = stream();
    spawn_actor(in_port, args, new_actor, on_recieve);
    in_chan
}

#[test]
fn test_spawn_actor() {

    struct MyActor {
        out_chan: Chan<int>,
    }
    let (in_port, in_chan) = stream();
    let (out_port, out_chan) = stream();

    do spawn_actor(in_port, out_chan,
                   |out_chan| MyActor{out_chan: out_chan,}) |actor, msg| {
        actor.out_chan.send(msg);
        true
    };
    in_chan.send(1);
    in_chan.send(2);
    assert_eq!(1 , out_port . recv ( ));
    assert_eq!(2 , out_port . recv ( ));
}




#[test]
fn test_spawn_actor_to_stream() {

    struct MyActor {
        out_chan: Chan<int>,
    }

    let (port, chan) =
        do spawn_actor_to_stream((),
                                 |(_, out_chan)|
                                     MyActor{out_chan:
                                                 out_chan,}) |actor, msg| {
            actor.out_chan.send(msg);
            true
        };
    chan.send(1);
    chan.send(2);
    assert_eq!(1 , port . recv ( ));
    assert_eq!(2 , port . recv ( ));
}



#[test]
fn test_spawn_actor_to_chan() {

    struct MyActor {
        out_chan: Chan<int>,
    }
    let (out_port, out_chan) = stream();

    let in_chan =
        do spawn_actor_to_chan(out_chan,
                               |out_chan|
                                   MyActor{out_chan:
                                               out_chan,}) |actor, msg| {
            actor.out_chan.send(msg);
            true
        };
    in_chan.send(1);
    in_chan.send(2);
    assert_eq!(1 , out_port . recv ( ));
    assert_eq!(2 , out_port . recv ( ));
}

#[test]
fn test_spawn_actor_to_port() {

    struct MyActor {
        out_chan: Chan<int>,
    }
    let (in_port, in_chan) = stream();

    let out_port =
        do spawn_actor_to_port(in_port, (),
                               |(_, out_chan)|
                                   MyActor{out_chan:
                                               out_chan,}) |actor, msg| {
            actor.out_chan.send(msg);
            true
        };
    in_chan.send(1);
    in_chan.send(2);
    assert_eq!(1 , out_port . recv ( ));
    assert_eq!(2 , out_port . recv ( ));
}

