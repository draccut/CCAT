use std::env;
use std::fs;

mod language;
mod state;
mod detect_filetype;
mod highlighter;
mod keywords;
mod specials;
mod col;
mod ccat_markdown;

use crate::state::State;
use crate::detect_filetype::detect_language;
use crate::highlighter::highlight_line;
use crate::col::HighlightConfig;
use crate::language::Language;
use crate::ccat_markdown::{md_html_main, md_ansi_main};
use crate::specials::{print_hex_file, log_highlight_line, to_c_array};

const INFO_TEXT: &str = r#"
Syntax: ccat <datei> [Optionen]
@VERSION 2.9.6 Rust

c <filename> <options ...>

Optionen:
    n   = Zeilennummern anzeigen
    r   = Remove all ANSI (save mode)
    h   = print as HTML Source
    a   = print file as C Hexadezimal Array
    x   = print as hex dump

    -s, --stop <nr> letzte auszugebende Zeile
"#;

/**
 * Liest eine Datei und baut daraus einen lesbaren String.
 *
 * Gültige UTF-8-Zeichen (inkl. Steuerzeichen `\n`, `\r`, `\t`
 * und druckbare ASCII) werden direkt übernommen.
 * Ungültige / nicht darstellbare Bytes werden als
 * ` 0xHH` (Hexadezimal) eingefügt.
 *
 * # Arguments
 * * `path`   – Pfad zur Eingabedatei.
 * * `output` – pointer-auf-Ziel-buffer wird vor dem Schreiben geleert
 */
fn read_file_with_hex_fallback(path: &str, output: &mut String) {
    output.clear();
    let bytes = match fs::read(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Fehler beim Lesen von '{}': {}", path, e);
            std::process::exit(1);
        }
    };
    output.reserve(bytes.len() * 2);
    let mut i = 0;
    while i < bytes.len() {
        if let Some((ch, len)) = decode_next_utf8_char(&bytes[i..]) {
            output.push(ch);
            i += len;
        } else {
            output.push_str(&format!(" 0x{:02X}", bytes[i]));
            i += 1;
        }
    }
}

/**
 * Versucht, ab dem Anfang von `slice` ein gültiges UTF-8-Zeichen
 * zu dekodieren.
 *
 * Akzeptiert:
 * - Steuerzeichen `\n`, `\r`, `\t`
 * - druckbare ASCII (32–126)
 * - mehrbyte UTF-8 (2–4 Bytes), sofern kein Control-Character
 *
 * # Arguments
 * * `slice` – Rest der Byte-Sequenz.
 *
 * # Returns
 * `Some((char, len))` bei Erfolg, sonst `None`
 *
 * Sequenzen die Shell Injections auslösen könnten werden
 * als Hex Byte ausgegeben. Funktioniert sehr zuverlässig
 * selbst bei Datein die dazu gebut wurden den Terminal Emulator
 * zu cracken.
 */
fn decode_next_utf8_char(slice: &[u8]) -> Option<(char, usize)> {
    if slice.is_empty() {
        return None;
    }
    let byte = slice[0];
    if matches!(byte, b'\n' | b'\r' | b'\t') {
        return Some((byte as char, 1));
    }
    if (32..=126).contains(&byte) {
        return Some((byte as char, 1));
    }
    if byte >= 128 {
        for len in 2..=4 {
            if len > slice.len() {
                break;
            }
            if let Ok(s) = std::str::from_utf8(&slice[..len]) {
                if let Some(ch) = s.chars().next() {
                    if ch.len_utf8() == len {
                        if !ch.is_control() {
                            return Some((ch, len));
                        }
                    }
                }
            }
        }
    }
    None
}

/**
 * Entfernt alle ANSI-Escape-Sequenzen aus einem String.
 *
 * Erkennt Sequenzen der Form `ESC [ ... Letter` und streicht sie.
 * Alle anderen Bytes werden unverändert übernommen.
 *
 * # Arguments
 * * `input`  – Quelltext (kann bereits Highlighting enthalten).
 * * `output` – Ziel-String; wird vor dem Schreiben geleert.
 */
fn remove_ansi_escapes(input: &str, output: &mut String) {
    output.clear();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == 0x1B {
            if i + 1 < bytes.len() && bytes[i + 1] == b'[' {
                i += 2;
                while i < bytes.len() {
                    let b = bytes[i];
                    i += 1;
                    if (b'A'..=b'Z').contains(&b) || (b'a'..=b'z').contains(&b) || (b'@'..=b'~').contains(&b) {
                        break;
                    }
                }
                continue;
            }
        }
        output.push(bytes[i] as char);
        i += 1;
    }
}

/**
 * Parst einen String in eine `u64`-Zahl.
 *
 * Wird für die Option `--stop <nr>` verwendet.
 * Panikt bei ungültiger Eingabe (`parse().unwrap()`).
 *
 * # Arguments
 * * `s` – Zahlen-String (z. B. 10000).
 *
 * # Returns
 * Die geparste Zahl.
 */
fn get_int_by_string(s: String) -> u64
{
    let num: u64 = s.parse().unwrap();
    return num;
}

/**
 * ==================================================================
 * Einstiegspunkt von CCAT
 * ==================================================================
 *
 * Ablauf:
 *
 * 1. Argumente lesen; bei fehlendem Dateinamen Hilfe ausgeben.
 * 2. Optionen (`n`, `r`, `h`, `x`, `a`, `-s`) in den State schreiben.
 * 3. Sprache über detect_language bestimmen.
 * 4. Datei einlesen oder bei Binary-Modus überspringen
 * 5. Je nach Flags einen der Spezialpfade wählen
 *    (
 *      Hex-Dump,
 *      Log-Highlight,
 *      C-Array, Binary, ANSI-Strip,
 *      Markdown, normaler Syntax-Highlight
 *    )
 *
 * 6. Ergebnis auf stdout ausgeben.
 */
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print!("{}", INFO_TEXT);
        return;
    }
    let mut state = State::new();
    if args.len() >= 3 {
        for ch in args[2].chars() {
            match ch {
                'n' => state.flag_nr = true,
                'r' => state.remove_all_ansi = true,
                'h' => state.html_color = true,
                'x' => state.hex = true,
                'a' => state.to_c_array = true,
                _ => {}
            }
        }
        if args[2] == "-s" || args[2] == "--stop" {
            state.stop_at_line = get_int_by_string(args[3].clone());
        } else {
            state.stop_at_line = 0xFFFFFFFFFFFFF000;
        }
    }
    let filename = &args[1];
    let lang = detect_language(filename);
    let mut source = String::new();
    let mut source_len = 0x1000;
    let mut output = String::with_capacity(source_len * 2);
    let mut jump: bool = false;
    let config = if state.html_color {
        HighlightConfig::html_colors()
    } else {
        HighlightConfig::default()
    };
    if state.hex {
        print_hex_file(filename, &mut state, &config);
        jump = true;
    }
    if lang == Language::LogFile {
        for line in source.lines() {
            log_highlight_line(line, &mut state, &config);
        }
        jump = true;
    }
    if state.to_c_array {
        to_c_array(filename, &config);
        jump = true;
    }
    if state.remove_all_ansi {
        remove_ansi_escapes(&source, &mut output);
        jump = true;
    }
    if !jump {
        if lang == Language::Markdown {
            if state.html_color {
                md_html_main(&source, &mut output);
            } else {
                md_ansi_main(&source, &mut output);
            }
        } else {
            for line in source.lines() {
                let stop_flag: bool = highlight_line(line, lang, &config, &mut output, &mut state);
                if stop_flag {
                    break;
                }
            }
        }
    }
    println!("{}", output);
}


#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Hilfsfunktion für Datei-basierte Tests.
     *
     * Schreibt `content` nach `/tmp/<filename>`, ruft
     * read_file_with_hex_fallback auf und vergleicht
     * das Ergebnis mit `expected`. Die Testdatei wird
     * anschließend gelöscht.
     *
     * # Arguments
     * * `filename` – Name der temporären Datei.
     * * `content`  – Roh-Bytes, die geschrieben werden.
     * * `expected` – Erwarteter String nach dem Einlesen.
     */
    fn test_with_file(filename: &str, content: &[u8], expected: &str) {
        let filepath = format!("/tmp/{}", filename);
        fs::write(&filepath, content).unwrap_or_else(|e| {
            panic!("Konnte Testdatei nicht erstellen {}: {}", filepath, e);
        });
        let mut result = String::new();
        read_file_with_hex_fallback(&filepath, &mut result);
        assert_eq!(result, expected, "Testdatei {} fehlgeschlagen", filename);
        let _ = fs::remove_file(&filepath);
    }

    /**
     * Prüft, dass gültige UTF-8-Sequenzen (Umlaute, Euro, Emoji)
     * korrekt dekodiert und ungültige Bytes als `0xHH` ausgegeben werden.
     */
    #[test]
    fn test_unicode_and_hex() {
        test_with_file(
            "rust_test_unicode.bin",
            b"Hello \xC3\xA4\xC3\xB6\xC3\xBC\xC3\x9F \xE2\x82\xAC \xF0\x9F\x9A\x80 World!\n\x00\x07\xFF",
            "Hello äöüß € 🚀 World!\n 0x00 0x07 0xFF",
        );
    }

    /**
     * Prüft die Behandlung ungültiger / abgeschnittener UTF-8-Sequenzen.
     */
    #[test]
    fn test_invalid_utf8_sequences() {
        test_with_file(
            "rust_test_invalid.bin",
            b"Valid text \xFF\xFE\xFD invalid again \xC3( broken \xE2\x82\xFF end",
            "Valid text  0xFF 0xFE 0xFD invalid again  0xC3( broken  0xE2 0x82 0xFF end",
        );
    }

    /**
     * Prüft reinen druckbaren Text inkl. Mehrbyte-UTF-8 (Umlaute).
     */
    #[test]
    fn test_only_printable_text() {
        test_with_file(
            "rust_test_text.txt",
            b"Hello World!\nThis is a test.\nLine 3 with umlauts: \xC3\xA4\xC3\xB6\xC3\xBC\xC3\x9F",
            "Hello World!\nThis is a test.\nLine 3 with umlauts: äöüß",
        );
    }

    /**
     * Prüft Steuerzeichen (`\n`, `\r`, `\t`) und nicht-druckbare Bytes.
     */
    #[test]
    fn test_control_characters() {
        test_with_file(
            "rust_test_control.bin",
            b"Line1\nLine2\r\nTab\t\x00\x01\x02\x1F\x7F\xFF",
            "Line1\nLine2\r\nTab\t 0x00 0x01 0x02 0x1F 0x7F 0xFF",
        );
    }

    /**
     * Prüft das Verhalten bei einer leeren Datei.
     */
    #[test]
    fn test_empty_file() {
        test_with_file("rust_test_empty.bin", b"", "");
    }
}
