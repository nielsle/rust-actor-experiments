extern mod actor;
extern mod std;

use std::int;

use actor::actor::{ActorWithStream};

struct Exponent {
    value: uint,
}

fn main() {

    let mut results =
        do range(0, 6).map |i: int| {
            //The number 2u is stored in exponent.value
            let actor =
                do ActorWithStream::new(2u, |a| { Exponent{value: a,}
                                    }) |exponent, chan, msg| {
                    chan.send(int::pow(msg, exponent.value));
                    true
                };

            actor.chan.send(i);
            actor.port
        };
    let mut norm2 = 0;
    for port in results { norm2 += port.recv(); }
    assert_eq!(norm2 , 1 * 1 + 2 * 2 + 3 * 3 + 4 * 4 + 5 * 5);
}

