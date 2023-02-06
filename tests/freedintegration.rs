use freed::common::*;
use freed::payloads::*;

#[test]
fn ExampleProgram() {
    
    let inpayload = PositionPollPayload::default();
    let msg = Message::new(inpayload, ALL_CAMERAS);
    let payload = msg.get_payload();
}