use std::comm::{SharedChan};

use actor::{spawn_actor, spawn_actor_to_stream};

enum PingPongMsg { Ping, Pong, }

struct PingPong {

    counter: int,
    chan: Chan<PingPongMsg>,
    master: SharedChan<bool>,
}

fn new_ping_pong(args: ((int, SharedChan<bool>), Chan<PingPongMsg>)) ->
 PingPong {
    let (head, chan) = args;
    let (counter, master) = head;
    PingPong{counter: counter, chan: chan, master: master,}
}

fn on_receive(actor: &mut PingPong, msg: PingPongMsg) -> bool {

    match msg { Ping => actor.chan.send(Pong), Pong => actor.chan.send(Ping) }
    actor.counter -= 1;
    if (actor.counter == 0) { actor.master.send(true); return false };
    true
}

#[test]
fn test_pingping() {

    // Prepare result
    let (master_port, master_chan) = stream();
    let master = SharedChan::new(master_chan);

    // Spawn first actor
    let (port, chan) =
        spawn_actor_to_stream((10, master.clone()), new_ping_pong,
                              on_receive);
    chan.send(Ping);
    // Spawning second actor using port and chan
    spawn_actor(port, ((10, master.clone()), chan), new_ping_pong,
                on_receive);

    // Both actors must terminate
    assert_eq!(true , master_port . recv ( ));
    assert_eq!(true , master_port . recv ( ));
}
