/**
 * Supported programming and markup languages that can be detected
 * from a filename (by extension or well-known special names).
 */
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Language {
    Plain,
    Python,
    JavaScript,
    Clang,
    Markdown,
    Html,
    Docker,
    Shell,
    AsmX86,
    Asm8051,
    Sql,
    Rust,
    LangDockerCompose,
    LangJava,
    LogFile
}
