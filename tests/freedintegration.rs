use freed::common::*;
use freed::freed::*;

#[test]
fn ExampleProgram() {
    
    let inpayload = PositionPollPayload::default();
    let msg = Message::new(inpayload, ALL_CAMERAS);
    let payload = msg.get_payload();
}