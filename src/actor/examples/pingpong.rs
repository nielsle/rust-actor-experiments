use std::comm::{SharedChan};

use actor::{Actor};

enum PingPongMsg { Ping, Pong, }

struct PingPong {

    counter: int,
    chan: Chan<PingPongMsg>,
    master: SharedChan<bool>,
}

fn new_ping_pong(args: (int, Chan<PingPongMsg>, SharedChan<bool>)) ->
 PingPong {
    let (counter, chan, master) = args;
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

    let (master_port, master_chan) = stream();
    let master_chan = SharedChan::new(master_chan);

    let (port1, chan1) = stream();
    let (port2, chan2) = stream();
    chan1.send(Ping);
    Actor::new(port1, (10, chan2, master_chan.clone()), new_ping_pong,
               on_receive);
    Actor::new(port2, (10, chan1, master_chan.clone()), new_ping_pong,
               on_receive);

    // Both actors must terminate
    assert_eq!(true , master_port.recv());
    assert_eq!(true , master_port.recv());
}
