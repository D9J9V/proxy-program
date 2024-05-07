#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub struct ProxyMetadata;

impl Metadata for ProxyMetadata {
    type Init = In<ActorId>;
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = ();
}

#[derive(TypeInfo, Encode, Decode)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]

pub enum Action {
    Hello,
    HowAreYou,
    MakeRandomNumber { range: u8 },
}

#[derive(TypeInfo, Encode, Decode)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]

pub enum Event {
    Hello,
    Fine,
    Number(u8),
    MessageSent,
}
