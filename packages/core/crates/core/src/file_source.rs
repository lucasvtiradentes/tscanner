use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    JavaScript,
    TypeScript,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageVariant {
    Standard,
    Jsx,
}

#[derive(Debug, Clone, Copy)]
pub struct FileSource {
    language: Language,
    variant: LanguageVariant,
}

impl FileSource {
    pub fn from_path(path: &Path) -> Self {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        match ext {
            "ts" => Self {
                language: Language::TypeScript,
                variant: LanguageVariant::Standard,
            },
            "tsx" => Self {
                language: Language::TypeScript,
                variant: LanguageVariant::Jsx,
            },
            "js" | "mjs" | "cjs" => Self {
                language: Language::JavaScript,
                variant: LanguageVariant::Standard,
            },
            "jsx" => Self {
                language: Language::JavaScript,
                variant: LanguageVariant::Jsx,
            },
            _ => Self {
                language: Language::TypeScript,
                variant: LanguageVariant::Standard,
            },
        }
    }

    pub fn is_typescript(&self) -> bool {
        matches!(self.language, Language::TypeScript)
    }

    pub fn is_javascript(&self) -> bool {
        matches!(self.language, Language::JavaScript)
    }

    pub fn is_jsx(&self) -> bool {
        matches!(self.variant, LanguageVariant::Jsx)
    }
}
