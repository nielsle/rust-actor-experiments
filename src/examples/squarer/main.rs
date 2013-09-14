extern mod actor;
extern mod std;

use actor::actor::{Actor, SurviveOrDie, Survive};
use actor::system::{System};

struct Squarer;

impl <C: GenericChan<int>> Actor<int, int, C> for Squarer {
    fn on_receive(&mut self, msg: int, chan: &C) -> SurviveOrDie {
        chan.send(msg * msg);
        Survive
    }
}

fn main() {

    let mut system = System::new();

    let mut squarers =
        do range(0, 6).map |i| {
            let squarer = system.add_actor((), |_| { Squarer });
            squarer.chan.send(i);
            squarer
        };

    let norm2 = squarers.fold(0, |sum, squarer| sum + squarer.port.recv());
    assert_eq!(norm2 , 1 * 1 + 2 * 2 + 3 * 3 + 4 * 4 + 5 * 5);
}

