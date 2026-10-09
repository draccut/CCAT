use crate::state::State;
use crate::col::HighlightConfig;

const BINARY_LINE_LENGTH: usize = 48;
const HEADER_LENGTH_BYTE: u32 = 38;

/**
 * Gibt den Inhalt einer Datei als farbigen Hex-Dump direkt
 * auf stdout aus (ähnlich `xxd` / `hexdump`).
 *
 * Im Gegensatz zu [`print_binary_file`] schreibt diese Funktion
 * nicht in einen String, sondern verwendet `print!` / `println!`.
 *
 * # Arguments
 * * `filepath` – Pfad zur Datei.
 * * `state`    – Zustand (Zeilennummer, Flag für Nummerierung).
 * * `config`   – Highlight-Konfiguration.
 */
pub fn print_hex_file(filepath: &str, state: &mut State, config: &HighlightConfig) {
    let mut file: std::fs::File = match std::fs::File::open(filepath) {
        Ok(opened_file) => opened_file,
        Err(_) => {
            println!("Fehler beim Lesen der Datei: {}", filepath);
            return;
        }
    };
    let file_size: u64 = match file.metadata() {
        Ok(metadata) => metadata.len(),
        Err(_) => {
            println!("Fehler beim Lesen der Datei: {}", filepath);
            return;
        }
    };
    if file_size == 0 {
        println!("Datei ist leer.");
        return;
    }
    let file_size_usize: usize = file_size as usize;
    let mut bytes: Vec<u8> = Vec::with_capacity(file_size_usize);
    bytes.resize(file_size_usize, 0u8);
    match std::io::Read::read_exact(&mut file, &mut bytes) {
        Ok(()) => {}
        Err(_) => {
            println!("Fehler beim Lesen der Datei: {}", filepath);
            return;
        }
    }
    let bytes_read: usize = bytes.len();
    let mut chunk_offset: usize = 0;
    while chunk_offset < bytes_read {
        let mut current_chunk_len: usize = bytes_read - chunk_offset;
        if current_chunk_len > BINARY_LINE_LENGTH {
            current_chunk_len = BINARY_LINE_LENGTH;
        }
        print!(
            "{}{:08x}  {}",
            config.color_header,
            chunk_offset,
            config.color_reset
        );
        if state.flag_nr {
            print!("{}{:08} {}", config.color_string, state.line_nr, config.color_reset);
        }
        print!("{}", config.color_keyword);
        let mut byte_index: usize = 0;
        while byte_index < current_chunk_len {
            let current_byte: u8 = bytes[chunk_offset + byte_index];
            if current_byte == b'\n' {
                state.line_nr += 1;
            }
            print!("{:02x} ", current_byte);
            byte_index += 1;
        }
        print!("{}", config.color_reset);
        if current_chunk_len < 8 {
            print!("{}", config.color_comment);
            let mut pad_index: usize = current_chunk_len;
            while pad_index < BINARY_LINE_LENGTH {
                print!("   ");
                pad_index += 1;
            }
            print!("{}", config.color_reset);
        }   
        print!("  |");
        byte_index = 0;
        while byte_index < current_chunk_len {
            let current_byte: u8 = bytes[chunk_offset + byte_index];
            if current_byte >= 32 && current_byte <= 126 {
                print!("{}", current_byte as char);
            } else {
                print!(".");
            }
            byte_index += 1;
        }
        println!("|");
        chunk_offset += current_chunk_len;
    }
    println!("{}{}{}Bytes insgesamt.", config.new_line, bytes_read, config.new_line);
}

/**
 * Hebt eine einzelne Log-Zeile farblich hervor und gibt sie
 * direkt auf stdout aus.
 *
 * Erkannte Muster:
 * - `[timestamp]` → `color_timestamp`
 * - `[ip]`        → `color_ip`
 * - `0xHH`        → `color_hex`
 *
 * # Arguments
 * * `line`   – Die zu verarbeitende Log-Zeile.
 * * `state`  – Zustand (Zeilennummer, Flag für Nummerierung).
 * * `config` – Highlight-Konfiguration.
 */
pub fn log_highlight_line(line: &str, state: &mut State, config: &HighlightConfig) {
    let line_bytes: &[u8] = line.as_bytes();
    let mut index: usize = 0;
    let mut stage: i32 = 0;
    print_line_number(state, config);
    state.line_nr += 1;
    let mut char_counter: u32 = 0;
    while index < line_bytes.len() {
        let current_byte: u8 = line_bytes[index];
        if current_byte == b'[' && char_counter < HEADER_LENGTH_BYTE {
            if stage == 0 {
                print!("{}[", config.color_timestamp);
                index += 1;
                while index < line_bytes.len() && line_bytes[index] != b']' {
                    print!("{}", line_bytes[index] as char);
                    index += 1;
                }
                if index < line_bytes.len() && line_bytes[index] == b']' {
                    print!("]{}", config.color_reset);
                    index += 1;
                    stage += 1;
                }
            } else {
                print!("{}[", config.color_ip);
                index += 1;
                while index < line_bytes.len() && line_bytes[index] != b']' {
                    print!("{}", line_bytes[index] as char);
                    index += 1;
                }
                if index < line_bytes.len() && line_bytes[index] == b']' {
                    print!("]{}", config.color_reset);
                    index += 1;
                    stage = 0;
                }
            }
            continue;
        } else if current_byte == b'0'
            && index + 3 < line_bytes.len()
            && line_bytes[index + 1] == b'x'
        {
            print!("{}0x", config.color_hex);
            index += 2;
            print!("{}", line_bytes[index] as char);
            index += 1;
            print!("{} ", line_bytes[index] as char);
            index += 1;
            print!("{}", config.color_reset);
            if index < line_bytes.len() && line_bytes[index] == b' ' {
                index += 1;
            }
            continue;
        } else {
            print!("{}", current_byte as char);
            index += 1;
        }
        char_counter += 1;
    }
    print!("{}", config.new_line);
}

/**
 * Liest eine Binärdatei und gibt sie als C-Array aus
 *
 * Bsp:
 * const unsigned char BUFF[145] = {
 * 0x5B, 0x70, 0x61, 0x63, 0x6B, 0x61, 0x67 . . .
 *
 * # Arguments
 * * `filename` – Pfad zur Eingabedatei.
 * * `config`   – Highlight-Konfiguration (für Keyword-/Zahlenfarben).
 */
pub fn to_c_array(filename: &str, config: &HighlightConfig) {
    let mut input_file: std::fs::File = match std::fs::File::open(filename) {
        Ok(opened_file) => opened_file,
        Err(_) => {
            eprintln!("Fehler: Datei {} konnte nicht geöffnet werden", filename);
            return;
        }
    };
    let file_size_u64: u64 = match input_file.metadata() {
        Ok(metadata) => metadata.len(),
        Err(_) => {
            eprintln!("Fehler: ftell fehlgeschlagen");
            return;
        }
    };
    let file_size: usize = file_size_u64 as usize;
    let mut data_buffer: Vec<u8> = Vec::with_capacity(file_size);
    data_buffer.resize(file_size, 0u8);
    if file_size > 0 {
        match std::io::Read::read_exact(&mut input_file, &mut data_buffer) {
            Ok(()) => {}
            Err(_) => {
                eprintln!("Fehler: Datei konnte nicht vollständig gelesen werden");
                return;
            }
        }
    }
    println!("{}const unsigned char BUFF[{}] = {{", config.color_keyword, file_size);
    let mut index: usize = 0;
    while index < file_size {
        print!("{}0x{:02X}{}", config.color_numbers, data_buffer[index], config.color_reset);
        if index + 1 < file_size {
            print!(", ");
        }
        if (index + 1) % 32 == 0 {
            println!("");
        }
        index += 1;
    }
    println!("");
    println!("{}}};{}", config.color_keyword, config.color_reset);
}

/**
 * Gibt eine formatierte Zeilennummer auf stdout aus,
 * sofern `state.flag_nr` gesetzt ist.
 *
 * Die Ausrichtung erfolgt analog zu [`append_line_number`],
 * verwendet jedoch `color_header` statt `color_numbers`.
 *
 * # Arguments
 * * `state`  – Aktueller Zustand (nur lesend).
 * * `config` – Highlight-Konfiguration.
 */
fn print_line_number(state: &State, config: &HighlightConfig) {
    if !state.flag_nr {
        return;
    }
    let line_nr = state.line_nr;
    let formatted_line = if line_nr < 10 {
        format!("{}{}   {}", config.color_header, line_nr, config.color_reset)
    } else if line_nr < 100 {
        format!("{}{}  {}", config.color_header, line_nr, config.color_reset)
    } else {
        format!("{}{} {}", config.color_header, line_nr, config.color_reset)
    };    
    print!("{}", formatted_line);
}
