extern mod actor;
extern mod std;

use actor::actor::{Actor, SurviveOrDie, Survive};
use actor::system::{System};

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

fn main() {
    let mut system = System::new();
    let counter = system.add_actor(0, |value| Counter{value: value,});

    for i in range(0, 100) { counter.chan.send(Add(i)); }
    counter.chan.send(GetSum);

    let result: int = counter.port.recv();
    assert_eq!(result , 100 * 99 / 2)
}

