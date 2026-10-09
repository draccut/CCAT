/**
 * Hintergrundfarbe (hoch) + Vordergrundfarbe (niedrig) als 48-Bit-Wert.
 * Layout: 0xBBGGRR_FFGGEE  (BG R/G/B | FG R/G/B)
 * Hier: dunkles Blau-Grau (#22223B) als Hintergrund, Weiß als Vordergrund.
 */
const MY_COLOR: u64 = 0x22223Bffffffu64;

const CARD_LEN: usize = 0x32;

/**
 * Mapping eines Markdown-Markers auf HTML- und ANSI-Äquivalente.
 *
 * Jeder Eintrag beschreibt:
 * - `md_start` / `md_end` – die Markdown-Syntax (z. B. `**` … `**`)
 * - `html_start` / `html_end` – die zugehörigen HTML-Tags
 * - `ansi_start` / `ansi_end` – die zugehörigen ANSI-Escape-Sequenzen
 */
#[derive(Clone, Copy)]
pub struct MdMapping {
    pub md_start: &'static str,
    pub md_end: &'static str,
    pub html_start: &'static str,
    pub html_end: &'static str,
    pub ansi_start: &'static str,
    pub ansi_end: &'static str,
}

/**
 * Statische Lookup-Tabelle aller unterstützten Markdown-Marker.
 *
 * Enthält Überschriften (H1–H6), Fett, Kursiv, Durchgestrichen und Inline-Code.
 * Einträge mit leerem `md_end` sind Zeilen-Präfixe (Überschriften),
 * alle anderen sind Inline-Paare.
 */
static MD_MAP: &[MdMapping] = &[
    MdMapping { md_start: "# ",      md_end: "",   html_start: "<h1>",     html_end: "</h1>",     ansi_start: "\x1b[1;37m", ansi_end: "\x1b[0m" },
    MdMapping { md_start: "## ",     md_end: "",   html_start: "<h2>",     html_end: "</h2>",     ansi_start: "\x1b[1;36m", ansi_end: "\x1b[0m" },
    MdMapping { md_start: "### ",    md_end: "",   html_start: "<h3>",     html_end: "</h3>",     ansi_start: "\x1b[1;35m", ansi_end: "\x1b[0m" },
    MdMapping { md_start: "#### ",   md_end: "",   html_start: "<h4>",     html_end: "</h4>",     ansi_start: "\x1b[1;34m", ansi_end: "\x1b[0m" },
    MdMapping { md_start: "##### ",  md_end: "",   html_start: "<h5>",     html_end: "</h5>",     ansi_start: "\x1b[1;33m", ansi_end: "\x1b[0m" },
    MdMapping { md_start: "###### ", md_end: "",   html_start: "<h6>",     html_end: "</h6>",     ansi_start: "\x1b[1;32m", ansi_end: "\x1b[0m" },
    MdMapping { md_start: "**",      md_end: "**", html_start: "<strong>", html_end: "</strong>", ansi_start: "\x1b[1m",    ansi_end: "\x1b[0m" },
    MdMapping { md_start: "__",      md_end: "__", html_start: "<strong>", html_end: "</strong>", ansi_start: "\x1b[1m",    ansi_end: "\x1b[0m" },
    MdMapping { md_start: "*",       md_end: "*",  html_start: "<em>",     html_end: "</em>",     ansi_start: "\x1b[3m",    ansi_end: "\x1b[0m" },
    MdMapping { md_start: "_",       md_end: "_",  html_start: "<em>",     html_end: "</em>",     ansi_start: "\x1b[3m",    ansi_end: "\x1b[0m" },
    MdMapping { md_start: "~~",      md_end: "~~", html_start: "<del>",    html_end: "</del>",    ansi_start: "\x1b[9m",    ansi_end: "\x1b[0m" },
    MdMapping { md_start: "`",       md_end: "`",  html_start: "<code>",   html_end: "</code>",   ansi_start: "\x1b[7m",    ansi_end: "\x1b[0m" },
];

fn log(message: &str) {
    println!("{}", message);
}

fn pp(res: &mut String, input: String) {
    res.push_str(&input);
}

/**
 * Schreibt `text` mit Vorder- und Hintergrundfarbe (aus `col`)
 * als ANSI-24-Bit-Sequenzen in `res`.
 *
 * `col` wird in 6 Bytes zerlegt:
 * - Bits 47–40 / 39–32 / 31–24  BG R/G/B
 * - Bits 23–16 / 15–8  /  7–0   FG R/G/B
 *
 * # Arguments
 * * `res`  – Ziel-String.
 * * `col`  – 48-Bit-Farbwert (siehe [`MY_COLOR`]).
 * * `text` – Der einzufärbende Text.
 */
fn pri(res: &mut String, col: u64, text: &str) {
    let bg_r = ((col >> 40) & 0xFF) as u8;
    let bg_g = ((col >> 32) & 0xFF) as u8;
    let bg_b = ((col >> 24) & 0xFF) as u8;
    let fg_r = ((col >> 16) & 0xFF) as u8;
    let fg_g = ((col >> 8)  & 0xFF) as u8;
    let fg_b = (col         & 0xFF) as u8;
    pp(res, format!("\x1b[38;2;{};{};{}m", fg_r, fg_g, fg_b));
    pp(res, format!("\x1b[48;2;{};{};{}m", bg_r, bg_g, bg_b));
    pp(res, format!("{}", text));
    pp(res, format!("\x1b[0m"));
}

/**
 * Zeichnet eine Trennlinie aus `CARD_LEN` Gleichheitszeichen
 * (mit führendem Einzug) in `res`.
 *
 * # Arguments
 * * `res` – Ziel-String.
 */
fn draw_line(res: &mut String) {
    let first = format!("     {} ", "=".repeat(CARD_LEN));
    res.push_str(&first);
}

/**
 * Schreibt den ANSI-Tabellen-Header (drei leere farbige Karten
 * nebeneinander) und eine Trennlinie in `res`.
 *
 * # Arguments
 * * `res` – Ziel-String.
 */
fn print_head(res: &mut String) {
    #[allow(non_snake_case)]
    let LL: String = String::from("\n");
    #[allow(non_snake_case)]
    let SS: String = String::from("     ");
    #[allow(non_snake_case)]
    let QQ: String = String::from(" | ");
    pp(res, SS.clone());
    pri(res, MY_COLOR, &format!("{: <50}", " "));
    pp(res, QQ.clone());
    pri(res, MY_COLOR, &format!("{: <50}", " "));
    pp(res, QQ);
    pri(res, MY_COLOR, &format!("{: <50}", " "));
    pp(res, LL.clone());
    draw_line(res);
    pp(res, LL);
}

/**
 * Rendert eine Markdown-Tabelle als ANSI-Kartenlayout.
 *
 * Überspringt Trennzeilen (`|---|`), spaltet jede Zeile an `|`
 * und schreibt jede Zelle als farbige, linksbündig gepaddete Karte.
 *
 * # Arguments
 * * `res`   – Ziel-String.
 * * `lines` – Die Tabellenzeilen (inkl. Header und Separator).
 */
fn process_table_ansi(res: &mut String, lines: &[&str]) {
    print_head(res);
    for (_idx, line) in lines.iter().enumerate() {
        if line.starts_with('|') && line.contains("---") {
            continue;
        }
        let cells: Vec<&str> = line.split('|').collect();
        for index in 1..cells.len() {
            pri(res, MY_COLOR, &format!("{: <50}", process_inline(cells[index], true)));
            res.push_str(" | ");
        }
        res.push_str("\n");
    }
    draw_line(res);
    res.push_str("\n");
}

/**
 * Ersetzt Inline-Markdown-Marker in `text` durch HTML- oder ANSI-Tags.
 *
 * Iteriert über [`MD_MAP`] und führt `String::replace` für jedes
 * Paar mit nicht-leerem `md_end` aus.
 *
 * # Arguments
 * * `text`     – Roher Zellen-/Zeilentext.
 * * `use_ansi` – `true` → ANSI, `false` → HTML.
 *
 * # Returns
 * Der transformierte String.
 */
fn process_inline(text: &str, use_ansi: bool) -> String {
    let mut result = text.to_string();
    for m in MD_MAP {
        if !m.md_end.is_empty() {
            let placeholder = format!("{}{}{}", m.md_start, "{}", m.md_end);
            let replacement = if use_ansi {
                format!("{}{}{}", m.ansi_start, "{}", m.ansi_end)
            } else {
                format!("{}{}{}", m.html_start, "{}", m.html_end)
            };
            result = result.replace(&placeholder, &replacement);
        }
    }
    result
}

/**
 * Erkennt eine Markdown-Überschrift (H1–H6) und liefert den
 * formatierten HTML- bzw. ANSI-String.
 *
 * # Arguments
 * * `line`     – Die zu prüfende Zeile.
 * * `use_ansi` – `true` → ANSI, `false` → HTML.
 *
 * # Returns
 * `Some(formatted)` wenn die Zeile eine Überschrift ist, sonst `None`.
 */
fn process_heading(line: &str, use_ansi: bool) -> Option<String> {
    for m in MD_MAP {
        if line.starts_with(m.md_start) && m.md_end.is_empty() {
            let content = line.trim_start_matches(m.md_start).trim();
            let processed = process_inline(content, use_ansi);
            if use_ansi {
                return Some(format!(
                    "\x1b[48;2;0;0;0m\x1b[38;2;255;255;255m{}\x1b[0m",
                    processed
                ));
            } else {
                return Some(format!("{}{}{}", m.html_start, processed, m.html_end));
            }
        }
    }
    None
}

/**
 * Prüft, ob `line` ein Listenpunkt ist (ungeordnet oder nummeriert)
 */
fn is_list_item(line: &str) -> Option<(usize, bool, String)> {
    log(&format!("Checking for list item: '{}'", line));
    let trimmed = line.trim_start();
    let indent = line.len() - trimmed.len();
    if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
        let content = trimmed[2..].trim().to_string();
        log(&format!("Unordered list item found: '{}'", content));
        return Some((indent, false, content));
    }
    if let Some(dot_pos) = trimmed.find(". ") {
        if dot_pos > 0 && trimmed[0..dot_pos].chars().all(|c| c.is_numeric()) {
            let content = trimmed[dot_pos + 2..].trim().to_string();
            log(&format!("Ordered list item found: '{}'", content));
            return Some((indent, true, content));
        }
    }
    None
}

/**
 * Prüft, ob `line` eine horizontale Linie ist
 * (`---`, `***` oder `___`, mindestens 3 Zeichen).
 *
 * # Arguments
 * * `line` – Die zu prüfende Zeile.
 *
 * # Returns
 * `true`, wenn es eine horizontale Regel ist.
 */
fn is_horizontal_rule(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.len() < 3 {
        return false;
    }
    (trimmed.starts_with("---") && trimmed.chars().all(|c| c == '-' || c.is_whitespace())) ||
    (trimmed.starts_with("***") && trimmed.chars().all(|c| c == '*' || c.is_whitespace())) ||
    (trimmed.starts_with("___") && trimmed.chars().all(|c| c == '_' || c.is_whitespace()))
}

/**
 * Wandelt Markdown-Tabellenzeilen in ein HTML-`<table>` um.
 *
 * Die erste Zeile (oder Zeilen mit `---`) werden als `<thead>` behandelt,
 * alle weiteren als `<tbody>`-Zeilen. Zellen werden über
 * process_inline formatiert.
 *
 * # Arguments
 * * `lines` – Die Rohzeilen der Tabelle.
 *
 * # Returns
 * Fertiges HTML-Tabellen-Fragment.
 */
fn process_table(lines: &[&str]) -> String {
    log(&format!("Processing table with {} rows", lines.len()));
    let mut html = String::from("<table style=\"border-collapse: collapse; border: 1px solid #ccc; margin: 1em 0;\">\n");
    for (i, line) in lines.iter().enumerate() {
        if !line.trim().starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line.split('|')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim())
            .collect();
        if cells.is_empty() {
            continue;
        }
        if i == 0 || line.contains("---") {
            html.push_str("<thead><tr>");
            for cell in &cells {
                html.push_str(&format!("<th style=\"border: 1px solid #ccc; padding: 10px; text-align: left;\">{}</th>", process_inline(cell, false)));
            }
            html.push_str("</tr></thead>\n");
        } else {
            html.push_str("<tr>");
            for cell in &cells {
                html.push_str(&format!("<td style=\"border: 1px solid #ccc; padding: 10px;\">{}</td>", process_inline(cell, false)));
            }
            html.push_str("</tr>\n");
        }
    }
    html.push_str("</table>\n");
    html
}

/**
 * Liest einen Fenced-Code-Block (` ``` ` … ` ``` `) und gibt ihn
 * als HTML-`<pre><code>` zurück.
 *
 * `start_idx` wird über den schließenden Fence hinaus erhöht.
 *
 * # Arguments
 * * `lines`     – Alle Zeilen des Dokuments.
 * * `start_idx` – Index der öffnenden ` ``` `-Zeile (wird mutiert).
 *
 * # Returns
 * HTML-Fragment des Code-Blocks.
 */
fn process_code_block(lines: &[&str], start_idx: &mut usize) -> String {
    log("Code block detected");
    let mut html = String::from("<pre><code>");
    *start_idx += 1;
    while *start_idx < lines.len() && !lines[*start_idx].starts_with("```") {
        html.push_str(lines[*start_idx]);
        html.push('\n');
        *start_idx += 1;
    }
    html.push_str("</code></pre>\n");
    *start_idx += 1;
    html
}

/**
 * Liest aufeinanderfolgende Listenpunkte und gibt sie als
 * HTML-`<ul>` zurück.
 *
 * `start_idx` wird hinter den letzten Listenpunkt gesetzt.
 *
 * # Arguments
 * * `lines`     – Alle Zeilen des Dokuments.
 * * `start_idx` – Index des ersten Listenpunkts (wird mutiert).
 *
 * # Returns
 * HTML-Fragment der Liste.
 */
fn process_list(lines: &[&str], start_idx: &mut usize) -> String {
    log("Starting new list");
    let mut html = String::from("<ul>\n");
    while *start_idx < lines.len() {
        if let Some((_, _, content)) = is_list_item(lines[*start_idx]) {
            html.push_str(&format!("  <li>{}</li>\n", process_inline(&content, false)));
            *start_idx += 1;
        } else if lines[*start_idx].trim().is_empty() {
            *start_idx += 1;
        } else {
            break;
        }
    }
    html.push_str("</ul>\n");
    html
}

/**
 * Konvertiert den gesamten Markdown-Text nach ANSI (Terminal-Ausgabe).
 *
 * Unterstützt: Überschriften, ungeordnete Listen, Tabellen (Kartenlayout)
 * und Inline-Formatierung. Zeilen werden nacheinander verarbeitet.
 *
 * # Arguments
 * * `md_content` – Roher Markdown-Quelltext.
 *
 * # Returns
 * ANSI-formatierter String.
 */
fn md_to_ansi(md_content: &str) -> String {
    let mut output = String::new();
    let lines: Vec<&str> = md_content.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.trim().starts_with('|') && i + 1 < lines.len() && lines[i + 1].contains("---") {
            let mut table_lines = Vec::new();
            while i < lines.len() && lines[i].trim().starts_with('|') {
                table_lines.push(lines[i]);
                i += 1;
            }
            process_table_ansi(&mut output, &table_lines);
            continue;
        }
        if let Some(h) = process_heading(line, true) {
            output.push_str(&h);
            output.push('\n');
            i += 1;
            continue;
        }
        let trimmed = line.trim_start();
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
            let content = trimmed[2..].trim();
            output.push_str(&format!("  • {}\n", process_inline(content, true)));
            i += 1;
            continue;
        }
        let processed = process_inline(line, true);
        if !processed.trim().is_empty() {
            output.push_str(&processed);
            output.push('\n');
        } else if line.trim().is_empty() {
            output.push('\n');
        }
        i += 1;
    }
    output
}

/**
 * Konvertiert den gesamten Markdown-Text nach HTML
 * (vollständiges Dokument mit `<head>` und CSS).
 *
 * Unterstützt: horizontale Linien, Überschriften, Listen,
 * Tabellen, Fenced-Code-Blöcke und Inline-Formatierung.
 *
 * # Arguments
 * * `md_content` – Roher Markdown-Quelltext.
 *
 * # Returns
 * Vollständiges HTML-Dokument als String.
 */
fn md_to_html(md_content: &str) -> String {
    log("Starting Markdown to HTML conversion");
    let mut html = String::from(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Markdown Preview</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif; max-width: 980px; margin: 40px auto; padding: 0 20px; line-height: 1.7; }
        h1, h2, h3 { border-bottom: 1px solid #ddd; padding-bottom: 0.3em; }
        ul, ol { padding-left: 2em; }
        li { margin: 0.35em 0; }
        table { border-collapse: collapse; margin: 1em 0; }
        th, td { border: 1px solid #ccc; padding: 10px; }
        pre { background: #f6f8fa; padding: 16px; overflow: auto; border-radius: 6px; }
        code { background: #f6f8fa; padding: 0.2em 0.4em; border-radius: 3px; }
        hr { border: 0; border-top: 3px double #333; margin: 2em 0; }
    </style>
</head>
<body class="markdown-body">
"#);
    let lines: Vec<&str> = md_content.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if is_horizontal_rule(line) {
            html.push_str("<hr>\n");
            i += 1;
            continue;
        }
        if let Some(heading) = process_heading(line, false) {
            html.push_str(&heading);
            html.push('\n');
            i += 1;
            continue;
        }
        if is_list_item(line).is_some() {
            let list_html = process_list(&lines, &mut i);
            html.push_str(&list_html);
            continue;
        }
        if line.trim().starts_with('|') && i + 1 < lines.len() && lines[i + 1].contains("---") {
            let mut table_lines = Vec::new();
            while i < lines.len() && lines[i].trim().starts_with('|') {
                table_lines.push(lines[i]);
                i += 1;
            }
            html.push_str(&process_table(&table_lines));
            continue;
        }
        if line.starts_with("```") {
            let code_html = process_code_block(&lines, &mut i);
            html.push_str(&code_html);
            continue;
        }
        let processed = process_inline(line, false);
        if !processed.trim().is_empty() {
            html.push_str(&processed);
            html.push('\n');
        } else if line.trim().is_empty() {
            html.push_str("<br>\n");
        }
        i += 1;
    }
    html.push_str("</body></html>");
    html
}

/**
 * Öffentliche Einstiegsfunktion: Markdown to HTML.
 *
 * Ruft md_to_html auf und schreibt das Ergebnis in den output buffer
 *
 * # Arguments
 * * `md_content` – Roher Markdown-Quelltext.
 * * `output`     – Ziel-String für das HTML-Dokument.
 */
pub fn md_html_main(md_content: &str, output: &mut String) {
    let html = md_to_html(md_content);
    output.clear();
    output.push_str(&html);
}

/**
 * Öffentliche Einstiegsfunktion: Markdown to ANSI.
 *
 * Ruft md_to_ansi auf und schreibt das Ergebnis nach `output`
 * (vorher wird `output` geleert).
 *
 * # Arguments
 * * `md_content` – Roher Markdown-Quelltext.
 * * `output`     – buffer ptr for result
 */
pub fn md_ansi_main(md_content: &str, output: &mut String) {
    let ansi = md_to_ansi(md_content);
    output.clear();
    output.push_str(&ansi);
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Prüft, dass H1-Überschriften korrekt nach HTML und ANSI
     * umgewandelt werden.
     */
    #[test]
    fn test_heading_h1() {
        let md = "# Hello World";
        let mut html = String::new();
        md_html_main(md, &mut html);
        assert!(html.contains("<h1>Hello World</h1>"), "H1 HTML fehlt: {}", html);
        let mut ansi = String::new();
        md_ansi_main(md, &mut ansi);
        assert!(ansi.contains("Hello World"), "H1 ANSI Text fehlt: {}", ansi);
        assert!(ansi.contains("\x1b[0m"), "ANSI-Reset fehlt");
    }

    /**
     * Prüft Inline-Fett (`**…**`) in HTML und ANSI.
     */
    #[test]
    fn test_inline_bold() {
        let md = "This is **bold** text";
        let mut html = String::new();
        md_html_main(md, &mut html);
        assert!(
            html.contains("<strong>bold</strong>") || html.contains("**bold**"),
                "Bold HTML nicht wie erwartet: {}",
                html
        );
        let mut ansi = String::new();
        md_ansi_main(md, &mut ansi);
        assert!(ansi.contains("bold"), "Bold-Text fehlt in ANSI: {}", ansi);
    }

    /**
     * Prüft ungeordnete Listen (`- item`).
     */
    #[test]
    fn test_unordered_list() {
        let md = "- First\n- Second\n- Third";
        let mut html = String::new();
        md_html_main(md, &mut html);
        assert!(html.contains("<ul>"), "ul-Tag fehlt");
        assert!(html.contains("<li>First</li>"), "Erstes li fehlt");
        assert!(html.contains("<li>Second</li>"), "Zweites li fehlt");
        assert!(html.contains("</ul>"), "schließendes ul fehlt");
    }

    /**
     * Prüft horizontale Linien (`---`).
     */
    #[test]
    fn test_horizontal_rule() {
        assert!(is_horizontal_rule("---"));
        assert!(is_horizontal_rule("***"));
        assert!(is_horizontal_rule("___"));
        assert!(!is_horizontal_rule("--"));
        assert!(!is_horizontal_rule("hello"));
        let md = "Before\n---\nAfter";
        let mut html = String::new();
        md_html_main(md, &mut html);
        assert!(html.contains("<hr>"), "hr-Tag fehlt: {}", html);
    }

    /**
     * Prüft is_list_item für ungeordnete und nummerierte Punkte.
     */
    #[test]
    fn test_is_list_item() {
        let (indent, ordered, content) = is_list_item("- hello").unwrap();
        assert_eq!(indent, 0);
        assert!(!ordered);
        assert_eq!(content, "hello");
        let (indent, ordered, content) = is_list_item("  * world").unwrap();
        assert_eq!(indent, 2);
        assert!(!ordered);
        assert_eq!(content, "world");
        let (indent, ordered, content) = is_list_item("1. numbered").unwrap();
        assert_eq!(indent, 0);
        assert!(ordered);
        assert_eq!(content, "numbered");
        assert!(is_list_item("no list").is_none());
    }

    /**
     * Prüft Fenced-Code-Blöcke.
     */
    #[test]
    fn test_code_block() {
        let md = "```\nfn main() {}\n```";
        let mut html = String::new();
        md_html_main(md, &mut html);
        assert!(html.contains("<pre><code>"), "pre/code öffnend fehlt");
        assert!(html.contains("fn main() {}"), "Code-Inhalt fehlt");
        assert!(html.contains("</code></pre>"), "pre/code schließend fehlt");
    }

    /**
     * Prüft eine einfache Markdown-Tabelle nach HTML.
     */
    #[test]
    fn test_table_html() {
        let md = "| Name | Age |\n|------|-----|\n| Alice | 30 |\n| Bob | 25 |";
        let mut html = String::new();
        md_html_main(md, &mut html);
        assert!(html.contains("<table"), "table-Tag fehlt");
        assert!(html.contains("<th"), "th-Tag fehlt");
        assert!(html.contains("Alice"), "Zelleninhalt fehlt");
        assert!(html.contains("</table>"), "schließendes table fehlt");
    }

    /**
     * Prüft, dass eine H1-Überschrift korrekt nach HTML umgewandelt wird.
     */
    #[test]
    fn test_mixed_heading() {
        let md = "# Title";
        let mut html = String::new();
        md_html_main(md, &mut html);
        assert!(html.contains("<h1>Title</h1>"), "H1 fehlt: {}", html);
        assert!(html.contains("</body></html>"), "HTML-Footer fehlt");
    }

    /**
     * Prüft Inline-Formatierung (Fett + Kursiv) in einem Absatz.
     */
    #[test]
    fn test_mixed_inline() {
        let md = "Some **bold** and *italic* text.";
        let mut html = String::new();
        md_html_main(md, &mut html);
        assert!(
            html.contains("bold") && html.contains("italic"),
                "Inline-Text fehlt: {}",
                html
        );
        assert!(html.contains("</body></html>"), "HTML-Footer fehlt");
    }

    /**
     * Prüft eine ungeordnete Liste mit zwei Einträgen.
     */
    #[test]
    fn test_mixed_list() {
        let md = "- Item A\n- Item B";
        let mut html = String::new();
        md_html_main(md, &mut html);
        assert!(html.contains("<ul>"), "ul-Tag fehlt: {}", html);
        assert!(html.contains("<li>Item A</li>"), "Item A fehlt: {}", html);
        assert!(html.contains("<li>Item B</li>"), "Item B fehlt: {}", html);
        assert!(html.contains("</ul>"), "schließendes ul fehlt: {}", html);
    }

    /**
     * Prüft eine horizontale Linie (`----`).
     */
    #[test]
    fn test_mixed_horizontal_rule() {
        let md = "----";
        let mut html = String::new();
        md_html_main(md, &mut html);
        assert!(html.contains("<hr>"), "hr-Tag fehlt: {}", html);
        assert!(html.contains("</body></html>"), "HTML-Footer fehlt");
    }

    /**
     * Prüft, dass ein leerer / nur-Whitespace-Block
     * trotzdem ein gültiges HTML-Dokument erzeugt.
     */
    #[test]
    fn test_mixed_empty() {
        let md = "   \n\n   ";
        let mut html = String::new();
        md_html_main(md, &mut html);
        assert!(html.contains("<!DOCTYPE html>"), "DOCTYPE fehlt");
        assert!(html.contains("</body></html>"), "HTML-Footer fehlt");
    }
}
