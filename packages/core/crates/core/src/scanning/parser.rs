use crate::utils::FileSource;
use anyhow::Result;
use std::path::Path;
use swc_common::{sync::Lrc, FileName, SourceMap};
use swc_ecma_ast::Program;
use swc_ecma_parser::{lexer::Lexer, EsSyntax, Parser, StringInput, Syntax, TsSyntax};

pub fn parse_file(path: &Path, source: &str) -> Result<Program> {
    let file_source = FileSource::from_path(path);
    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.new_source_file(
        FileName::Real(path.to_path_buf()).into(),
        source.to_string(),
    );

    let syntax = if file_source.is_typescript() {
        Syntax::Typescript(TsSyntax {
            tsx: file_source.is_jsx(),
            decorators: true,
            dts: false,
            no_early_errors: true,
            disallow_ambiguous_jsx_like: false,
        })
    } else {
        Syntax::Es(EsSyntax {
            jsx: file_source.is_jsx(),
            decorators: true,
            decorators_before_export: true,
            ..Default::default()
        })
    };

    let lexer = Lexer::new(syntax, Default::default(), StringInput::from(&*fm), None);

    let mut parser = Parser::new_from(lexer);

    parser
        .parse_program()
        .map_err(|e| anyhow::anyhow!("Parse error: {:?}", e))
}
