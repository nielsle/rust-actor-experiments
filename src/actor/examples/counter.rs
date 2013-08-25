use actor::{spawn_actor_to_stream};

pub enum CounterMsg { Add(int), GetSum, }

pub struct Counter {
    value: int,
    chan: Chan<int>,
}

fn new_counter(args: (int, Chan<int>)) -> Counter {
    let (value, chan) = args;
    Counter{value: value, chan: chan,}
}

#[test]
fn test_counter() {
    let (port, chan) =
        do spawn_actor_to_stream(0, new_counter) |actor, msg: CounterMsg| {
            match msg {
                Add(value) => actor.value += value,
                GetSum => actor.chan.send(actor.value)
            }
            true
        };

    for i in range(0, 100) { chan.send(Add(i)); }

    chan.send(GetSum);
    let result: int = port.recv();
    assert_eq!(result , 100 * 99 / 2)
}

