#![allow(dead_code)]

use crate::common::*;

use std::alloc::System;
use std::vec;

use ux::i24;
use ux::u24;



///Convenience enum for all the payload types. 
enum Payloads {
    PollPayload(PollPayload),
    PositionPollPayload(PositionPollPayload),
    SystemStatusPayload(SystemStatusPayload),
    SystemControlPayload(SystemControlPayload),
    TargetDataPayload(TargetDataPayload),
    ImageDataPayload(ImageDataPayload),
    EEPROMDataPayload(EEPROMDataPayload),
    EEPROMDataRequestPayload(EEPROMDataRequestPayload),
    CameraCalibrationPayload(CameraCalibrationPayload),
    DiagnosticModePayload(DiagnosticModePayload),
}



#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
///Struct for a simple poll command, corresponding to free-d command `0xD1`
/// 
///This is used by the free-d protocol to send poll commands to a free-d unit, which
/// should return back a payload message corresponding to the desired command.
/// 
/// ```rust
/// let payload: PollPayload = PollPayload {command: Commands::POSITION_POLL}
/// 
/// //serialise the data to send it
/// let data = payload.serialise();
/// ```
pub struct PollPayload {
    pub command: Commands,
}

impl Default for PollPayload {
    fn default() -> Self {
        return PollPayload {
            command: Commands::SYSTEM_STATUS,
        };
    }
}

impl Serialise for PollPayload {
    const COMMAND: Commands = Commands::POSITION_POLL; //unused?
    fn serialise(self) -> Vec<u8> {
        return vec![self.command as u8];
    }

}

impl Deserialise for PollPayload {

    fn deserialise(array: &[u8]) -> Result<PollPayload, DeserialiseError> {
        let command: Commands = match array[0].try_into() {
            Ok(x) => x,
            Err(x) => return Err(DeserialiseError { description: x }),
        };

        Ok(PollPayload { command: command })
    }
}



///System Status Payload. Typically reported by the free-d unit in response to a
/// `0xD2` `SYSTEM_STATUS` poll. 
/// 
/// `switchsetting` - see `SwitchSettingFlags`. Describes the status of the free-d switch
/// 
/// `ledindication` - see `LEDFlags`. Reports the status of the LED indicators on the free-d unit
/// 
/// `systemstatus` - see `SystemStatus`. Reports the internal state of the free-d unit.
/// 
/// `cpufirmwareversion`, `pldfirmwareversion`, `dspsoftwareversion`. Firmware version values of various hardware components of the
/// free-d unit in Binary Coded Decimal, where there is an implied decimal point between the two digits. e.g. `0x12 = 1.2`. 
/// 
/// `dspstatus` - Reports either the number of iterations required to compute the camera's position, or an error as defined in `DSPError`
/// 
/// `numtargetsseen` - The number of targets detected by the hardware.
/// 
/// `numtargetsidentified` - The number of targets identified (bar codes read)
/// 
/// `numtargetsused` - The number of targets used (identified and in database)
/// 
/// `rmserror` - RMS error expressed in pixels, where 1 unit = 1/32768 pixels. Bits 22 to 15 decide the integer part of the value,
/// while bits 14 to 0 decide the fractional part of the value. 
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SystemStatusPayload {
    pub switchsetting: SwitchSettingFlags,
    pub ledindication: LEDFlags,
    pub systemstatus: SystemStatus,
    pub cpufirmwareversion: u8,
    pub pldfirmwareversion: u8,
    pub dspsoftwareversion: u8,
    pub dspstatus: Result<i8, DSPError>,
    pub numtargetsseen: u8,
    pub numtargetsidentified: u8,
    pub numtargetsused: u8,
    pub rmserror: u24,
}

impl Default for SystemStatusPayload {
    fn default() -> Self {
        Self {
            switchsetting: SwitchSettingFlags::default(),
            ledindication: LEDFlags::default(),
            systemstatus: SystemStatus::default(),
            cpufirmwareversion: u8::default(),
            pldfirmwareversion: u8::default(),
            dspsoftwareversion: u8::default(),
            dspstatus: Ok(i8::default()),
            numtargetsseen: u8::default(),
            numtargetsidentified: u8::default(),
            numtargetsused: u8::default(),
            rmserror: u24::default(),
        }
    }
}

#[allow(non_snake_case)]
impl Serialise for SystemStatusPayload {
    fn serialise(self) -> Vec<u8> {
        let dspstatusserial = match self.dspstatus {
            Ok(x) => x,
            Err(DSPError) => DSPError as i8,
        };

        let rmserror32: u32 = self.rmserror.into();
        let [__a, b, c, d] = rmserror32.to_be_bytes();

        vec![
            SwitchSettingFlags::bits(&self.switchsetting),
            LEDFlags::bits(&self.ledindication),
            self.systemstatus as u8,
            self.cpufirmwareversion,
            self.pldfirmwareversion,
            self.dspsoftwareversion,
            dspstatusserial as u8,
            self.numtargetsseen,
            self.numtargetsidentified,
            self.numtargetsused,
            b,
            c,
            d,
        ]
    }
}

impl Deserialise for SystemStatusPayload {
    fn deserialise(array: &[u8]) -> Result<Self, DeserialiseError> where Self: Sized {
        const SIZE: usize = 16 -3;
        const PAYLOADID: &str = "SystemStatusPayload";
        if array.len() != SIZE {
            return Err(DeserialiseError {description: format!("Misformed data - the array must be exactly {} bytes for the payload type: {}",
            SIZE, PAYLOADID)})
        }

        let switchsetting: SwitchSettingFlags = match array[0].try_into() {
            Ok(x) => x,
            Err(x) => return Err(DeserialiseError { description: x })
        };

        let ledindication: LEDFlags = match array[1].try_into() {
            Ok(x) => x,
            Err(x) => return Err(DeserialiseError { description: x })
        };

        let systemstatus: SystemStatus = match array[2].try_into() {
            Ok(x) => x,
            Err(x) => return Err(DeserialiseError { description: x })
        };
        let dspcode = array[6] as i8;
        
        let dspstatus: Result<i8, DSPError> = match dspcode {
            x if x >= 0 => Ok(x), 
            x if x < 0 => Err(match x.try_into() 
            {
                Ok(x) => x, 
                Err(x) => return Err(DeserialiseError { description: x })}),
            _ => unreachable!()};

            
         Ok(SystemStatusPayload {
            switchsetting: switchsetting,
            ledindication: ledindication,
            systemstatus: systemstatus,
            cpufirmwareversion: array[3],
            pldfirmwareversion: array[4],
            dspsoftwareversion: array[5],
            dspstatus: dspstatus,
            numtargetsseen: array[7],
            numtargetsidentified: array[8],
            numtargetsused: array[9], 
            rmserror: u24::new(u32::from_be_bytes(array[10..].try_into().expect("Last three bytes")))
        })

        
        
     }
}

///Struct used to transfer control parameters either to or from the free-d unit. A `0xD3` `SYSTEM_PARAMS` poll payload message will
/// request this from the free-d unit. 
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct SystemControlPayload {
    pub studioid: u8,
    pub smoothing: u8,
    pub maxasymmetry: u8,
    pub halfboxwidth: u8,
    pub blackvidthreshold: u8,
    pub whitevidthreshold: u8,
    pub blackvidclip: u8,
    pub whitevidclip: u8,
    pub maxblackpixels: u8,
    pub minwhitepixels: u8,
}

impl Serialise for SystemControlPayload {
    const COMMAND: Commands = Commands::SYSTEM_PARAMS;
    fn serialise(self) -> Vec<u8> {
        vec![
            self.studioid,
            self.smoothing,
            self.maxasymmetry,
            self.halfboxwidth,
            self.blackvidthreshold,
            self.whitevidthreshold,
            self.blackvidclip,
            self.whitevidclip,
            self.maxblackpixels,
            self.minwhitepixels,
        ]
    }
}

impl Deserialise for SystemControlPayload {
    fn deserialise(array: &[u8]) -> Result<Self, DeserialiseError> where Self: Sized {
        const SIZE: usize = 13 -3;
        const PAYLOADID: &str = "SystemControlPayload";
        if array.len() != SIZE {
            return Err(DeserialiseError { description: DeserialiseError::length_template(SIZE, PAYLOADID) });
        }

        Ok(SystemControlPayload { 
            studioid: array[0], 
            smoothing: array[1], 
            maxasymmetry: array[2], 
            halfboxwidth: array[3], 
            blackvidthreshold: array[4], 
            whitevidthreshold: array[5], 
            blackvidclip: array[6], 
            whitevidclip: array[7], 
            maxblackpixels: array[8], 
            minwhitepixels: array[9] })
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct TargetDataPayload {
    pub studioid: u8,
    pub targetnumber: u16,
    pub targetx: i24,
    pub targety: i24,
    pub targetz: i24,
    pub targetflags: i24,
}

impl Serialise for TargetDataPayload {
    const COMMAND: Commands = Commands::FIRST_TARGET;
    fn serialise(self) -> Vec<u8> {
        let targetarray = [self.targetx, self.targety, self.targetz, self.targetflags];
        let mut serial = vec![self.studioid];
        serial.extend(self.targetnumber.to_be_bytes().iter());
        serial.extend(serialisei24array(&targetarray));

        return serial;
    }
}

impl Deserialise for TargetDataPayload {
    fn deserialise(array: &[u8]) -> Result<Self, DeserialiseError> where Self: Sized {
        const SIZE: usize = 18-3;
        const PAYLOADID: &str = "TargetDataPayload";
        if array.len() != SIZE {
            return Err(DeserialiseError { description: DeserialiseError::length_template(SIZE, PAYLOADID) })
        }

        Ok(TargetDataPayload { 
            studioid: array[0] as u8, 
            targetnumber: u16::from_be_bytes(array[1..3].try_into().unwrap()), 
            targetx: i24::new(i32::from_be_bytes(array[3..6].try_into().unwrap())), 
            targety: i24::new(i32::from_be_bytes(array[6..9].try_into().unwrap())), 
            targetz: i24::new(i32::from_be_bytes(array[9..12].try_into().unwrap())), 
            targetflags: i24::new(i32::from_be_bytes(array[12..15].try_into().unwrap())) })
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct ImageDataPayload {
    pub targetindex: u8,
    pub targetnum: u16,
    pub targetx: i24,
    pub targety: i24,
    pub xerror: i24,
    pub yerror: i24,
}

impl Serialise for ImageDataPayload {
    const COMMAND: Commands = Commands::FIRST_IMAGE;
    fn serialise(self) -> Vec<u8> {
        let targettuple = [self.targetx, self.targety, self.xerror, self.yerror];
        let mut serial = vec![self.targetindex];
        serial.extend(self.targetnum.to_be_bytes().iter());
        serial.extend(serialisei24array(&targettuple));
        return serial;
    }
}

impl Deserialise for ImageDataPayload {
    fn deserialise(array: &[u8]) -> Result<Self, DeserialiseError> where Self: Sized {
        const SIZE: usize = 18 - 3;
        const PAYLOADID: &str = "ImageDataPayload";
        if array.len() != SIZE { 
            return Err(DeserialiseError { description: DeserialiseError::length_template(SIZE, PAYLOADID) })
        }

        Ok(ImageDataPayload { 
            targetindex: array[0], 
            targetnum: u16::from_be_bytes(array[1..2].try_into().unwrap()), 
            targetx: i24::new(i32::from_be_bytes(array[2..5].try_into().unwrap())), 
            targety: i24::new(i32::from_be_bytes(array[5..8].try_into().unwrap())),
             xerror: i24::new(i32::from_be_bytes(array[8..11].try_into().unwrap())), 
             yerror: i24::new(i32::from_be_bytes(array[11..15].try_into().unwrap())) })
    }
}

#[allow(non_snake_case)]
#[derive(Copy, Clone, Default, Debug)]
pub struct EEPROMDataPayload {
    pub EEPROMaddress: u16,
    pub EEPROMdata: [u8; 16],
}

impl Serialise for EEPROMDataPayload {
    const COMMAND: Commands = Commands::EEPROM_DATA;
    fn serialise(self) -> Vec<u8> {
        let mut serial = self.EEPROMaddress.to_be_bytes().to_vec();
        let databytes = self.EEPROMdata;
        serial.extend(databytes.iter());

        return serial;
    }
}

impl Deserialise for EEPROMDataPayload {
    fn deserialise(array: &[u8]) -> Result<Self, DeserialiseError> where Self: Sized {
        const SIZE: usize = 21 - 3;
        const PAYLOADID: &str = "EEPROMDataPayload";

        if array.len() != SIZE {
            return Err(DeserialiseError { description: DeserialiseError::length_template(SIZE, PAYLOADID) })
        }

        Ok(EEPROMDataPayload { 
            EEPROMaddress: u16::from_be_bytes(array[0..2].try_into().unwrap()), 
            EEPROMdata: array[2..].try_into().unwrap()})
    }
}

#[allow(non_snake_case)]
#[derive(Copy, Clone, Default, Debug)]
pub struct EEPROMDataRequestPayload {
    pub EEPROMaddress: u16,
}

impl Serialise for EEPROMDataRequestPayload {
    const COMMAND: Commands = Commands::REQUEST_EEPROM;
    fn serialise(self) -> Vec<u8> {
        let bytes = self.EEPROMaddress.to_be_bytes();
        return bytes.to_vec();
    }
}

impl Deserialise for EEPROMDataRequestPayload {
    fn deserialise(array: &[u8]) -> Result<Self, DeserialiseError> where Self: Sized {
        const SIZE: usize = 5 - 3;
        const PAYLOADID: &str = "EEPROMDataRequestPayload";

        if array.len() != SIZE {
            return Err(DeserialiseError { description: DeserialiseError::length_template(SIZE, PAYLOADID) })
        }

        Ok(EEPROMDataRequestPayload { EEPROMaddress: u16::from_be_bytes(array.try_into().unwrap()) })
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct CameraCalibrationPayload {
    pub lenscentrex: i24,
    pub lenscentrey: i24,
    pub lensscalex: i24,
    pub lensscaley: i24,
    pub lensdistortiona: i24,
    pub lensdistortionb: i24,
    pub xoffset: i24,
    pub yoffset: i24,
    pub zoffset: i24,
}

impl Serialise for CameraCalibrationPayload {
    const COMMAND: Commands = Commands::CAMERA_CALIBRATION;
    fn serialise(self) -> Vec<u8> {
        let order = [
            self.lenscentrex,
            self.lenscentrey,
            self.lensscalex,
            self.lensscaley,
            self.lensdistortiona,
            self.lensdistortionb,
            self.xoffset,
            self.yoffset,
            self.zoffset,
        ];

        return serialisei24array(&order);
    }
}

impl Deserialise for CameraCalibrationPayload {
    fn deserialise(array: &[u8]) -> Result<Self, DeserialiseError> where Self: Sized {
        const SIZE: usize = 30 - 3;
        const PAYLOADID: &str = "CameraCalibrationPayload";

        if array.len() != SIZE {
            return Err(DeserialiseError { description: DeserialiseError::length_template(SIZE, PAYLOADID) })
        }

        Ok(CameraCalibrationPayload { 
            lenscentrex: i24::new(i32::from_be_bytes(array[0..3].try_into().unwrap())), 
            lenscentrey: i24::new(i32::from_be_bytes(array[3*1..3*2].try_into().unwrap())), 
            lensscalex: i24::new(i32::from_be_bytes(array[3*2..3*3].try_into().unwrap())), 
            lensscaley: i24::new(i32::from_be_bytes(array[3*3..3*4].try_into().unwrap())), 
            lensdistortiona: i24::new(i32::from_be_bytes(array[3*4..3*5].try_into().unwrap())), 
            lensdistortionb: i24::new(i32::from_be_bytes(array[3*5..3*6].try_into().unwrap())), 
            xoffset: i24::new(i32::from_be_bytes(array[3*6..3*7].try_into().unwrap())), 
            yoffset: i24::new(i32::from_be_bytes(array[3*7..3*8].try_into().unwrap())), 
            zoffset: i24::new(i32::from_be_bytes(array[3*8..3*9].try_into().unwrap())) })
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct DiagnosticModePayload {
    pub diagnosticflag: DiagnosticModes,
}

impl Serialise for DiagnosticModePayload {
    const COMMAND: Commands = Commands::DIAGNOSTIC_MODE;
    fn serialise(self) -> Vec<u8> {
        return vec![self.diagnosticflag as u8];
    }
}

impl Deserialise for DiagnosticModePayload {
    fn deserialise(array: &[u8]) -> Result<Self, DeserialiseError> where Self: Sized {
        const SIZE: usize = 4 - 3;
        const PAYLOADID: &str = "DiagnosticModePayload";

        if array.len() != SIZE {
            return Err(DeserialiseError { description: DeserialiseError::length_template(SIZE, PAYLOADID) })
        }

        Ok(DiagnosticModePayload { diagnosticflag: match array[0].try_into() {
            Ok(x) => x, 
            Err(x) => return Err(DeserialiseError{description: x})} })

    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
///Struct containing camera location information for a `0xD1` `POSITION_POLL` request. 
/// Note that most fields are 24 bit (as required by the protocol spec) - this will panic if you
/// attempt to place too large or small values into it. Use the `u24::new()` (or `i24::new()`) function to generate values
/// from literals or primitive integer types.

//TODO: Compile time checks?
pub struct PositionPollPayload {
    pub pitch: i24,
    pub yaw: i24,
    pub roll: i24,
    pub pos_z: i24,
    pub pos_y: i24,
    pub pos_x: i24,
    pub zoom: u24,
    pub focus: u24,
    pub userdefined: u16, //arbitrary [u8; 2]?
}

impl Default for PositionPollPayload {
    fn default() -> Self {
        Self {
            pitch: i24::new(0),
            yaw: i24::new(0),
            roll: i24::new(0),
            pos_z: i24::new(0),
            pos_y: i24::new(0),
            pos_x: i24::new(0),
            zoom: u24::new(0),
            focus: u24::new(0),
            userdefined: 0,
        }
    }
}

impl Serialise for PositionPollPayload {
    const COMMAND: Commands = Commands::POSITION_POLL;
    ///This function serialises a `POSITION_POLL` payload struct into a `u8` bytearray in big endian order.
    /// Intended as the last step before transmitting over Serial or UDP.    
    fn serialise(self) -> Vec<u8> {
        let mut serial: [u8; 26] = [0; 26];

        let pitch32: i32 = self.pitch.into();
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

            serial[(index * 3) + 0] = b;
            serial[(index * 3) + 1] = c;
            serial[(index * 3) + 2] = d;
        }

        for (index, field) in u32fields.iter().enumerate() {
            let [__a, b, c, d] = field.to_be_bytes();

            serial[(index * 3) + i32fields.len() * 3 + 0] = b;
            serial[(index * 3) + i32fields.len() * 3 + 1] = c;
            serial[(index * 3) + i32fields.len() * 3 + 2] = d;
        }

        let [a, b] = self.userdefined.to_be_bytes();
        serial[serial.len() - 2] = a;
        serial[serial.len() - 1] = b;

        let constserial = serial.to_vec();

        return constserial;
    }
}

impl Deserialise for PositionPollPayload {
    // type Output = PositionPollPayload;

    //only way to recover from these is to ask for another message
    fn deserialise(array: &[u8]) -> Result<Self, DeserialiseError> {
        const SIZE: usize = 29 - 3;
        const PAYLOADID: &str = "PositionPollPayload";
        if array.len() < SIZE || array.len() > SIZE {
            return Err(DeserialiseError {
                description: format!(
                    "Misformed data - the array must be exactly {} bytes for the payload type: {}",
                    SIZE, PAYLOADID
                )
                .to_string(),
            });
        };

        let payload = PositionPollPayload {
            pitch: i24::new(i32::from_be_bytes(array[..3].try_into().unwrap())), //can we assume this will never panic as we've verified the array size?
            yaw: i24::new(i32::from_be_bytes(array[3..3 * 2].try_into().unwrap())),
            roll: i24::new(i32::from_be_bytes(array[3 * 2..3 * 3].try_into().unwrap())),
            pos_z: i24::new(i32::from_be_bytes(array[3 * 3..3 * 4].try_into().unwrap())),
            pos_y: i24::new(i32::from_be_bytes(array[3 * 4..3 * 5].try_into().unwrap())),
            pos_x: i24::new(i32::from_be_bytes(array[3 * 5..3 * 6].try_into().unwrap())),
            zoom: u24::new(u32::from_be_bytes(array[3 * 6..3 * 7].try_into().unwrap())),
            focus: u24::new(u32::from_be_bytes(array[3 * 7..3 * 8].try_into().unwrap())),
            userdefined: u16::from_be_bytes(array[3 * 8..].try_into().unwrap()),
        };

        return Ok(payload);
    }
}

impl TryFrom<Payloads> for PositionPollPayload {
    type Error = DeserialiseError;

    fn try_from(value: Payloads) -> Result<Self, Self::Error> {
        return match value {
            Payloads::PositionPollPayload(x) => Ok(x),
            _=> Err(DeserialiseError { description: "Not a position poll payload".to_string() })
        }
    }
}

fn deserialise<T: Serialise + Default + Deserialise>(data: &[u8]) -> Result<Message<T>, DeserialiseError> {
    if data.len() < 4 {
        return Err(DeserialiseError {
            description: "Misformed data - the protocol defines no messages smaller than 4 bytes"
                .to_string(),
        });
    }

    let checksum = data[data.len()-1];

    if generate_checksum(&data) != checksum {
        return Err(DeserialiseError {
            description: "Misformed data - checksum is incorrect.".to_string(),
        });
    }
    let cameraid = data[1];
    let command: Commands = match data[0].try_into() {
        Ok(x) => x,
        Err(x) => return Err(DeserialiseError { description: x }),
    };

    let payload: T = T::deserialise(&data[3..data.len()-1]).unwrap();

    // let payload = match command {
    //     Commands::POSITION_POLL => Payloads::PositionPollPayload(PositionPollPayload::deserialise(
    //         &data[3..data.len() - 1],
    //     )?),
    //     _ => todo!(),
    // };
    return Ok(Message::<T> {command: command, cameraid: cameraid, payload: payload, checksum: checksum})
}


///Message type for serialising and deserialising protocol messages. 
/// Once you have selected and filled out a payload struct, it can be wrapped
/// into a message with `Message::new()`. This will instantiate a message with
/// the correct command, cameraid and checksum. The message may then be serialised
/// into a `Vec<u8>` with `Message::serialise(self)`, which can then be sent over UDP or similar.
/// 
/// Certain structs, like `ImageDataPayload` and `TargetDataPayload` may be used with
/// more than one command. For these types extra methods are implemented to allow you to
/// change the command. The default command used is `FIRST_[type]`. 
#[derive(Copy, Clone, Debug)]
pub struct Message<T: Serialise + Default> {
    command: Commands,
    pub cameraid: u8,
    pub payload: T,
    checksum: u8,
}

impl<T: Serialise + Default + Copy + Clone> Serialise for Message<T> {
    const COMMAND: Commands = Commands::POSITION_POLL; //unused? Feasibly could pass Message as T :|
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
    pub fn new(payload: T, cameraid: u8) -> Message<T> {
        let payload: T = payload;
        let command: Commands = <T as Serialise>::COMMAND;

        
        Message {
            command: command,
            cameraid: cameraid,
            payload: payload,
            checksum: 0,
        }
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
    let mut checksum: u16 = 0x40;
    for byte in serialised {
        let upcast = *byte as u16;
        checksum = (checksum.wrapping_sub(upcast)) % 256;
    }

    return (checksum % 256).try_into().unwrap(); //spec says 256.. verify.
}

impl Message<TargetDataPayload> {
    fn set_command(&mut self, command: Commands) {
        if command != Commands::FIRST_TARGET || command != Commands::NEXT_TARGET {
            panic!(
                "Only FIRST_TARGET or NEXT_TARGET commands may be used with a TargetDataPayload"
            );
        }
        self.command = command;
    }
}

impl Message<ImageDataPayload> {
    fn set_command(&mut self, command: Commands) {
        if command != Commands::FIRST_IMAGE || command != Commands::NEXT_IMAGE {
            panic!("Only FIRST_IMAGE or NEXT_IMAGE commands may be used with an ImageDataPayload");
        }
        self.command = command;
    }
}

fn is_valid_message<T: Serialise>(array: &[u8]) -> bool {
    let command: Result<Commands, String> = array[0].try_into();

    let command_correct: bool = match command {
        Ok(x) => x == T::COMMAND,
        Err(_) => false,
    };

    //when is it possible for different arrays to have the same checksum?
    let checksum_correct = generate_checksum(&array) == array[array.len() - 1];

    return command_correct && checksum_correct;
}

fn serialisei24array(array: &[ux::i24]) -> Vec<u8> {
    let mut serial = Vec::<u8>::new();

    for element in array {
        let elementi32: i32 = (*element).into();

        serial.extend_from_slice(&elementi32.to_be_bytes()[1..]);
    }
    return serial;
}

#[cfg(test)]
mod test {
    use std::task::Poll;

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

    #[test]
    fn serialisei24array_test() {
        let a: i32 = 2555;
        let b: i32 = -1000;

        let array = [i24::new(a), i24::new(0), i24::new(b)];
        let serial = serialisei24array(&array);

        let mut value1: [u8; 4] = [0x00; 4];
        value1[1..].clone_from_slice(&serial[0..3]);

        assert_eq!(serial.len(), 3 * 3);
        assert_eq!(i32::from_be_bytes(value1), a)
    }

    #[test]
    fn pollpayload_serialise() {
        let payload = PollPayload {
            command: Commands::EEPROM_DATA,
        };

        let serial = payload.serialise();
        assert_eq!(serial.len(), 1);
        assert_eq!(serial[0], payload.command as u8);
    }

    #[test]
    fn pollpayload_deserialise() {
        let payload = PollPayload {
            command: Commands::EEPROM_DATA,
        };

        let serial = payload.serialise();

        if let Ok(deserialisedpayload) = PollPayload::deserialise(&serial) {
            assert_eq!(payload, deserialisedpayload);
            
        }
        else {
            //failed to deserialise
            assert!(false);
        }
    }

    #[allow(non_snake_case)]
    #[test]
    fn systemstatuspayload_serialise() {
        let switchset = SwitchSettingFlags::S5_HEX_00 | SwitchSettingFlags::IS_S3_RIGHT;
        let LEDindicate = LEDFlags::VIDEO_PRESENT | LEDFlags::VIDEO_OK | LEDFlags::SERIAL_PRESENT;

        let payload = SystemStatusPayload {
            switchsetting: switchset,
            ledindication: LEDindicate,
            systemstatus: SystemStatus::SYSTEM_NORMAL,
            cpufirmwareversion: 1,
            pldfirmwareversion: 1,
            dspsoftwareversion: 1,
            dspstatus: Ok(3),
            numtargetsseen: 8,
            numtargetsidentified: 8,
            numtargetsused: 4,
            rmserror: u24::new(0),
        };

        let serial = payload.serialise();
        assert_eq!(serial.len(), 13);

        assert_eq!(serial[0], SwitchSettingFlags::bits(&switchset));
        assert_eq!(serial[1], LEDFlags::bits(&LEDindicate));

        assert_eq!(serial[2], SystemStatus::SYSTEM_NORMAL as u8);

        assert_eq!(serial[6], 3);
    }



    #[allow(non_snake_case)]
    #[test]
    fn systemstatuspayload_serialise_dsperror() {
        let switchset = SwitchSettingFlags::S5_HEX_00 | SwitchSettingFlags::IS_S3_RIGHT;
        let LEDindicate = LEDFlags::VIDEO_PRESENT | LEDFlags::VIDEO_OK | LEDFlags::SERIAL_PRESENT;

        let payload = SystemStatusPayload {
            switchsetting: switchset,
            ledindication: LEDindicate,
            systemstatus: SystemStatus::SYSTEM_NORMAL,
            cpufirmwareversion: 1,
            pldfirmwareversion: 1,
            dspsoftwareversion: 1,
            dspstatus: Err(DSPError::INTERNAL_ERROR),
            numtargetsseen: 8,
            numtargetsidentified: 8,
            numtargetsused: 4,
            rmserror: u24::new(0),
        };

        let serial = payload.serialise();

        assert_eq!(serial[6], DSPError::INTERNAL_ERROR as u8)
    }

    #[test]
    fn systemstatuspayload_deserialise() {
        let switchset = SwitchSettingFlags::S5_HEX_00 | SwitchSettingFlags::IS_S3_RIGHT;
        let LEDindicate = LEDFlags::VIDEO_PRESENT | LEDFlags::VIDEO_OK | LEDFlags::SERIAL_PRESENT;

        let payload = SystemStatusPayload {
            switchsetting: switchset,
            ledindication: LEDindicate,
            systemstatus: SystemStatus::SYSTEM_NORMAL,
            cpufirmwareversion: 1,
            pldfirmwareversion: 1,
            dspsoftwareversion: 1,
            dspstatus: Ok(3),
            numtargetsseen: 8,
            numtargetsidentified: 8,
            numtargetsused: 4,
            rmserror: u24::new(0),
        };

        let serial = payload.serialise();

        if let Ok(deserialised) = SystemStatusPayload::deserialise(&serial) {
            assert_eq!(deserialised, payload);
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn systemcontrolpayload_serialise() {
        let payload = SystemControlPayload {
            studioid: 25,
            smoothing: 13,
            maxasymmetry: 45,
            halfboxwidth: 8,
            blackvidthreshold: 1,
            whitevidthreshold: 55,
            blackvidclip: 255,
            whitevidclip: 23,
            maxblackpixels: 0,
            minwhitepixels: 0,
        };

        let serial = payload.serialise();

        //don't need to test that u8s are converted to u8s correctly
        assert_eq!(serial.len(), 10);
    }

    #[test]
    fn systemcontrolpayload_deserialise() {
        let payload = SystemControlPayload {
            studioid: 25,
            smoothing: 13,
            maxasymmetry: 45,
            halfboxwidth: 8,
            blackvidthreshold: 1,
            whitevidthreshold: 55,
            blackvidclip: 255,
            whitevidclip: 23,
            maxblackpixels: 0,
            minwhitepixels: 0,
        };

        let serial = payload.serialise();
        if let Ok(deserialised) = SystemControlPayload::deserialise(&serial) {
            assert_eq!(deserialised, payload);
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn targetdatapayload_serialise() {
        let testflags = 60606;

        let payload = TargetDataPayload {
            studioid: 25,
            targetnumber: 10404,
            targetx: i24::new(-500),
            targety: i24::new(-44545),
            targetz: i24::new(4040404),
            targetflags: i24::new(testflags),
        };

        let serial = payload.serialise();

        assert_eq!(serial.len(), 15);

        // assert_eq!(i32::from_be_bytes(&serial[serial.len()-3..serial.len()-1]), testflags);
    }

    #[test]
    fn imagedatapayload_serialise() {
        let payload = ImageDataPayload {
            targetindex: 250,
            targetnum: 30505,
            targetx: i24::new(-44545),
            targety: i24::new(-4114444),
            xerror: i24::new(-43422),
            yerror: i24::new(344),
        };
        let serial = payload.serialise();

        assert_eq!(serial.len(), 18 - 3);
    }

    #[allow(non_snake_case)]
    #[test]
    fn EEPROMdatapayload_serialise() {
        let payload = EEPROMDataPayload {
            EEPROMaddress: 0xFFAA,
            EEPROMdata: [0x00, 0x00, 0xFF, 0xA1, 0xFF, 0x12, 0x00, 0x44, 0x00, 0x00, 0x11, 0x44, 0xCA, 0x55, 0xAB, 0x43],
        };

        let serial = payload.serialise();

        assert_eq!(serial.len(), 18);

    }

    #[allow(non_snake_case)]
    #[test]
    fn EEPROMDataRequestPayload_serialise() {
        let payload = EEPROMDataRequestPayload {
            EEPROMaddress: 0xAAFF
        };

        let serial = payload.serialise();
        assert_eq!(serial.len(), 5 - 3);

    }  

    #[test]
    fn cameracalibrationpayload_serialise() {
        let payload = CameraCalibrationPayload {
            lenscentrex: i24::new(444),
            lenscentrey: i24::new(44433),
            lensscalex: i24::new(12),
            lensscaley: i24::new(-33),
            lensdistortiona: i24::new(1010),
            lensdistortionb: i24::new(-3000),
            xoffset: i24::new(101010),
            yoffset: i24::new(-440),
            zoffset: i24::new(100000),
        };

        let serial = payload.serialise();

        assert_eq!(serial.len(), 30-3);
    }

    #[test]
    fn diagnosticmodepayload_serialise() {
        let payload = DiagnosticModePayload {
            diagnosticflag: DiagnosticModes::NORMAL_OPERATION
        };

        let serial = payload.serialise();

        assert_eq!(serial.len(), 4 - 3);
        
        let flag:DiagnosticModes = serial[0].try_into().expect("Serialised diagnostic flag is valid");

        assert_eq!(flag, DiagnosticModes::NORMAL_OPERATION);
    }

    /*
     * This tests that POSITION_POLL payload structs are serialised to u8 arrays correctly:
     * - Values appear in the right places (sequential C-struct order)
     * - Reconstructing the array preserves values.
     */
    #[test]
    fn positionpollpayload_serialise() {
        let testpitch: i32 = -1000; //signed negative number for i24
        let testfocus: u32 = 0xAABBCC; //large unsigned number for u24

        let payload = PositionPollPayload {
            pitch: i24::new(testpitch),
            yaw: i24::new(0),
            roll: i24::new(0),
            pos_z: i24::new(0),
            pos_y: i24::new(0),
            pos_x: i24::new(0),
            zoom: u24::new(0),
            focus: u24::new(testfocus),
            userdefined: 0,
        };

        let serial = payload.serialise();
        println!("{:?}", serial);

        let testpitchbytes = testpitch.to_be_bytes();

        //byte values in the array at the expected positions match those generated by to_be_bytes()
        assert_eq!(serial[0], testpitchbytes[1]);
        assert_eq!(serial[1], testpitchbytes[2]);
        assert_eq!(serial[2], testpitchbytes[3]);

        //reconstructing pitch from array bytes preserves semantics (=-1000)
        let mut pitchreconstruct: [u8; 4] = [0xFF; 4]; //hack sign-extension in. Not in the spirit of testing. Not super important?
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
        let mut finalvals: [u8; 4] = [0; 4];
        let slice = &serial[(serial.len() - 2 - 3)..(serial.len() - 2)];
        assert_eq!(slice.len(), 3);
        finalvals[1..].clone_from_slice(slice);
        assert_eq!(u32::from_be_bytes(finalvals), testfocus);
    }

    #[test]
    fn message_new() {
        let inpayload = PositionPollPayload::default();
        let message = Message::new(inpayload, ALL_CAMERAS);

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

        let inpayload = PositionPollPayload::default();

        let mut message = Message::new(inpayload, ALL_CAMERAS);

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

        let inpayload = PositionPollPayload::default();

        let mut message: Message<PositionPollPayload> = Message::new(inpayload, ALL_CAMERAS);

        let mut payload = message.get_payload();
        payload.pitch = testpitch;
        payload.zoom = testzoom;

        message.set_payload(payload);

        let serial = message.serialise();
        println!("{:?}", serial);

        assert_eq!(serial.len(), 29);
        assert_eq!(serial[0], Commands::POSITION_POLL as u8);
        assert_eq!(serial[1], ALL_CAMERAS);

        assert_eq!(
            serial[serial.len() - 1],
            generate_checksum(&serial[..serial.len() - 1])
        ); //this kinda just checks that the checksum exists - ok so long as the unit tests are correct?
    }

    #[test]
    fn message_deserialise() {

        let inpayload = PositionPollPayload::default();
        
        let mut msg = Message::new(inpayload, ALL_CAMERAS);
        let mut payload = msg.get_payload();
        payload.roll = i24::new(55555);
        msg.set_payload(payload);


        let serial = msg.serialise();

        let deserialised: Message<PositionPollPayload> = deserialise(&serial).unwrap();

        let _p: PositionPollPayload = deserialised.payload.try_into().unwrap(); 
        
    }
}
