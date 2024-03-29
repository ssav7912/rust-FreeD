mod payloadui;

use std::fmt::{Display, format};
use std::net::{UdpSocket, IpAddr, Ipv4Addr, SocketAddr};
use std::ops::Not;
use crossterm::terminal::{Clear, ClearType, ScrollUp, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{self, event, execute};
use crossterm::event::{read,Event,KeyCode, KeyEvent, MouseEvent};
use freed::payloads::{PositionPollPayload, PollPayload, SystemStatusPayload, SystemControlPayload, TargetDataPayload, ImageDataPayload, EEPROMDataPayload, EEPROMDataRequestPayload, CameraCalibrationPayload, DiagnosticModePayload};
use payloadui::StructUI;
use tui::Frame;
use tui::widgets::{Paragraph, Wrap};
use tui::{backend::CrosstermBackend, widgets::{Widget, Block, Borders},layout::{Layout, Constraint, Direction}, Terminal};
use freed;

struct CleanUp(std::io::Stdout);
impl Drop for CleanUp {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().expect("Could not disable raw mode");
        execute!(self.0, LeaveAlternateScreen).expect("Failed to cleanup");
    }
}

#[derive(Copy, Clone)]
struct Status {
    operating_mode: OperatingModes,
    payload_mode: PayloadModes,
    payload_index: Option<i32>,
    payload_history: [PayloadModes; 10],
    address: IpAddr,
    port: u16,
}

impl Status {
    pub fn change_payload_mode(&mut self, new_mode: PayloadModes) {
        self.payload_history[self.payload_mode.get_array_index()] = self.payload_mode;
        self.payload_mode = new_mode;
    }
    
    pub fn increment_index(&mut self) -> Option<i32> {
        self.new_index(match self.payload_index {Some(i) => Some(i + 1), None => Some(0)})
    }

    pub fn decrement_index(&mut self) -> Option<i32> {
        self.new_index(match self.payload_index {Some(i) => Some(i -1), 
            None => Some(self.payload_mode.to_array_of_strings().len() as i32)})
    }

    fn new_index(&mut self, new_index: Option<i32>) -> Option<i32> {
        let payloadarray = self.payload_mode.to_array_of_strings();
        let len = payloadarray.len();

        let index = new_index;
        match index {
            Some(i) => if i as usize >= len {self.payload_index = None} 
            else {self.payload_index = index},
            None => self.payload_index = None
            
        }

        self.payload_index
    }

}

impl Default for Status {
    fn default() -> Self {
        let payloadhistory = PayloadModes::array();

        Status {operating_mode: OperatingModes::FreezeMode, payload_mode: PayloadModes::PositionPollPayload(PositionPollPayload::default()),
        payload_index: None, payload_history: payloadhistory, address: IpAddr::V4(Ipv4Addr::LOCALHOST), port: 40000}
    }
}





#[derive(Clone, Copy)]
enum OperatingModes {
    StreamMode,
    FreezeMode,
}

impl Not for OperatingModes {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::StreamMode => Self::FreezeMode,
            Self::FreezeMode => Self::StreamMode
        }
    }
}

impl Display for OperatingModes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {Self::StreamMode => "Stream Mode", Self::FreezeMode => "Freeze Mode"})
    }
}

#[derive(Copy, Clone, Debug)]
enum PayloadModes {
    PollPayload(freed::payloads::PollPayload),
    PositionPollPayload(freed::payloads::PositionPollPayload),
    SystemStatusPayload(freed::payloads::SystemStatusPayload),
    SystemControlPayload(freed::payloads::SystemControlPayload),
    TargetDataPayload(freed::payloads::TargetDataPayload),
    ImageDataPayload(freed::payloads::ImageDataPayload),
    EEPROMDataPayload(freed::payloads::EEPROMDataPayload),
    EEPROMDataRequestPayload(freed::payloads::EEPROMDataRequestPayload),
    CameraCalibrationPayload(freed::payloads::CameraCalibrationPayload),
    DiagnosticModePayload(freed::payloads::DiagnosticModePayload),
}

impl PayloadModes {
    pub fn get_array_index(self) -> usize {
        match self {
            PayloadModes::PollPayload(_) => 0,
            PayloadModes::PositionPollPayload(_) => 1,
            PayloadModes::SystemStatusPayload(_) => 2,
            PayloadModes::SystemControlPayload(_) => 3,
            PayloadModes::TargetDataPayload(_) => 4,
            PayloadModes::ImageDataPayload(_) => 5,
            PayloadModes::EEPROMDataPayload(_) => 6,
            PayloadModes::EEPROMDataRequestPayload(_) => 7,
            PayloadModes::CameraCalibrationPayload(_) => 8,
            PayloadModes::DiagnosticModePayload(_) => 9
        }
    }
    pub fn array() -> [Self; 10] {
         [
            PayloadModes::PollPayload(PollPayload::default()), 
            PayloadModes::PositionPollPayload(PositionPollPayload::default()),
            PayloadModes::SystemStatusPayload(SystemStatusPayload::default()),
            PayloadModes::SystemControlPayload(SystemControlPayload::default()),
            PayloadModes::TargetDataPayload(TargetDataPayload::default()),
            PayloadModes::ImageDataPayload(ImageDataPayload::default()),
            PayloadModes::EEPROMDataPayload(EEPROMDataPayload::default()),
            PayloadModes::EEPROMDataRequestPayload(EEPROMDataRequestPayload::default()),
            PayloadModes::CameraCalibrationPayload(CameraCalibrationPayload::default()),
            PayloadModes::DiagnosticModePayload(DiagnosticModePayload::default()),]
    }

}

impl Display for PayloadModes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PayloadModes::PollPayload(_) => "Poll Payload",
            PayloadModes::PositionPollPayload(_) => "Position Poll Payload",
            PayloadModes::SystemStatusPayload(_) => "System Status Payload",
            PayloadModes::SystemControlPayload(_) => "System Control Payload",
            PayloadModes::TargetDataPayload(_) => "Target Data Payload",
            PayloadModes::ImageDataPayload(_) => "Image Data Payload",
            PayloadModes::EEPROMDataPayload(_) => "EEPROM Data Payload",
            PayloadModes::EEPROMDataRequestPayload(_) => "EEPROM Data Request Payload",
            PayloadModes::CameraCalibrationPayload(_) => "Camera Calibration Payload",
            PayloadModes::DiagnosticModePayload(_) => "Diagnostic Mode Payload"
    
        })
    
    }
}

impl StructUI for PayloadModes {
    fn to_array_of_strings(self) -> Vec<[String; 3]> {
        match self {
            PayloadModes::CameraCalibrationPayload(a) => a.to_array_of_strings(),
            PayloadModes::PollPayload(a) => a.to_array_of_strings(),
            PayloadModes::PositionPollPayload(a) => a.to_array_of_strings(),
            PayloadModes::SystemStatusPayload(a) => a.to_array_of_strings(),
            PayloadModes::SystemControlPayload(a) => a.to_array_of_strings(),
            PayloadModes::TargetDataPayload(a) => a.to_array_of_strings(),
            PayloadModes::ImageDataPayload(a) => a.to_array_of_strings(),
            PayloadModes::EEPROMDataPayload(a) => a.to_array_of_strings(),
            PayloadModes::EEPROMDataRequestPayload(a) => a.to_array_of_strings(),
            PayloadModes::DiagnosticModePayload(a) => a.to_array_of_strings(),
        }
    }

    fn draw_fields_as_table<B>(self, f: &mut Frame<B>, index: Option<i32>, area: tui::layout::Rect) where B: tui::backend::Backend {
        match self {
            PayloadModes::CameraCalibrationPayload(a) => a.draw_fields_as_table(f, index, area),
            PayloadModes::PollPayload(a) => a.draw_fields_as_table(f, index, area),
            PayloadModes::PositionPollPayload(a) => a.draw_fields_as_table(f, index, area),
            PayloadModes::SystemStatusPayload(a) => a.draw_fields_as_table(f, index, area),
            PayloadModes::SystemControlPayload(a) => a.draw_fields_as_table(f, index, area),
            PayloadModes::TargetDataPayload(a) => a.draw_fields_as_table(f, index, area),
            PayloadModes::ImageDataPayload(a) => a.draw_fields_as_table(f, index, area),
            PayloadModes::EEPROMDataPayload(a) => a.draw_fields_as_table(f, index, area),
            PayloadModes::EEPROMDataRequestPayload(a) => a.draw_fields_as_table(f, index, area),
            PayloadModes::DiagnosticModePayload(a) => a.draw_fields_as_table(f, index, area)
            
        }
    }
}

//Maximum of 100 columns!
fn even_columns(columns: usize) -> Vec<Constraint> {
    let percent = 100/columns;
    std::iter::repeat(Constraint::Percentage(percent as u16)).take(columns).collect()
}


fn ui<B: tui::backend::Backend>(f: &mut Frame<B>, status: Status) {

    let main = Layout::default().direction(Direction::Vertical)
    .constraints([Constraint::Percentage(20), Constraint::Percentage(80), ].as_ref()).split(f.size());
    
    let inoutsplit = Layout::default().direction(Direction::Horizontal).margin(0).constraints(even_columns(2).as_ref()).split(main[1]);
    
    let outstructblock = Block::default().title("Data Out").borders(Borders::ALL);
    let innerarea = outstructblock.inner(inoutsplit[0]);
    f.render_widget(outstructblock, inoutsplit[0]);
    let payload = status.payload_mode;
    payload.draw_fields_as_table(f, status.payload_index, innerarea);
    

    // let outstructtext = Paragraph::new(format!("{:?}", status.payload_mode)).block(outstructblock).wrap(Wrap {trim: true});
    // f.render_widget(outstructtext, inoutsplit[0]);

    let instructblock = Block::default().title("Data In").borders(Borders::ALL);
    f.render_widget(instructblock, inoutsplit[1]);

    let chunks = Layout::default().direction(Direction::Horizontal)
    .constraints(even_columns(4).as_ref()).split(main[0]);
    

    let pollblock = Block::default().title("Poll Mode").borders(Borders::ALL);
    let polltext = tui::widgets::Paragraph::new(status.operating_mode.to_string()).block(pollblock);
    f.render_widget(polltext, chunks[0]);


    let payloadblock = Block::default().title("Payload Mode").borders(Borders::ALL);
    let payloadtext = Paragraph::new(status.payload_mode.to_string()).block(payloadblock);
    f.render_widget(payloadtext, chunks[1]);
    
    let addressblock = Block::default().title("Address").borders(Borders::ALL);
    let addresstext = Paragraph::new(status.address.to_string()).block(addressblock);
    f.render_widget(addresstext, chunks[2]);

    let portblock = Block::default().title("Port").borders(Borders::ALL);
    let porttext = Paragraph::new(status.port.to_string()).block(portblock);
    f.render_widget(porttext, chunks[3]);


}
    

fn main() -> Result<(), std::io::Error> {
    let mut status = Status::default();
    let mut inputBuffer = Vec::<char>::new();

    let socket = UdpSocket::bind(SocketAddr::new(status.address, status.port)).unwrap();

    crossterm::terminal::enable_raw_mode()?;

    let _cleanup = CleanUp(std::io::stdout());
    

    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;


    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| ui(f, status))?;
        

        if let Event::Key(event) = crossterm::event::read().expect("Failed to read line") {
            match event {
                
                //quit
                KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: event::KeyModifiers::CONTROL,
                    ..
                } => break,
                
                //change operating mode
                KeyEvent {
                    code: KeyCode::F(num),
                    modifiers: event::KeyModifiers::NONE,
                    .. 
                } => {
                    if num == 1 {
                        status.operating_mode = !status.operating_mode;
                    }

                    
                },
                
                //change payload struct
                KeyEvent {
                    code: KeyCode::Char(char),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    match status.payload_index {
                        None =>  match char.to_digit(10) {
                            Some(1) => status.change_payload_mode(status.payload_history[0]),
                            Some(2) => status.change_payload_mode(status.payload_history[1]),
                            Some(3) => status.change_payload_mode(status.payload_history[2]),
                            Some(4) => status.change_payload_mode(status.payload_history[3]),
                            Some(5) => status.change_payload_mode(status.payload_history[4]),
                            Some(6) => status.change_payload_mode(status.payload_history[5]),
                            Some(7) => status.change_payload_mode(status.payload_history[6]),
                            Some(8) => status.change_payload_mode(status.payload_history[7]),
                            Some(9) => status.change_payload_mode(status.payload_history[8]),
                            Some(0) => status.change_payload_mode(status.payload_history[9]), 
                            _ => continue
                        },
                        Some(index) => {inputBuffer.push(char);
                            
                        }

                    }
                   
                }

                KeyEvent {
                    code: KeyCode::Tab,
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    status.increment_index();
                }

                KeyEvent {
                    code: KeyCode::BackTab,
                    ..
                } => {status.decrement_index();}


                _ => {}

            }

            };




    }
    Ok(())
}

