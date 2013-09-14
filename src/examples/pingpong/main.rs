extern mod actor;
extern mod std;

use actor::actor::{Actor, SurviveOrDie, Survive, Die};
use actor::system::{System};
use std::comm::{SharedChan};

enum PingPongMsg { Ping, Pong, }

struct PingPong {

    counter: uint,
    master: SharedChan<bool>,
}

impl <C: GenericChan<PingPongMsg>> Actor<PingPongMsg, PingPongMsg, C> for
     PingPong {
    fn on_receive(&mut self, msg: PingPongMsg, chan: &C) -> SurviveOrDie {

        match msg { Ping => chan.send(Pong), Pong => chan.send(Ping) }
        self.counter -= 1;
        if (self.counter == 0u) { self.master.send(true); return Die };
        Survive
    }

}

fn new_pingpong(args: (uint, SharedChan<bool>)) -> PingPong {
    let (counter, master) = args;
    PingPong{counter: counter, master: master,}
}


fn main() {

    let mut system = System::new();

    let (master_port, master_chan): (Port<bool>, Chan<bool>) = stream();
    let master_chan = SharedChan::new(master_chan);

    let (port1, chan1): (Port<PingPongMsg>, Chan<PingPongMsg>) = stream();
    let (port2, chan2): (Port<PingPongMsg>, Chan<PingPongMsg>) = stream();
    chan1.send(Ping);

    system.add_actor_from_port_and_chan(port1, chan2,
                                        (10u, master_chan.clone()),
                                        new_pingpong);
    system.add_actor_from_port_and_chan(port2, chan1,
                                        (10u, master_chan.clone()),
                                        new_pingpong);
    // Both actors must terminate
    assert_eq!(true , master_port.recv());
    assert_eq!(true , master_port.recv());
}
