use std::io;

const UNISON_HEADER_VERSION: &str = "Unison:SyntaxRevision1909.15;";

pub struct UdigPattern {
    name: String,
    comment: String,
    pattern_type: String,
    pin_list: String,
    pin_usage: String,
    subset_pins: String,
    alias_map: String,
    default_waveform: String,
    default_vector: String,
    send_ref: String,
    capture_ref: String,
    sync_ref: String,
    pattern_data: Vec<String>,
    pattern_file_name: String,
}


impl UdigPattern {
    // This method will write the .uno pattern
    pub fn write_to_file(&self) -> Result<(), ()> {

        if self.pattern_file_name.len() == 0 {
            return Err(());
        }
        else {
            return Ok(())
        }
    }

}