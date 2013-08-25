use actor::{spawn_actor_to_stream};

struct Squarer {
    chan: Chan<int>,
}

fn new_squarer(args: (int, Chan<int>)) -> Squarer {
    let (_, chan) = args;
    Squarer{chan: chan,}
}

#[test]
fn test_squarer() {

    let results: ~[Port<int>] =
        do [1, 2, 3, 4, 5].map |i| {
            let (port, chan) =
                do spawn_actor_to_stream(0, new_squarer) |actor, msg: int| {
                    actor.chan.send(msg * msg);
                    true
                };

            chan.send(*i);
            port
        };
    let mut norm2 = 0;
    for port in results.iter() { norm2 += port.recv(); }
    assert_eq!(norm2 , 1 * 1 + 2 * 2 + 3 * 3 + 4 * 4 + 5 * 5);
}

