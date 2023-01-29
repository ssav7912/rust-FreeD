pub mod freed;

pub mod common {
    use std::fmt::{self, write, Display};


    use bitflags::bitflags;
    pub const ALL_CAMERAS: u8 = 0xFF;
    pub trait Serialise {
        const COMMAND: Commands = Commands::DIAGNOSTIC_MODE;
        ///Generate an arbitrary array of `u8`s.
        fn serialise(self) -> Vec<u8>;

    }
    //move this into the serialise trait -
    pub trait Deserialise {
        // type Output;
        ///Deserialises an arbitrary array into the associated `Output` payload type, 
        /// or returns an error if this was not possible for some reason.
        fn deserialise(array: &[u8]) -> Result<Self, DeserialiseError> where Self: Sized;
    }

    #[derive(Debug, Clone)]
    pub struct DeserialiseError {
        pub description: String,
    }
    impl fmt::Display for DeserialiseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.description)
        }
    }

    ///NewType wrapper for RMS error - each unit is 1/32768th of a pixel.
    #[derive(Copy, Clone)]
    pub struct Pixel32768th(pub ux::u24);
    ///Newtype wrapper for 1 unit in camera control structs. Each unit is 1/64th of a pixel.
    #[derive(Copy, Clone)]
    #[cfg_attr(test, derive(PartialEq, Debug))]
    pub struct Millimetre64th(pub ux::i24);

    #[derive(Copy, Clone, Debug)]
    #[cfg_attr(test, derive(PartialEq))]
    pub struct Millimetre32768th(pub ux::i24);

    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, PartialEq, Debug)]

    ///Command bytes that may be used to specify payloads or signal the freed unit.
    pub enum Commands {
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
        EEPROM_DATA = 0xD8,
        REQUEST_EEPROM = 0xD9, //not in the source set of commands for 0xD0...
        CAMERA_CALIBRATION = 0xDA,
        DIAGNOSTIC_MODE = 0xDB,
    }
    //sucks. Write macro to autogenerate?
    impl TryFrom<u8> for Commands {
        type Error = String;
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                x if x == Self::STREAM_MODE_START as u8 => Ok(Self::STREAM_MODE_START),
                x if x == Self::STREAM_MODE_STOP as u8 => Ok(Self::STREAM_MODE_STOP),
                x if x == Self::FREEZE_MODE_STOP as u8 => Ok(Self::FREEZE_MODE_STOP),
                x if x == Self::FREEZE_MODE_START as u8 => Ok(Self::FREEZE_MODE_START),
                x if x == Self::POSITION_POLL as u8 => Ok(Self::POSITION_POLL),
                x if x == Self::SYSTEM_STATUS as u8 => Ok(Self::SYSTEM_STATUS),
                x if x == Self::SYSTEM_PARAMS as u8 => Ok(Self::SYSTEM_PARAMS),
                x if x == Self::FIRST_TARGET as u8 => Ok(Self::FIRST_TARGET),
                x if x == Self::NEXT_TARGET as u8 => Ok(Self::NEXT_TARGET),
                x if x == Self::FIRST_IMAGE as u8 => Ok(Self::FIRST_IMAGE),
                x if x == Self::NEXT_IMAGE as u8 => Ok(Self::NEXT_IMAGE),
                x if x == Self::EEPROM_DATA as u8 => Ok(Self::EEPROM_DATA),
                x if x == Self::REQUEST_EEPROM as u8 => Ok(Self::REQUEST_EEPROM),
                x if x == Self::CAMERA_CALIBRATION as u8 => Ok(Self::CAMERA_CALIBRATION),
                x if x == Self::DIAGNOSTIC_MODE as u8 => Ok(Self::DIAGNOSTIC_MODE),
                _ => Err(stringify!("Not a valid command!").to_string()),
            }
        }
    }

    impl Display for Commands{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::STREAM_MODE_START => "Stream Mode Start",
            Self::STREAM_MODE_STOP => "Stream Mode Stop",
            Self::FREEZE_MODE_START => "Freeze Mode Start",
            Self::FREEZE_MODE_STOP => "Freeze Mode Stop",
            Self::POSITION_POLL => "Position Poll",
            Self::SYSTEM_STATUS => "System Status",
            Self::SYSTEM_PARAMS => "System Params",
            Self::FIRST_TARGET => "First Target",
            Self::NEXT_TARGET => "Next Target",
            Self::FIRST_IMAGE => "First Image",
            Self::NEXT_IMAGE => "Next Image",
            Self::EEPROM_DATA => "EEPROM Data",
            Self::REQUEST_EEPROM => "Request EEPROM",
            Self::CAMERA_CALIBRATION => "Camera Calibration",
            Self::DIAGNOSTIC_MODE => "Diagnostic Mode"
            
        })
    }
    }

    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug)]
    ///Set of possible status codes the freed unit may report as part of 
    /// a 'system status' payload.
    pub enum SystemStatus {
        SYSTEM_NORMAL = 0,
        PROCESSOR_RESET = 1,
        SERIAL_ERROR = 2,
        VBLANK_FAIL = 3,
        XILINX_FAIL = 4,
        I2C_FAIL = 5,
        EEPROM_FAIL = 6,
        DSP_ACKNOWLEDGE_FAIL = 7,
        DSP_ACCEPT_FAIL = 8,
        DSP_PROVIDE_FAIL = 9,
        DSP_EXCEPTION_ERROR = 10,
        I2C_NOREPLY_FAIL = 91,
        I2C_BUSERROR_FAIL = 93,
        I2C_ACK_FAIL = 94,
        I2C_UNDEFINED_STATE = 95,
        I2C_OVERFLOW = 96,
    }

    impl Display for SystemStatus {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", match self {
                Self::SYSTEM_NORMAL => "System Normal",
                Self::PROCESSOR_RESET => "Processor Reset",
                Self::SERIAL_ERROR => "Serial Error",
                Self::VBLANK_FAIL => "VBlank Fail",
                Self::XILINX_FAIL => "XILINX Fail",
                Self::I2C_FAIL => "I2C Fail",
                Self::EEPROM_FAIL => "EEPROM Fail",
                Self::DSP_ACKNOWLEDGE_FAIL => "DSP Acknowledge Fail",
                Self::DSP_ACCEPT_FAIL => "DSP Accept Fail",
                Self::DSP_PROVIDE_FAIL => "DSP Provide Fail",
                Self::DSP_EXCEPTION_ERROR => "DSP Exception Error",
                Self::I2C_NOREPLY_FAIL => "I2C No Reply Fail",
                Self::I2C_BUSERROR_FAIL => "I2C Bus Error Fail",
                Self::I2C_ACK_FAIL => "I2C ACK Fail",
                Self::I2C_UNDEFINED_STATE => "I2C Undefined State",
                Self::I2C_OVERFLOW => "I2C Overflow"
            })
        }
    }


    impl Default for SystemStatus {
        fn default() -> Self {
            Self::SYSTEM_NORMAL
        }
    }

    bitflags! {
        #[derive(Copy, Clone, Default, Debug)]
        ///bitfield of possible flags that may be set for the freed LED indicator.
        pub struct LEDFlags: u8 {
            const VIDEO_PRESENT   = 0b00000001;
            const VIDEO_OK        = 0b00000010;
            const SERIAL_PRESENT  = 0b00000100;
            const DATA_FREEZE     = 0b00001000;
            const TOO_FEW_TARGETS = 0b00010000;
            const RMS_ERROR_HIGH  = 0b00100000;
            const DSP_ALERT       = 0b01000000;
            const FAULT           = 0b10000000;

        }
    }
    impl Display for LEDFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:08b}", self.bits())
        }
    }

    bitflags! {
        #[derive(Copy, Clone, Default, Debug)]
        pub struct SwitchSettingFlags: u8 {
            const S5_HEX_00   = 0b00000001;
            const S5_HEX_01   = 0b00000010;
            const S5_HEX_02   = 0b00000100;
            const S5_HEX_03   = 0b00001000;
            const S2_LEFT     = 0b00000000; //why does the protocol pattern change here? should I invert?
            const S2_RIGHT    = 0b00000000;
            const S4_CLOSED   = 0b00000000;
            const IS_S3_RIGHT = 0b10000000;
        }
    }

    impl Display for SwitchSettingFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:08b}", self.bits())
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug)]
    pub enum DSPError {
        TOO_FEW_TARGETS = -1,
        ITERATION_CONVERGE_FAIL = -2,
        DSP_RESET = -3,
        INTERNAL_ERROR = -4,
    }

    impl Default for DSPError {
        fn default() -> Self {
            Self::TOO_FEW_TARGETS
        }
    }

    impl Display for DSPError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", match self {
                Self::TOO_FEW_TARGETS => "Too Few Targets",
                Self::ITERATION_CONVERGE_FAIL => "Iteration Converge Fail",
                Self::DSP_RESET => "DSP Reset",
                Self::INTERNAL_ERROR => "Internal Error"
            } )
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub enum DSPStatus {
        Error(DSPError),
        Iterations(i8),
    }

    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug)]
    pub enum DiagnosticModes {
        NORMAL_OPERATION = 0x00,
        VIDEO_DATA_0x55 = 0x40,
        VIDEO_DATA_0xAA = 0x80,
        VIDEO_DATA_TEST = 0xC0,
    }

    impl Display for DiagnosticModes {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", match self {
                Self::NORMAL_OPERATION => "Normal Operation",
                Self::VIDEO_DATA_0x55 => "Video Data 0x55",
                Self::VIDEO_DATA_0xAA => "Video Data 0xAA",
                Self::VIDEO_DATA_TEST => "Video Data Test"
            })
        }
    }

    impl Default for DiagnosticModes {
        fn default() -> Self {
            Self::NORMAL_OPERATION
        }
    }
}
