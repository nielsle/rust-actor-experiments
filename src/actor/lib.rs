#[link(name = "actor",
       vers = "0.0",
       uuid = "8769ffe0-0800-11e3-8ffd-0800200c9a66")];
#[comment = "Rust Actor library"];
#[license = "MIT/ASL2"];
#[crate_type = "lib"];

extern mod extra;

extern mod std;

pub mod actor;

pub mod actorref;

pub mod system;

