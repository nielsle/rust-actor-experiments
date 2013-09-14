extern mod actor;
extern mod extra;
extern mod std;

use actor::actor::{Actor, SurviveOrDie, Survive, Die};
use actor::system::System;
use std::iter::{Iterator};

pub enum SieveMsg { Try(int), Prime(int), Gather, }

struct Sieve {
    divisor: Option<int>,
}
impl Sieve {

    fn new() -> Sieve { Sieve{divisor: None,} }
}

impl <C: GenericChan<SieveMsg>> Actor<SieveMsg, SieveMsg, C> for Sieve {

    fn on_receive(&mut self, msg: SieveMsg, chan: &C) -> SurviveOrDie {
        match msg {
            Try(number) => {
                match (self.divisor) {
                    None => { self.divisor = Some(number) }
                    Some(div) => {
                        if (number % div != 0) { chan.send(Try(number)) }
                    }
                };
                Survive
            }
            Gather => {
                match self.divisor {
                    Some(div) => chan.send(Prime(div)),
                    None => { }
                }
                chan.send(Gather);
                Die
            }
            Prime(number) => { 
                chan.send(Prime(number)); Survive 
            }
        }
    }
}

fn main() {

    let mut system = System::new();

    // Create pipeline
    let (port, chan) = stream();
    let mut port = port;
    for _ in range (0,10) {
        port = system.add_actor_from_port(port, (), |_| Sieve::new()).port;
    }

    // Send numbers  waiting for output
    for number in range(2, 20) {
        chan.send(Try(number));
        if (port.peek()) { break  }
    }

    // Read primes
    chan.send(Gather);
    let mut primes: ~[int] = ~[];
    loop  {
        let msg: SieveMsg = port.recv();
        match msg {
            Prime(number) => { primes = std::vec::append_one(primes, number) }
            Try(_) => { }
            Gather => { break  }
        }
    }

    // Check output
    let expected: ~[int] = ~[2, 3, 5, 7, 11, 13, 17, 19];
    for (i, j) in expected.iter().zip(primes.iter()) { assert_eq!(i , j); }
}


