use crate::language::Language;
use std::path::Path;

const DOCKERFILE_KEY_WORD: &str = "Dockerfile";
const DOCKER_COMPOSE_KEYWORD: &str = "docker-compose.yml";

/**
 * Detects the programming or markup language of a file based on its name.
 *
 * The detection works in two stages:
 * 1. By file extension (e.g. `.py` → Python, `.rs` → Rust).
 * 2. By exact (case-insensitive) basename for special files
 *    (`Dockerfile`, `docker-compose.yml`).
 *
 * If the filename is empty or no known pattern matches: Language::Plain
 * is returned.
 *
 * # Arguments
 *
 * * `filename` – Path or bare filename to inspect. May contain directories;
 *   only the last component (basename) and its extension are considered.
 *
 * # Examples
 *
 * ```
 * assert_eq!(detect_language("main.py"), Language::Python);
 * assert_eq!(detect_language("Dockerfile"), Language::Docker);
 * assert_eq!(detect_language("unknown.xyz"), Language::Plain);
 * ```
 */
pub fn detect_language(filename: &str) -> Language {
    if filename.is_empty() {
        return Language::Plain;
    }
    let path = Path::new(filename);
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        match ext {
            "py" => return Language::Python,
            "js" => return Language::JavaScript,
            "c" | "h" | "cpp" | "ino" => return Language::Clang,
            "md" => return Language::Markdown,
            "html" | "htm" | "xml" => return Language::Html,
            "sh" => return Language::Shell,
            "asm" => return Language::AsmX86,
            "a51" => return Language::Asm8051,
            "sql" => return Language::Sql,
            "rs" => return Language::Rust,
            "log" => return Language::LogFile,
            "java" => return Language::LangJava,
            _ => {}
        }
    }
    let base = path.file_name().and_then(|s| s.to_str()).unwrap_or(filename);
    if base.eq_ignore_ascii_case(DOCKERFILE_KEY_WORD) {
        return Language::Docker;
    }
    if base.eq_ignore_ascii_case(DOCKER_COMPOSE_KEYWORD) {
        return Language::LangDockerCompose;
    }
    Language::Plain
}


#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Verifies that common file extensions are mapped to the correct
     * Language variant.
     */
    #[test]
    fn detects_common_extensions() {
        assert_eq!(detect_language("main.py"), Language::Python);
        assert_eq!(detect_language("app.js"), Language::JavaScript);
        assert_eq!(detect_language("lib.rs"), Language::Rust);
        assert_eq!(detect_language("Main.java"), Language::LangJava);
        assert_eq!(detect_language("script.sh"), Language::Shell);
        assert_eq!(detect_language("README.md"), Language::Markdown);
        assert_eq!(detect_language("index.html"), Language::Html);
        assert_eq!(detect_language("query.sql"), Language::Sql);
        assert_eq!(detect_language("boot.asm"), Language::AsmX86);
        assert_eq!(detect_language("timer.a51"), Language::Asm8051);
        assert_eq!(detect_language("error.log"), Language::LogFile);
        assert_eq!(detect_language("main.c"), Language::Clang);
        assert_eq!(detect_language("header.h"), Language::Clang);
        assert_eq!(detect_language("sketch.ino"), Language::Clang);
    }

    /**
     * Verifies Docker-related special filenames (case-insensitive),
     * empty input, unknown extensions, and paths that contain a directory.
     */
    #[test]
    fn detects_docker_files_and_fallbacks() {
        assert_eq!(detect_language("Dockerfile"), Language::Docker);
        assert_eq!(detect_language("dockerfile"), Language::Docker);
        assert_eq!(detect_language("DOCKERFILE"), Language::Docker);
        assert_eq!(
            detect_language("docker-compose.yml"),
                   Language::LangDockerCompose
        );
        assert_eq!(
            detect_language("Docker-Compose.yml"),
                   Language::LangDockerCompose
        );
        assert_eq!(detect_language(""), Language::Plain);
        assert_eq!(detect_language("unknown.xyz"), Language::Plain);
        assert_eq!(detect_language("Makefile"), Language::Plain);
        assert_eq!(detect_language("path/to/Dockerfile"), Language::Docker);
        assert_eq!(
            detect_language("path/to/docker-compose.yml"),
                   Language::LangDockerCompose
        );
    }
}
