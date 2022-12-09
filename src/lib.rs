mod freed;

mod common {
    use bitflags::bitflags;
    pub const ALL_CAMERAS: u8 = 0xFF;
    pub trait Serialise {
        const COMMAND: Commands = Commands::DIAGNOSTIC_MODE;
        ///Generate an arbitrary array of `u8`s.
        fn serialise(self) -> Vec<u8>;

        fn deserialise<T: Serialise + Default>(
            array: &[u8],
        ) -> Result<T, String> {
            

            todo!()
        }
    }

    trait Command {
        const COMMAND: Commands;
    }


    ///NewType wrapper for RMS error - each unit is 1/32768th of a pixel.
    #[derive(Copy, Clone)]
    pub struct Pixel32768th(pub ux::u24);
    ///Newtype wrapper for 1 unit in camera control structs. Each unit is 1/64th of a pixel.
    #[derive(Copy, Clone)]
    #[cfg_attr(test, derive(PartialEq, Debug))]
    pub struct Millimetre64th(pub ux::i24);

    #[derive(Copy, Clone)]
    #[cfg_attr(test, derive(PartialEq, Debug))]
    pub struct Millimetre32768th(pub ux::i24);

    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, PartialEq)]
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
        REQUEST_EEPROM = 0xD9, //not in the source set of commands for D0...
        CAMERA_CALIBRATION = 0xDA,
        DIAGNOSTIC_MODE = 0xDB,
    }
    //sucks
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



    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone)]
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

    bitflags! {
        #[derive(Copy, Clone)]
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

    bitflags! {
        #[derive(Copy, Clone)]
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

    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone)]
    pub enum DSPError {
        TOO_FEW_TARGETS = -1,
        ITERATION_CONVERGE_FAIL = -2,
        DSP_RESET = -3,
        INTERNAL_ERROR = -4,
    }

    #[derive(Copy, Clone)]
    pub enum DSPStatus {
        Error(DSPError),
        Iterations(i8),
    }

    #[derive(Copy, Clone, Default)]

    pub enum DiagnosticModes {
        #[default]
        NORMAL_OPERATION = 0x00,
        VIDEO_DATA_0x55 = 0x40,
        VIDEO_DATA_0xAA = 0x80,
        VIDEO_DATA_TEST = 0xC0,
    }
}
