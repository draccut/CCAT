#[derive(Clone)]
/**
 * Konfiguration der Farben und Formatierungszeichen für Syntax-Highlighting.
 *
 * Enthält ANSI-Escape-Sequenzen (Terminal) oder HTML-Spans,
 * je nachdem welche Factory-Methode verwendet wird.
 */
pub struct HighlightConfig {
    pub color_keyword: &'static str,
    pub color_string: &'static str,
    pub color_comment: &'static str,
    pub color_header: &'static str,
    pub color_tag: &'static str,
    pub color_numbers: &'static str,
    pub color_reset: &'static str,
    pub color_ip:   &'static str,
    pub color_timestamp: &'static str,
    pub color_hex: &'static str,
    pub new_line: &'static str
}

impl HighlightConfig {
    /**
     * Erzeugt eine Konfiguration mit HTML-`<span>`-Elementen
     * für die Darstellung im Browser.
     *
     * # Returns
     * Eine [`HighlightConfig`] mit HTML-Farbcodes und `<br>` als Zeilenumbruch.
     */
    pub fn html_colors() -> Self {
        Self {
            color_keyword: "<span style='color: #0000DD; font-weight: bold;'>",
            color_string:  "<span style='color: #855500;'>",
            color_comment: "<span style='color: #ffff00; font-style: italic;'>",
            color_header:  "<span style='color: #75803B;'>",
            color_tag:     "<span style='color: #A70022;'>",
            color_numbers: "<span style='color: #ff0000;'>",
            color_reset:   "</span>",
            color_hex:     "<span style='color: #ff0000'>",
            color_ip:      "<span style='color: #ffff00'>",
            color_timestamp: "<span style='color: #555555'>",
            new_line:      "<br>",
        }
    }
}

impl Default for HighlightConfig {
    /**
     * Erzeugt die Standard-Konfiguration mit ANSI-24-Bit-Farben
     * für die Ausgabe im Terminal.
     *
     * # Returns
     * Eine [`HighlightConfig`] mit Neon-Farbschema und `\n` als Zeilenumbruch.
     */
    fn default() -> Self {
        Self {
            color_keyword:   "\x1b[38;2;0;255;255m",   // Neon-Cyan
            color_string:    "\x1b[38;2;255;255;85m",  // Neon-Gelb
            color_comment:   "\x1b[38;2;180;180;180m", // Helles Grau
            color_header:    "\x1b[38;2;255;0;255m",   // Knalliges Magenta
            color_tag:       "\x1b[38;2;255;85;85m",   // Neon-Rot
            color_numbers:   "\x1b[38;2;255;0;0m",     // Rot
            color_reset:     "\x1b[0m",                // Reset
            color_ip:        "\x1b[38;2;0;170;255m",   // Leuchtendes Blau
            color_timestamp: "\x1b[38;2;200;100;255m", // Lila/Violett
            color_hex:       "\x1b[38;2;220;220;220m", // Helles Silber
            new_line:        "\n",
        }
    }
}

