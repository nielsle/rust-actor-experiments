use actor::{ActorWithStream};

pub enum CounterMsg { Add(int), GetSum, }

pub struct Counter {
    value: int,
}

fn new_counter(value: int) -> Counter {
    Counter{value: value,}
}

#[test]
fn test_counter() {
    let actor =
        do ActorWithStream::new(0, new_counter) 
                       |actor, chan, msg: CounterMsg| {
            match msg {
                Add(value) => actor.value += value,
                GetSum => chan.send(actor.value)
            }
            true
        };

    for i in range(0, 100) { actor.chan.send(Add(i)); }

    actor.chan.send(GetSum);
    let result: int = actor.port.recv();
    assert_eq!(result , 100 * 99 / 2)
}

