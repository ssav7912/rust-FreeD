use std::{vec, fmt};

use freed::freed::*;
use tui::{backend::Backend, layout::{Rect, Layout, Direction, Constraint}, Frame, widgets::{Table, Paragraph, Block, Cell, Row, Borders}};

use crate::even_columns;
pub trait StructUI {    

    fn to_array_of_strings(self) -> Vec<[String; 3]>;

    fn draw_fields_as_table<B>(self, f: &mut Frame<B>, area: Rect) where B: Backend;
 
    fn table_template<B>(f: &mut Frame<B>, area: Rect, title: &str, fields: Vec<[String; 3]>) where B: Backend {
        let innerlayout = Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(10), Constraint::Percentage(90)]).split(area);

        let struct_title = Paragraph::new(title).block(Block::default().borders(Borders::NONE));
        f.render_widget(struct_title, innerlayout[0]);

        let header_cells = ["Field", "Value", "Unit"].iter().map(|h| Cell::from(*h));
        let header = Row::new(header_cells).height(1).bottom_margin(1);

        let rows = fields.iter().map(|item| {
            let cells = item.iter().map(|c| Cell::from(c.as_ref()));
            Row::new(cells).height(1)
        });        

        
        let binding = even_columns(3);
        let table = Table::new(rows).header(header).block(Block::default().borders(Borders::ALL)).
        widths(binding.as_ref());
        f.render_widget(table, innerlayout[1]);
    }
}

impl StructUI for PollPayload {
    fn to_array_of_strings(self) -> Vec<[String; 3]> {
        vec![["Command".to_string(), self.command.to_string(), "".to_string()]]
    }

    fn draw_fields_as_table<B>(self, f: &mut Frame<B>, area: Rect) where B: Backend {
        let fields = self.to_array_of_strings();

        Self::table_template(f, area, "Poll Payload", fields);
    }
}

impl StructUI for PositionPollPayload {
    fn to_array_of_strings(self) -> Vec<[String; 3]> {
        vec![["pitch".to_string(), self.pitch.to_string(), "Millimetre".to_string()],
         ["yaw".to_string(), self.yaw.to_string(), "Millimetre".to_string()], 
         ["roll".to_string(), self.roll.to_string(), "Millimetre".to_string()],
         ["pos_z".to_string(), self.pos_z.to_string(), "Millimetre".to_string()],
         ["pos_y".to_string(), self.pos_y.to_string(), "Millimetre".to_string()],
         ["pos_x".to_string(), self.pos_x.to_string(), "Millimetre".to_string()],
         ["zoom".to_string(), self.zoom.to_string(), "Millimetre".to_string()],
         ["focus".to_string(), self.focus.to_string(), "".to_string()]
         ]
    }


    fn draw_fields_as_table<B>(self, f: &mut Frame<B>, area: Rect) where B: Backend {
        let fields = self.to_array_of_strings();

        Self::table_template(f, area, "Position Poll Payload", fields);
    }
}

impl StructUI for SystemStatusPayload {
    fn to_array_of_strings(self) -> Vec<[String; 3]> {
        vec![["Switch Setting".to_string(), self.switchsetting.to_string(), "Flags".to_string()],
        ["LED Indicator".to_string(), self.ledindication.to_string(), "Flags".to_string()],
        ["System Status".to_string(), self.systemstatus.to_string(), "Flags".to_string()],
        ["CPU Firmware Version".to_string(), self.cpufirmwareversion.to_string(), "".to_string()],
        ["PLD Firmware version".to_string(), self.pldfirmwareversion.to_string(), "".to_string()],
        ["DSP Software Version".to_string(), self.dspsoftwareversion.to_string(), "".to_string()],
        ["DSP Status".to_string(), match self.dspstatus {Ok(iter) => iter.to_string(), Err(err) => err.to_string()}, 
            match self.dspstatus{Ok(_) => "Iterations".to_string(), Err(_) => "DSP Error".to_string()}],
        ["Targets Seen".to_string(), self.numtargetsseen.to_string(), "".to_string()],
        ["Targets Identified".to_string(), self.numtargetsidentified.to_string(), "".to_string()],
        ["Targets Used".to_string(), self.numtargetsused.to_string(), "".to_string()],
        ["RMS Error".to_string(), self.rmserror.to_string(), "1/32768th Pixel".to_string()],
        ]
    }

    fn draw_fields_as_table<B>(self, f: &mut Frame<B>, area: Rect) where B: Backend {
        let fields = self.to_array_of_strings();
        
        Self::table_template(f, area, "System Status Payload", fields);
    }
}

impl StructUI for SystemControlPayload {
    fn to_array_of_strings(self) -> Vec<[String; 3]> {
        vec![
        ["Studio ID".to_string(), self.studioid.to_string(), "".to_string()],
        ["Smoothing".to_string(), self.smoothing.to_string(), "".to_string()],
        ["Max Asymmetry".to_string(), self.maxasymmetry.to_string(), "".to_string()],
        ["Half Box Width".to_string(), self.halfboxwidth.to_string(), "".to_string()],
        ["Black Video Threshold".to_string(), self.blackvidthreshold.to_string(), "".to_string()],
        ["White Video Threshold".to_string(), self.whitevidthreshold.to_string(), "".to_string()],
        ["Black Video Clip".to_string(), self.blackvidclip.to_string(), "".to_string()],
        ["White Video Clip".to_string(), self.whitevidclip.to_string(), "".to_string()],
        ["Max Black Pixels".to_string(), self.maxblackpixels.to_string(), "".to_string()],
        ["Min White Pixels".to_string(), self.minwhitepixels.to_string(), "".to_string()]
        ]
    }
    fn draw_fields_as_table<B>(self, f: &mut Frame<B>, area: Rect) where B: Backend {
        let fields = self.to_array_of_strings();

        Self::table_template(f, area, "System Control Payload", fields)
    }
}

impl StructUI for TargetDataPayload {
    fn to_array_of_strings(self) -> Vec<[String; 3]> {
        vec![
            ["Studio ID".to_string(), self.studioid.to_string(), "".to_string()],
            ["Target Number".to_string(), self.targetnumber.to_string(), "".to_string()],
            ["Target X".to_string(), self.targetx.to_string(), "".to_string()],
            ["Target Y".to_string(), self.targety.to_string(), "".to_string()],
            ["Target Z".to_string(), self.targetz.to_string(), "".to_string()],
            ["Target Flags".to_string(), self.targetflags.to_string(), "".to_string()]
        ]
    }

    fn draw_fields_as_table<B>(self, f: &mut Frame<B>, area: Rect) where B: Backend {
        let fields = self.to_array_of_strings();

        Self::table_template(f, area, "Target Data Payload", fields)
    }
}

impl StructUI for ImageDataPayload {
    fn to_array_of_strings(self) -> Vec<[String; 3]> {
        vec![
            ["Target Index".to_string(), self.targetindex.to_string(), "".to_string()],
            ["Target Num".to_string(), self.targetnum.to_string(), "".to_string()],
            ["Target X".to_string(), self.targetx.to_string(), "".to_string()],
            ["Target Y".to_string(), self.targety.to_string(), "".to_string()],
            ["X Error".to_string(), self.xerror.to_string(), "".to_string()],
            ["Y Error".to_string(), self.yerror.to_string(), "".to_string()]
        ]
    }

    fn draw_fields_as_table<B>(self, f: &mut Frame<B>, area: Rect) where B: Backend {
        let fields = self.to_array_of_strings();
        Self::table_template(f, area, "Image Data Payload", fields)
    }
}

impl StructUI for EEPROMDataPayload {
    fn to_array_of_strings(self) -> Vec<[String; 3]> {
        let mut fields = vec![
            ["EEPROM Address".to_string(), format!("{:#06x}", self.EEPROMaddress), "".to_string()],
            
        ];

        for byte in self.EEPROMdata {
            fields.push(["".to_string(), format!("{:#04x}", byte), "".to_string()]);
        }
        return fields;
    }

    fn draw_fields_as_table<B>(self, f: &mut Frame<B>, area: Rect) where B: Backend {
        let fields = self.to_array_of_strings();

        Self::table_template(f, area, "EEPROM Data Payload", fields)
    }
}

impl StructUI for EEPROMDataRequestPayload {
    fn to_array_of_strings(self) -> Vec<[String; 3]> {
        vec![
            ["EEPROM Address".to_string(), format!("{:#06x}", self.EEPROMaddress), "".to_string()]
        ]
    }
    fn draw_fields_as_table<B>(self, f: &mut Frame<B>, area: Rect) where B: Backend {
        let fields = self.to_array_of_strings();

        Self::table_template(f, area, "EEPROM Data Request Payload", fields)
    }
}

impl StructUI for CameraCalibrationPayload {
    fn to_array_of_strings(self) -> Vec<[String; 3]> {
        vec![
            ["Lens Centre X".to_string(), self.lenscentrex.to_string(), "".to_string()],
            ["Lens Centre Y".to_string(), self.lenscentrey.to_string(), "".to_string()],
            ["Lens Scale X".to_string(), self.lensscalex.to_string(), "".to_string()],
            ["Lens Scale Y".to_string(), self.lensscaley.to_string(), "".to_string()],
            ["Lens Distortion A".to_string(), self.lensdistortiona.to_string(), "".to_string()],
            ["Lens Distortion B".to_string(), self.lensdistortionb.to_string(), "".to_string()],
            ["X Offset".to_string(), self.xoffset.to_string(), "".to_string()],
            ["Y Offset".to_string(), self.yoffset.to_string(), "".to_string()],
            ["Z Offset".to_string(), self.zoffset.to_string(), "".to_string()]
        ]
    }

    fn draw_fields_as_table<B>(self, f: &mut Frame<B>, area: Rect) where B: Backend {
        let fields = self.to_array_of_strings();

        Self::table_template(f, area, "Camera Calibration Payload", fields)
    }
}

impl StructUI for DiagnosticModePayload {
    fn to_array_of_strings(self) -> Vec<[String; 3]> {
        vec![
            ["Diagnostic Flag".to_string(), self.diagnosticflag.to_string(), "".to_string()]
        ]
    }
    fn draw_fields_as_table<B>(self, f: &mut Frame<B>, area: Rect) where B: Backend {
        let fields = self.to_array_of_strings();

        Self::table_template(f, area, "Diagnostic Mode Payload", fields)
    }
}