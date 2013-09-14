
use actor::{SurviveOrDie, Survive, Die, Actor};
use actorref::{ActorRefWithStream, ActorRefWithChan, ActorRefWithPort,
               ActorRefWithoutPortAndChan};

use std::cell::Cell;
// use std::vec::{append_one};


/// A system of actors
/// TODO store references to actors in the struct 
pub struct System;

impl System {

    pub fn new() -> System { System }

    // Spawns an actor with a Port<T1> and a Chan<T2>.
    pub fn add_actor<InitArgs: Send, T1: Send, T2: Send,
                     A: Actor<T1, T2,
                              Chan<T2>>>(&mut self, args: InitArgs,
                                         init: ~fn(InitArgs) -> A) ->
     ActorRefWithStream<Port<T2>, Chan<T1>> {
        let (in_port, in_chan): (Port<T1>, Chan<T1>) = stream();
        let (out_port, out_chan): (Port<T2>, Chan<T2>) = stream();

        self.spawn_actor(in_port, out_chan, args, init);

        ActorRefWithStream{port: out_port, chan: in_chan,}
    }

    // Spawns an actor that listens to an existing GenericPort<T1>.
    pub fn add_actor_from_port<T1: Send, T2: Send, InitArgs: Send,
                               P1: GenericPort<T1> + Send,
                               A: Actor<T1, T2,
                                        Chan<T2>>>(&mut self, port: P1,
                                                   args: InitArgs,
                                                   init: ~fn(InitArgs) -> A)
     -> ActorRefWithPort<Port<T2>> {

        let (out_port, out_chan): (Port<T2>, Chan<T2>) = stream();
        self.spawn_actor(port, out_chan, args, init);

        ActorRefWithPort{port: out_port,}
    }

    // Spawns an actor that sends to an existing GenericChan<T2>.
    pub fn add_actor_from_chan<T1: Send, T2: Send, InitArgs: Send,
                               C2: GenericChan<T2> + Send,
                               A: Actor<T1, T2,
                                        C2>>(&mut self, chan: C2,
                                             args: InitArgs,
                                             init: ~fn(InitArgs) -> A) ->
     ActorRefWithChan<Chan<T1>> {

        let (in_port, in_chan): (Port<T1>, Chan<T1>) = stream();
        self.spawn_actor(in_port, chan, args, init);

        ActorRefWithChan{chan: in_chan,}
    }

    // Spawns an actor that listens to existing GenericPort<T1> 
    /// and sends to existing GenericChan<T2>.
    pub fn add_actor_from_port_and_chan<T1: Send, T2: Send, InitArgs: Send,
                                        P1: GenericPort<T1> + Send,
                                        C2: GenericChan<T2> + Send,
                                        A: Actor<T1, T2,
                                                 C2>>(&mut self, port: P1,
                                                      chan: C2,
                                                      args: InitArgs,
                                                      init:
                                                          ~fn(InitArgs) -> A)
     -> ActorRefWithoutPortAndChan {
        self.spawn_actor(port, chan, args, init);

        ActorRefWithoutPortAndChan
    }

    /// Private utility function for spawing an actors.
    /// todo spawn controller
    fn spawn_actor<T1: Send, T2: Send, P1: GenericPort<T1> + Send,
                   C2: GenericChan<T2> + Send, InitArgs: Send,
                   A: Actor<T1, T2,
                            C2>>(&mut self, port: P1, chan: C2,
                                 args: InitArgs, init: ~fn(InitArgs) -> A) {

        let port_cell = Cell::new(port);
        let chan_cell = Cell::new(chan);
        let args_cell = Cell::new(args);

        do spawn || {
            let port = port_cell.take();
            let chan = chan_cell.take();
            let mut actor = init(args_cell.take());

            actor.on_start();
            loop  {
                match port.try_recv() {
                    None => { actor.on_missing_port(); break  }
                    Some(msg) => {
                        let answer: SurviveOrDie =
                            actor.on_receive(msg, &chan);
                        match answer { Die => { break  } Survive => { } }
                    }
                }
            }
            actor.on_stop();
        };
    }
}

#[test]
fn test_add_actor() {

    struct MyActor;
    impl Actor<int, int, Chan<int>> for MyActor {
        fn on_receive(&mut self, msg: int, chan: &Chan<int>) -> SurviveOrDie {
            chan.send(msg);
            Survive
        }
    }

    let mut system = System::new();
    let actor_ref = system.add_actor((), |_| { MyActor });

    actor_ref.chan.send(1);
    actor_ref.chan.send(2);
    assert_eq!(1 , actor_ref.port.recv());
    assert_eq!(2 , actor_ref.port.recv());
}



#[test]
fn test_add_actor_from_port_and_chan() {


    struct MyActor;
    impl Actor<int, int, Chan<int>> for MyActor {
        fn on_receive(&mut self, msg: int, chan: &Chan<int>) -> SurviveOrDie {
            chan.send(msg);
            Survive
        }
    }

    let mut system = System::new();
    let (in_port, in_chan) = stream();
    let (out_port, out_chan) = stream();
    system.add_actor_from_port_and_chan(in_port, out_chan, (), |_| { MyActor
                                    });

    in_chan.send(1);
    in_chan.send(2);
    assert_eq!(1 , out_port.recv());
    assert_eq!(2 , out_port.recv());
}

#[test]
fn test_add_actor_from_chan() {

    struct MyActor;
    impl Actor<int, int, Chan<int>> for MyActor {
        fn on_receive(&mut self, msg: int, chan: &Chan<int>) -> SurviveOrDie {
            chan.send(msg);
            Survive
        }
    }

    let mut system = System::new();
    let (out_port, out_chan) = stream();
    let actor_ref = system.add_actor_from_chan(out_chan, (), |_| { MyActor });

    actor_ref.chan.send(1);
    actor_ref.chan.send(2);
    assert_eq!(1 , out_port.recv());
    assert_eq!(2 , out_port.recv());
}

#[test]
fn test_actor_from_port() {

    struct MyActor;
    impl Actor<int, int, Chan<int>> for MyActor {
        fn on_receive(&mut self, msg: int, chan: &Chan<int>) -> SurviveOrDie {
            chan.send(msg);
            Survive
        }
    }

    let mut system = System::new();
    let (in_port, in_chan) = stream();
    let actor_ref = system.add_actor_from_port(in_port, (), |_| { MyActor });

    in_chan.send(1);
    in_chan.send(2);
    assert_eq!(1 , actor_ref.port.recv());
    assert_eq!(2 , actor_ref.port.recv());
}


#[test]
fn test_counter() {

    pub enum CounterMsg { Add(int), GetSum, }

    pub struct Counter {
        value: int,
    }

    impl <C: GenericChan<int>> Actor<CounterMsg, int, C> for Counter {
        fn on_receive(&mut self, msg: CounterMsg, chan: &C) -> SurviveOrDie {
            match msg {
                Add(value) => self.value += value,
                GetSum => chan.send(self.value)
            };
            Survive
        }
    }

    let mut system = System::new();
    let counter = system.add_actor(0, |value| Counter{value: value,});

    for i in range(0, 100) { let c = &counter.chan; c.send(Add(i)); }
    counter.chan.send(GetSum);

    let result: int = counter.port.recv();
    assert_eq!(result , 100 * 99 / 2)
}


