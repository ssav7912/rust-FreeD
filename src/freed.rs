
use std::vec;

use ux::u24;
use ux::i24;

const ALL_CAMERAS: u8 = 0xFF;


#[derive(Copy, Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
///Plain-Old-Data struct that contains payload information for a POSITION_POLL request.
/// Note that most fields are 24 bit (as required by the protocol spec) - this will panic if you 
/// attempt to place too large or small values into it. Use the `u24::new()` (or `i24::new()`) function to generate values
/// from literals or primitive integer types. 
struct PositionPollPayload {
    pitch: i24,
    yaw: i24,
    roll: i24,
    pos_z: i24,
    pos_y: i24,
    pos_x: i24,
    zoom: u24,
    focus: u24,
    reserved: u16
}

trait Serialise {
    ///Generate an arbitrary array of `u8`s.
    fn serialise(self) ->  Vec<u8>;
}

impl Default for PositionPollPayload {
    fn default() -> Self {
        Self { pitch: i24::new(0), yaw: i24::new(0), roll: i24::new(0), 
            pos_z: i24::new(0), pos_y: i24::new(0), pos_x: i24::new(0), 
            zoom: u24::new(0), focus: u24::new(0), reserved: 0 }
    }
}

impl Serialise for PositionPollPayload {
    ///This function serialises a `POSITION_POLL` payload struct into a `u8` bytearray in big endian order.
    /// Intended as the last step before transmitting over Serial or UDP.    
    fn serialise(self) -> Vec<u8> {
        let mut serial: [u8; 26] = [0; 26];
        
        let pitch32 : i32 = self.pitch.into();
        let yaw32: i32 = self.yaw.into();
        let roll32: i32 = self.roll.into();
        let pos_z32: i32 = self.pos_z.into();
        let pos_y32: i32 = self.pos_y.into();
        let pos_x32: i32 = self.pos_x.into();
        let zoom32: u32 = self.zoom.into();
        let focus32: u32 = self.focus.into();

        let i32fields: [i32; 6] = [pitch32, yaw32, roll32, pos_z32, pos_y32, pos_x32]; 
        let u32fields: [u32; 2] = [zoom32, focus32];

        for (index, field) in i32fields.iter().enumerate() {
            let [__a, b, c, d] = field.to_be_bytes();
            
            
            serial[(index*3) + 0] = b;
            serial[(index*3) + 1] = c;
            serial[(index*3) + 2] = d;
        }
        
        for (index, field) in u32fields.iter().enumerate() {
            let [__a, b, c, d] = field.to_be_bytes();
            
            serial[(index*3) + i32fields.len()*3 + 0] = b;
            serial[(index*3) + i32fields.len()*3 + 1] = c;
            serial[(index*3) + i32fields.len()*3 + 2] = d;
        }

        let [a, b] = self.reserved.to_be_bytes();
        serial[serial.len() - 2] = a;
        serial[serial.len() - 1] = b;

        let constserial = serial.to_vec();

        return constserial;
    }

}

#[derive(Copy, Clone)]
struct Message<T: Serialise + Default> {
    command: Commands,
    cameraid: u8,
    payload: T,
    checksum: u8
}

impl<T: Serialise + Default + Copy + Clone> Serialise for Message<T> {

    
    fn serialise(self) -> Vec<u8> {
        
        let payloaddata: Vec<u8> = self.payload.serialise();
        let mut output = vec![self.command as u8, self.cameraid];
        output.extend(payloaddata);
        let checksum = generate_checksum(&output);
        output.push(checksum);
        return output;       
    }
}


impl<T: Serialise + Default> Message<T> {
    pub fn new(command: Commands, cameraid: u8) -> Message<T> {
        
        
        let payload: T = match command {
            Commands::STREAM_MODE_START => todo!(),
            Commands::STREAM_MODE_STOP => todo!(),
            Commands::FREEZE_MODE_START => todo!(),
            Commands::POSITION_POLL => <T as Default>::default(), 
            Commands::SYSTEM_STATUS => todo!(),
            Commands::SYSTEM_PARAMS => todo!(),
            Commands::FIRST_TARGET => todo!(),
            Commands::NEXT_TARGET => todo!(),
            Commands::FIRST_IMAGE => todo!(),
            Commands::NEXT_IMAGE => todo!(),
            _ => todo!()
        };

        Message {command: command, cameraid: cameraid, payload: payload, checksum: 0 }
    }
    //reset this to return a mutable reference.
    ///Gets a copy of the payload struct
    pub fn get_payload(self) -> T {
        return self.payload;
        
    }

    //Sets the payload struct. Note that the new payload must be the same type as the one retrieved from the message.
    pub fn set_payload(&mut self, payload: T) {
        self.payload = payload;
    } 


}

//pull this out to public function? Doesn't depend on type
fn generate_checksum(serialised: &[u8]) -> u8 {
    let mut checksum: u8 = 0x40;
    for byte in serialised{
        checksum = (checksum - byte)%255;
    }

    return checksum; //spec says 256.. verify.
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
enum Commands {
    STREAM_MODE_STOP = 0x00,
    STREAM_MODE_START = 0x01,
    FREEZE_MODE_STOP = 0x02,
    FREEZE_MODE_START = 0x03,
    POSITION_POLL = 0xD1,
    SYSTEM_STATUS = 0xD2,
    SYSTEM_PARAMS = 0xD3,
    FIRST_TARGET = 0xD4,
    NEXT_TARGET = 0xD5,
    FIRST_IMAGE = 0xD6,
    NEXT_IMAGE = 0xD7,
    NEXT_EEPROM = 0xD8,
    CAMERA_CALIBRATION = 0xDA,
    DIAGNOSTIC_MODE = 0xDB
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn u24_test_overflow() {
        let x: u24 = u24::new(0xFFFFFF);
        let y: u24 = u24::new(0x000001);

        let _ = x + y; //doesn't do compile time check
    }

    #[test]
    #[should_panic]
    fn i24_test_overflow() {
        let x: i24 = i24::new(0xFFFFFF);
        let y: i24 = i24::new(0x000001);

        let _ = x + y;
    }


    /*
     * This tests that POSITION_POLL payload structs are serialised to u8 arrays correctly:
     * - Values appear in the right places (sequential C-struct order)
     * - Reconstructing the array preserves values.
    */
    #[test]
    fn payload_serialise() {
        let testpitch : i32 = -1000; //signed negative number for i24
        let testfocus : u32 = 0xAABBCC; //large unsigned number for u24

        let payload = PositionPollPayload {pitch: i24::new(testpitch), yaw: i24::new(0), roll: i24::new(0), 
            pos_z: i24::new(0), pos_y: i24::new(0), pos_x: i24::new(0), 
            zoom: u24::new(0), focus: u24::new(testfocus), reserved: 0};
        
        let serial = payload.serialise();
        println!("{:?}", serial);

        let testpitchbytes = testpitch.to_be_bytes();
        
        //byte values in the array at the expected positions match those generated by to_be_bytes()
        assert_eq!(serial[0], testpitchbytes[1]);
        assert_eq!(serial[1], testpitchbytes[2]);
        assert_eq!(serial[2], testpitchbytes[3]);

        //reconstructing pitch from array bytes preserves semantics (=-1000)
        let mut pitchreconstruct: [u8; 4] = [0xFF;4]; //hack sign-extension in. Not in the spirit of testing. Not super important?
        let pitchslice = &serial[0..3];
        assert_eq!(pitchslice.len(), 3);
        pitchreconstruct[1..].clone_from_slice(pitchslice);
        assert_eq!(i32::from_be_bytes(pitchreconstruct), testpitch);
        
        let testfocusbytes = testfocus.to_be_bytes();

        //byte values in the array at the expected positions match those generated by to_be_bytes()
        assert_eq!(serial[serial.len() - 2 - 3], testfocusbytes[1]);
        assert_eq!(serial[serial.len() - 2 - 2], testfocusbytes[2]);
        assert_eq!(serial[serial.len() - 2 - 1], testfocusbytes[3]);

        //reconstructing focus from array values preserves semantics (0xAABBCC)
        let mut finalvals : [u8; 4] = [0; 4];
        let slice = &serial[(serial.len() - 2 - 3)..(serial.len()-2)];
        assert_eq!(slice.len(), 3);
        finalvals[1..].clone_from_slice(slice);
        assert_eq!(u32::from_be_bytes(finalvals), testfocus);
    }

    #[test]
    fn message_new() {
        let message = Message::<PositionPollPayload>::new(Commands::POSITION_POLL, ALL_CAMERAS);
        
        assert_eq!(message.command as u8, Commands::POSITION_POLL as u8);
        assert_eq!(message.cameraid, ALL_CAMERAS);
        
        let payload: PositionPollPayload = message.get_payload();
        let testpayload: PositionPollPayload = PositionPollPayload::default();
        assert_eq!(payload, testpayload);
    }

    #[test]
    fn message_payload_get_and_set() {
        let testpitch = i24::new(-1000);
        let testzoom = u24::new(25000);

        let mut message = Message::<PositionPollPayload>::new(Commands::POSITION_POLL, ALL_CAMERAS);
        
        let mut payload: PositionPollPayload = message.get_payload();
        let testpayload: PositionPollPayload = PositionPollPayload::default();
        assert_eq!(payload, testpayload);

        payload.pitch = testpitch;
        payload.zoom = testzoom;

        message.set_payload(payload);

        let newpayload = message.get_payload();
        println!("{:?}", newpayload);
        assert_eq!(newpayload.pitch, testpitch);
        assert_eq!(newpayload.zoom, testzoom);
    }

    #[test]
    fn checksum_simple() {
        let array: [u8; 5] = [1, 2, 3, 4, 5];
        assert_eq!(generate_checksum(&array), 49);
    }

    #[test]
    fn message_serialise() {
        let testpitch = i24::new(-1000);
        let testzoom = u24::new(25000);

        let mut message: Message<PositionPollPayload> = Message::new(Commands::POSITION_POLL, ALL_CAMERAS);

        let mut payload = message.get_payload();
        payload.pitch = testpitch;
        payload.zoom = testzoom;

        message.set_payload(payload);

        let serial = message.serialise();
        println!("{:?}",serial);
        
        assert_eq!(serial.len(), 29);
        assert_eq!(serial[0], Commands::POSITION_POLL as u8);
        assert_eq!(serial[1], ALL_CAMERAS);

        assert_eq!(serial[serial.len()-1], generate_checksum(&serial[..serial.len()-1]));

    }
}
