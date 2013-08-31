use std::vec;
use std::comm::{SharedChan};
use extra::sort::quick_sort3;
use actor::{ActorWithChan};

pub enum SieveMsg { 
    Try(int), 
    Gather, 
}

struct Sieve<'self> {
    divisor: int,
    next: Option<Chan<SieveMsg>>,
    master: SharedChan<int>,
}

fn new_sieve(args: (int, SharedChan<int>)) -> Sieve {
    let (divisor, master) = args;
    Sieve{divisor: divisor, next: None, master: master,}
}

fn on_receive(sieve: &mut Sieve, msg: SieveMsg) -> bool {
    match msg {
        Try(number) => {
            if (number % sieve.divisor != 0) {
                match sieve.next {
                    Some(ref chan) => { chan.send(Try(number)); }
                    None => {
                        let actor =
                            ActorWithChan::new((number, sieve.master.clone()),
                                               new_sieve, on_receive);
                        sieve.next = Some(actor.chan);
                    }
                }
            };
            true //survive
        }
        Gather => {
            sieve.master.send(sieve.divisor);
            match sieve.next {
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

    let head = ActorWithChan::new((2, master.clone()), new_sieve, on_receive);
    for i in range(3, 20) { head.chan.send(Try(i)); }
    head.chan.send(Gather);

    let expected = [2, 3, 5, 7, 11, 13, 17, 19];
    let mut primes = vec::from_fn(8, |_| master_port.recv());
    quick_sort3(primes);
    for (i, j) in expected.iter().zip(primes.iter()) { assert_eq!(i , j); }
}


