A few experiments with concurrency in rust. 

I am trying to learn rust, so helpful comments are very welcome, but so far the code looks like this:

```rust
use std::int;

use actor::{ActorWithStream};

struct Exponent {
    value: uint,
}

#[test]
fn test_exponent() {

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
```
