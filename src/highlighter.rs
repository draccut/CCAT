use std::fs;

use crate::keywords::{
    ASM_8051_KEYWORDS, ASM_X86_KEYWORDS, KEYWORDS_C, DOCKER_KEYWORDS, JS_KEYWORDS,
    PYTHON_KEYWORDS, RUST_KEYWORDS, SHELL_KEYWORDS, SQL_KEYWORDS, Keyword, 
    DOCKER_COMPOSE_KEYWORDS, JAVA_KEYWORDS
};

use crate::state::State;
use crate::language::Language;
use crate::col::HighlightConfig;

/**
 * Prüft, ob ein Zeichen Teil eines Identifiers / Wortes ist
 * (Buchstaben, Ziffern, `_` oder `$`).
 *
 * # Arguments
 * * `c` – Das zu prüfende Zeichen.
 *
 * # Returns
 * `true`, wenn das Zeichen zu einem Wort gehört.
 */
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$'
}

/**
 * Prüft, ob das gegebene Wort in der Keyword-Liste vorkommt.
 *
 * # Arguments
 * * `word`     – Das zu suchende Wort.
 * * `keywords` – Slice von [`Keyword`]-Einträgen.
 *
 * # Returns
 * `true`, wenn das Wort ein Schlüsselwort ist.
 * z.B: while, for, void
 */
fn is_keyword(word: &str, keywords: &[Keyword]) -> bool 
{
    keywords.iter().any(|kw| kw.0 == word)
}

/**
 * Fügt eine formatierte Zeilennummer an den Ergebnis-String an,
 * sofern `state.flag_nr` gesetzt ist.
 *
 * Die Nummer wird je nach Größe mit unterschiedlicher
 * Anzahl Leerzeichen ausgerichtet (1–3 Ziffern).
 *
 * # Arguments
 * * `state`  – Aktueller Parser-Zustand (enthält `line_nr` und `flag_nr`).
 * * `result` – Ziel-String, an den die Ausgabe angehängt wird.
 * * `con`    – Highlight-Konfiguration mit Farbcodes.
 */
pub fn append_line_number(state: &mut State, result: &mut String, con: &HighlightConfig) 
{
    if !state.flag_nr {
        return;
    }
    if state.line_nr < 10 {
        result.push_str(&format!("{}{}   {}", con.color_numbers, state.line_nr, con.color_reset));
    } else if state.line_nr < 100 {
        result.push_str(&format!("{}{}  {}",  con.color_numbers, state.line_nr, con.color_reset));
    } else {
        result.push_str(&format!("{}{} {}",   con.color_numbers, state.line_nr, con.color_reset));
    }
}

/**
 * Hebt eine einzelne Quellcode-Zeile farblich hervor und hängt
 * das Ergebnis an `result` an.
 *
 * Unterstützt je nach [`Language`]:
 * - Einzeilige Kommentare (`#`, `//`, `;`)
 * - Mehrzeilige C-/SQL-Kommentare (`/* … */`)
 * - Strings mit Escape-Sequenzen
 * - HTML-Tags
 * - Keyword-Highlighting
 *
 * # Arguments
 * * `line`   – Die zu verarbeitende Zeile.
 * * `lang`   – Erkannte Programmiersprache.
 * * `config` – Farb- und Format-Konfiguration.
 * * `result` – Ziel-String für die formatierte Ausgabe.
 * * `state`  – Parser-Zustand (Zeilennummer, Abbruchbedingung, …).
 *
 * # Returns
 * `true`, wenn das Limit `stop_at_line` erreicht wurde und
 * die Verarbeitung abgebrochen werden soll, sonst `false`.
 */
pub fn highlight_line(line: &str, lang: Language, config: &HighlightConfig, result: &mut String, state: &mut State) -> bool 
{
    if state.stop_at_line <= state.line_nr {
        return true;
    }
    state.line_nr += 1;
    append_line_number(state, result, config);
    let mut p = line.chars().peekable();
    while let Some(mut ch) = p.next() {
        match lang {
            Language::AsmX86 | Language::Asm8051 => {
                if ch == ';' {
                    result.push_str(config.color_comment);
                    result.push(';');
                    for c in p { result.push(c); }
                    result.push_str(config.color_reset);
                    break;
                }
            }
            Language::Python | Language::Shell => {
                if ch == '#' {
                    result.push_str(config.color_comment);
                    result.push('#');
                    for c in p { result.push(c); }
                    result.push_str(config.color_reset);
                    break;
                }
            }
            Language::JavaScript | Language::Docker => {
                if ch == '/' && p.peek() == Some(&'/') {
                    result.push_str(config.color_comment);
                    result.push_str("//");
                    p.next();
                    for c in p {
                        result.push(c);
                    }
                    result.push_str(config.color_reset);
                    break;
                }
            }
            Language::Clang => {
                if ch == '/' {
                    match p.peek() {
                        Some(&'/') => {
                            result.push_str(config.color_comment);
                            result.push_str("//");
                            p.next();
                            for c in p {
                                result.push(c);
                            }
                            result.push_str(config.color_reset);
                            break;
                        }
                        Some(&'*') => {
                            result.push_str(config.color_comment);
                            result.push_str("/*");
                            p.next();
                            while let Some(c) = p.next() {
                                if c == '*' && p.peek() == Some(&'/') {
                                    result.push('*');
                                    result.push('/');
                                    p.next();
                                    result.push_str(config.color_reset);
                                    break;
                                }
                                result.push(c);
                            }
                            continue;
                        }
                        _ => {}
                    }
                }
            }
            Language::Sql => {
                if ch == '/' && p.peek() == Some(&'*') {
                    result.push_str(config.color_comment);
                    result.push_str("/*");
                    p.next();
                    while let Some(c) = p.next() {
                        if c == '*' && p.peek() == Some(&'/') {
                            result.push('*');
                            result.push('/');
                            p.next();
                            result.push_str(config.color_reset);
                            break;
                        }
                        result.push(c);
                    }
                    continue;
                }
            }
            _ => {}
        }
        if ch == '"' || ch == '\'' {
            let quote = ch;
            result.push_str(config.color_string);
            result.push(quote);
            ch = match p.next() {
                Some(c) => c,
                None => { result.push_str(config.color_reset); break; }
            };
            while ch != quote {
                if ch == '\\' {
                    result.push('\\');
                    if let Some(next) = p.next() {
                        result.push(next);
                        ch = match p.next() { Some(c) => c, None => break };
                        continue;
                    }
                } else {
                    result.push(ch);
                }
                ch = match p.next() { Some(c) => c, None => break };
            }
            if ch == quote {
                result.push(quote);
            }
            result.push_str(config.color_reset);
            continue;
        }
        if lang == Language::Html && ch == '<' {
            result.push_str(config.color_tag);
            result.push('<');
            ch = match p.next() { Some(c) => c, None => { result.push_str(config.color_reset); break; } };
            while ch != '>' {
                if ch == '"' {
                    let quote = ch;
                    result.push_str(config.color_string);
                    result.push(quote);
                    ch = match p.next() { Some(c) => c, None => break };
                    while ch != quote {
                        if ch == '\\' {
                            result.push('\\');
                            if let Some(n) = p.next() { result.push(n); ch = match p.next() { Some(c)=>c, None=>break }; continue; }
                        } else {
                            result.push(ch);
                        }
                        ch = match p.next() { Some(c) => c, None => break };
                    }
                    if ch == quote {
                        result.push(quote);
                    }
                    result.push_str(config.color_tag);
                } else {
                    result.push(ch);
                }
                ch = match p.next() { Some(c) => c, None => break };
            }
            if ch == '>' {
                result.push('>');
            }
            result.push_str(config.color_reset);
            continue;
        }
        if is_word_char(ch) {
            let mut buf = String::new();
            buf.push(ch);
            while let Some(&next) = p.peek() {
                if is_word_char(next) && buf.len() < 250 {
                    buf.push(next);
                    p.next();
                } else {
                    break;
                }
            }
            let is_kw = match lang {
                Language::Python                => is_keyword(&buf, PYTHON_KEYWORDS),
                Language::JavaScript            => is_keyword(&buf, JS_KEYWORDS),
                Language::Clang                 => is_keyword(&buf, KEYWORDS_C),
                Language::Shell                 => is_keyword(&buf, SHELL_KEYWORDS),
                Language::Docker                => is_keyword(&buf, DOCKER_KEYWORDS),
                Language::AsmX86                => is_keyword(&buf, ASM_X86_KEYWORDS),
                Language::Asm8051               => is_keyword(&buf, ASM_8051_KEYWORDS),
                Language::Sql                   => is_keyword(&buf, SQL_KEYWORDS),
                Language::Rust                  => is_keyword(&buf, RUST_KEYWORDS),
                Language::LangDockerCompose     => is_keyword(&buf, DOCKER_COMPOSE_KEYWORDS),
                Language::LangJava              => is_keyword(&buf, JAVA_KEYWORDS),
                _                               => false,
            };
            if is_kw {
                result.push_str(config.color_keyword);
                result.push_str(&buf);
                result.push_str(config.color_reset);
            } else {
                result.push_str(&buf);
            }
            continue;
        }
        result.push(ch);
    }
    result.push_str(config.new_line);
    return false;
}


