use std::task::Poll;

use freed::common::*;
use freed::payloads::*;
use rand::{distributions::{Distribution, Standard}, prelude::*, seq::SliceRandom};

#[test]
fn arbitrarydeserialisation() {
    
    let inpayload = PositionPollPayload::default();
    let msg = Message::new(inpayload, ALL_CAMERAS);
    let payload = msg.get_payload();
}


impl Distribution<PollPayload> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PollPayload {
        PollPayload { command: rand::random() }
    }
}

impl Distribution<Commands> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Commands {
        [0x00, 0x01, 0x02, 0x03, 0xD1, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA, 0xDB].choose(&mut rand::thread_rng())
    }
}

///fuzzy serialisation - picks a payload and parameters at random to serialise.
fn arbitrary_serialisation() -> Vec<u8> {
    
}