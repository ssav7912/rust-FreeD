use freed::common::*;
use freed::freed::*;

#[test]
fn ExampleProgram() {
    

    let msg = Message::<PositionPollPayload>::new(ALL_CAMERAS);
    let payload = msg.get_payload();
}