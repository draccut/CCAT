# CCAT

Syntax-Highlighter für Quellcode, Hex-Dump, Logfiles und Markdown

----

## 1. Architektur-Überblick

CCAT ist als mehrschichtiges CLI-Werkzeug aufgebaut:

```
main.rs
 ├── detect_filetype   -> Language-Enum aus Dateiname/Extension
 ├── highlighter       -> zeilenweises Token-Highlighting
 ├── specials          -> Hex-Dump, Log-Highlight, C-Array-Export
 ├── ccat_markdown     -> Markdown → ANSI / HTML
 ├── col               -> HighlightConfig (ANSI-24-bit / HTML-Spans)
 ├── keywords          -> sprachspezifische Keyword-Tabellen
 └── state             -> Laufzeit-Flags (Zeilennummern, Binary, Stop-Line …)
```

Der Kontrollfluss in `main`:

1. Argumente parsen State befüllen
2. Sprache über detect_language(filename) bestimmen  
3. Datei einlesen (`read_file_with_hex_fallback`) – ungültige Bytes
   werden als ` 0xHH` substituiert  
4. Spezialpfade abarbeiten (`hex`, `binary`, `log`, `to_c_array`,
   `remove_ansi`, Markdown)  
5. Sonst: zeilenweises `highlight_line`  
6. Ergebnis auf stdout

----

## 2. Features

### 2.1 Syntax-Highlighting

| Sprache              | Kommentare      | Strings | Keywords |
|----------------------|-----------------|---------|----------|
| Python / Shell       | `#`             | ja      | ja       |
| JavaScript / Docker  | `//`            | ja      | ja       |
| C / C++ / Arduino    | `//` und `/* */`| ja      | ja       |
| SQL                  | `/* */`         | ja      | ja       |
| x86- / 8051-ASM      | `;`             | ja      | ja       |
| Rust / Java          | – (Keywords)    | ja      | ja       |
| HTML / XML           | –               | ja      | Tags     |
| Docker-Compose       | –               | ja      | ja       |
| Log-Dateien          | Timestamp / IP / Hex | –  | –        |
| Markdown             | eigener Parser  | –       | –        |

Keyword-Erkennung erfolgt über statische Tabellen (`keywords`-Modul)
und einen einfachen Identifier-Scanner (`is_word_char`).

### 2.2 Ausgabeformate

- **ANSI 24-bit** (Default) – Truecolor-Escape-Sequenzen
- **HTML** (`-h`) – `<span style="color:…">` bzw. vollständige
  Markdown-HTML-Dokumente mit CSS
- **Hex-Dump** (`-b` / `-x`) – Offset, Hex-Bytes, ASCII-Spalte
- **C-Array** (`-a`) – `const unsigned char BUFF[…] = { 0x.., … };`
- **ANSI-Strip** (`-r`) – Escape-Sequenzen entfernen

### 2.3 Steuerungsoptionen

| Flag | Wirkung |
|------|---------|
| `n`  | Zeilennummern (rechtsbündig, 1–3 Stellen) |
| `b`  | Binary-Hex-Dump über `print_binary_file` |
| `x`  | Alternative Hex-Implementierung (`print_hex_file`) |
| `a`  | Export als C-`unsigned char`-Array |
| `h`  | HTML-Farbcodes statt ANSI |
| `r`  | Alle ANSI-Sequenzen strippen |
| `-s` / `--stop <n>` | Ausgabe nach Zeile `n` abbrechen |

### 2.4 Markdown-Engine (`ccat_markdown`)

Eigener, dependency-freier Parser:

- Überschriften H1–H6
- Fett (`**` / `__`), Kursiv (`*` / `_`), Durchgestrichen (`~~`)
- Inline-Code (`` ` ``)
- Ungeordnete und nummerierte Listen
- Fenced Code-Blöcke (` ``` `)
- Tabellen (HTML-`<table>` und ANSI-Kartenlayout)
- Horizontale Linien (`---`, `***`, `___`)

Zwei öffentliche Einstiege:

- `md_html_main` → vollständiges HTML-Dokument
- `md_ansi_main` → Terminal-Ausgabe mit ANSI

----

## 3. Funktionsweise im Detail

### 3.1 Spracheerkennung

`detect_language` prüft zuerst die Dateiendung, danach den Basename
(case-insensitive) für `Dockerfile` und `docker-compose.yml`.
Unbekannte Dateien fallen auf `Language::Plain` zurück.

### 3.2 Highlight-Pipeline

`highlight_line` arbeitet zeichenweise mit einem `Peekable`-Iterator:

1. Abbruch, wenn `state.line_nr >= state.stop_at_line`
2. Optional Zeilennummer anhängen
3. Kommentar-Erkennung (sprachabhängig, inkl. mehrzeiliger `/* */`)
4. String-Literale inkl. Escape-Handling
5. HTML-Tag-Scanner (bei `Language::Html`)
6. Identifier sammeln Keyword-Lookup farbig oder plain
7. Restzeichen unverändert übernehmen
8. Newline aus der Config anhängen

Alle Farben kommen aus `HighlightConfig` (Default = ANSI-Neon,
`html_colors()` = CSS-Spans).

### 3.3 Binär und Spezialpfade

- `print_binary_file` – 8-Byte-Chunks, Offset, optionale Zeilennummer,
   Hex + ASCII, Padding bei kurzen Chunks
- `print_hex_file` – ähnliche Dump-Logik, direkt auf stdout
- `log_highlight_line` – erkennt `[timestamp]`, `[ip]`, `0xHH`
- `to_c_array` – Bytes als `0xXXXXXXXX`-Literale, 16 pro Zeile

### 3.4 Robuster Dateileser

`read_file_with_hex_fallback` dekodiert UTF-8 (1–4 Bytes).
Steuerzeichen `\n/\r/\t` und druckbares ASCII bleiben erhalten;
ungültige Sequenzen werden als ` 0xHH` eingefügt. Dadurch können
gemischte Text-/Binärdateien ohne Absturz angezeigt werden.

----

## 4. Testabdeckung

Die Tests sind modulweise organisiert und decken die kritischen
Pfade ab:

----

## 5. Abhängigkeiten und Build

Keine externen Crates. Nur `std` (fs, env, io, path).

Typischer Build:

```
cargo build --release
./target/release/ccat datei.rs n
./target/release/ccat dummy.asm
./target/release/ccat notes.md h
```

----

## 6. Speicherverwaltung in Rust (bezogen auf ccat)

Rust trennt strikt **Stack**, **Heap** und **statische Segmente**.
Ein typischer Aufruf wie `let s = String::from("hello");` erzeugt:

- **Stack (aktueller Stack-Frame der Funktion):**  
  Das `String`-Struct selbst – drei Pointer-große Felder  
  (`ptr: *mut u8`, `len: usize`, `capacity: usize`).  
  Auf 64-Bit: **24 Byte**. Diese 24 Byte liegen im Stack-Frame
  der umgebenden Funktion und werden beim `return` mit dem
  Frame freigegeben (Drop läuft vorher).

- **Heap:**  
  Der eigentliche UTF-8-Puffer (`"hello"` + ggf. Überkapazität).  
  Allokation zur **Laufzeit** über den globalen Allocator
  (`__rust_alloc` → in der Regel jemalloc/system-malloc).  
  Die Page, die den Puffer hält, ist `RW` (read/write),
  nicht executable. Bei `drop(s)` wird der Heap-Block
  freigegeben; die Stack-Felder verschwinden mit dem Frame.

- **Rodata / Text (Compile-Zeit):**  
  String-Literale wie `"hello"` in `String::from("hello")`
  oder `&'static str`-Konstanten (`INFO_TEXT`, Keyword-Tabellen,
  `MD_MAP`) liegen im **read-only Data-Segment** (`.rodata`).  
  Die zugehörigen Memory-Pages sind `R` (read-only), oft
  shared zwischen Prozessen, und werden **nicht** allokiert
  oder freigegeben. Ihre Adressen sind zur Link-Zeit fix.

**Was erzeugt einen Stack-Frame?**  
Jeder Funktionsaufruf (`main`, `highlight_line`, `md_to_html`, …).
Lokale Variablen (`let mut output = String::new()`, `let config = …`,
Schleifenzähler, `Peekable`-Iteratoren) leben in diesem Frame.
Referenzen (`&str`, `&HighlightConfig`, `&mut State`) sind nur
Pointer (8 Byte auf 64-Bit) und speichern keine Besitzdaten.

**Compile-Zeit vs. Runtime:**  
- Größe und Layout von Structs (`State`, `HighlightConfig`,
  `String`) sind zur Compile-Zeit bekannt (kein vtable-Lookup
  für diese Typen).  
- Allokationen (`String::from`, `Vec::push`, `format!`) und
  damit Heap-Traffic sind **rein runtime**.  
- `const` / `static` (z. B. `EXPECTED_*`-Arrays in Tests,
  `DOCKERFILE_KEY_WORD`) werden in `.rodata` bzw. `.data`
  gelegt und brauchen keinen Heap.

**Page-Flags (Linux, grob):**  

| Segment        | Typische Flags     | Inhalt in ccat                          |
|----------------|--------------------|-----------------------------------------|
| `.text`        | `r-x`              | Maschinencode der Funktionen            |
| `.rodata`      | `r--`              | Literale, Keyword-Tabellen, `INFO_TEXT` |
| Stack          | `rw-` (+ grow down)| Frames, `String`-Header, Lokale         |
| Heap           | `rw-`              | `String`-/`Vec`-Puffer, format!-Results |

Rust garantiert zur Compile-Zeit, dass kein Heap-Puffer
nach `drop` noch erreichbar ist (Ownership + Borrow-Checker).
Zur Laufzeit sieht der Prozess also dieselben Page-Flags wie
ein C-Programm mit `malloc`/`free` – nur dass die Freigabe
deterministisch am Scope-Ende passiert und nicht manuell
vergessen werden kann.

