///A reference to a spawned actorpub trait ActorRef { }
pub trait ActorRef{ }


///Spawned actor reference with port and channel
pub struct ActorRefWithStream<P2, C1> {

    port: P2,
    chan: C1,
}
impl <T1: Send, T2: Send, C1: GenericChan<T1>, P2: GenericPort<T2>> ActorRef
     for ActorRefWithStream<P2, C1> {}



///Spawned actor reference with channel
pub struct ActorRefWithChan<C1>{

    chan: C1,
}
impl<T1: Send, C1: GenericChan<T1>> ActorRef for ActorRefWithChan<C1>{}

///Spawned actor reference with port
pub struct ActorRefWithPort<P2>{

    port: P2,
}
impl<T2: Send, P2: GenericPort<T2>> ActorRef for ActorRefWithPort<P2>{}


///Spawned actor reference with no port and channel
pub struct ActorRefWithoutPortAndChan;
impl ActorRef for ActorRefWithoutPortAndChan{}

