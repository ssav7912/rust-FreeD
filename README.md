# Free-D Protocol Rust Implementation 

This aims to provide a library of all the types used by the FreeD protocol, in rich Rust semantics, for interacting with free-d hardware or simulating it for your own implementations.

I found that there were very few public implementations of the Free-D protocol available, and none that implemented the entire protocol.


Current Payload implementations, as per the spec:
- `PollPayload`
- `PositionPollPayload`
- `SystemStatusPayload`
- `SystemControlPayload`
- `TargetDataPayload`
- `ImageDataPayload`
- `EEPROMDataPayload`
- `EEPROMDataRequestPayload`
- `CameraCalibrationPayload`
- `DiagnosticModePayload`

These are designed to be wrapped in a `Message<T: Serialise + Deserialise>` struct, which appends the command tag and camera information, alongside a checksum before serialisation. Likewise deserialised data is returned as a `Message<T>`, which the payload may be extracted from.

There is also a simple `Serialise` and `Deserialise` trait interface implemented for all payloads, so that data may be sent over UDP or
serial interfaces.

```rust
//Example sending data.

//set up a position packet for a camera
let payload: PositionPollPayload = PositionPollPayload {
    pitch: i24::new(-1000),
    yaw: i24::new(0),
    roll: i24::new(0),
    pos_z: i24::new(150500),
    pos_y: i24::new(0),
    pos_x: i24::new(0),
    zoom: u24::new(100),
    focus: u24::new(4096),
    userdefined: 0,
}

//wrap the payload in a Message struct, which automatically appends the command type & checksum
let message: Message<PositionPollPayload> = Message::new(payload, ALL_CAMERAS);

//serialise to an array
let serial: Vec<u8> = message.serialise();

//send the data!
Send(serial, port, address);
```
Deserialising data to Payload types is a little more complicated, requiring a match on the command byte in the incoming message. I am open to suggestions or PRs that will improve these ergonomics. 
