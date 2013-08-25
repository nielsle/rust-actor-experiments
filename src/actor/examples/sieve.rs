use std::vec;
use std::comm::{SharedChan};

use extra::sort::quick_sort3;

use actor::{spawn_actor_to_chan};

pub enum SieveMsg { Try(int), Gather, }

struct Sieve {
    divisor: int,
    next: Option<~Chan<SieveMsg>>,
    master: SharedChan<int>,
}

fn new_sieve(args: (int, SharedChan<int>)) -> Sieve {
    let (divisor, master) = args;
    Sieve{divisor: divisor, next: None, master: master,}
}

fn on_receive(actor: &mut Sieve, msg: SieveMsg) -> bool {
    match msg {
        Try(number) => {
            if (number % actor.divisor != 0) {
                match actor.next {
                    Some(ref chan) => { chan.send(Try(number)); }
                    None => {
                        let chan =
                            spawn_actor_to_chan((number,
                                                 actor.master.clone()),
                                                new_sieve, on_receive);
                        actor.next = Some(~chan);
                    }
                }
            };
            true //survive
        }
        Gather => {
            actor.master.send(actor.divisor);
            match actor.next {
                Some(ref c) => { c.send(Gather); }
                None => { }
            }
            false //die
        }
    }
}


#[test]
fn test_sieve() {

    // Prepare result
    let (master_port, master_chan) = stream();
    let master = SharedChan::new(master_chan);

    let head =
        spawn_actor_to_chan((2, master.clone()), new_sieve, on_receive);
    for i in range(3, 20) { head.send(Try(i)); }
    head.send(Gather);

    let expected = [2, 3, 5, 7, 11, 13, 17, 19];
    let mut primes = vec::from_fn(8, |_| master_port.recv());
    quick_sort3(primes);
    for (i, j) in expected.iter().zip(primes.iter()) { assert_eq!(i , j); }
}


