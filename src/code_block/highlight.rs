use std::sync::OnceLock;

use syntect::{
    easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet, util::LinesWithEndings,
};

pub(super) struct Token {
    pub text: String,
    pub rgba: Option<u32>,
    pub dim: bool,
}

pub(super) fn syntax(source: &str, language: &str, dark: bool) -> Vec<Vec<Token>> {
    static SYNTAXES: OnceLock<SyntaxSet> = OnceLock::new();
    static THEMES: OnceLock<ThemeSet> = OnceLock::new();

    let grammars = SYNTAXES.get_or_init(two_face::syntax::extra_newlines);
    let Some(grammar) = grammars.find_syntax_by_token(language) else {
        return Vec::new();
    };
    let themes = THEMES.get_or_init(ThemeSet::load_defaults);
    let theme_name = if dark {
        "base16-ocean.dark"
    } else {
        "InspiredGitHub"
    };
    let mut parser = HighlightLines::new(grammar, &themes.themes[theme_name]);
    LinesWithEndings::from(source)
        .map(|line| {
            parser
                .highlight_line(line, grammars)
                .ok()
                .into_iter()
                .flatten()
                .filter_map(|(style, text)| {
                    let text = text.trim_end_matches(['\r', '\n']);
                    let c = style.foreground;
                    (!text.is_empty()).then(|| Token {
                        text: text.to_owned(),
                        rgba: Some(u32::from_be_bytes([c.r, c.g, c.b, c.a])),
                        dim: false,
                    })
                })
                .collect()
        })
        .collect()
}

#[derive(Default)]
struct Ansi {
    color: Option<u32>,
    dim: bool,
}

impl Ansi {
    fn line(&mut self, mut line: &str) -> Vec<Token> {
        const COLORS: [u32; 8] = [
            0x555555ff, 0xdc2626ff, 0x15803dff, 0xa16207ff, 0x2563ebff, 0xa21cafff, 0x0e7490ff,
            0x737373ff,
        ];
        let mut tokens = Vec::new();
        let mut push = |text: &str, state: &Ansi| {
            if !text.is_empty() {
                tokens.push(Token {
                    text: text.to_owned(),
                    rgba: state.color,
                    dim: state.dim,
                });
            }
        };
        while let Some((before, sequence)) = line.split_once("\x1b[") {
            push(before, self);
            let Some((codes, rest)) = sequence.split_once('m') else {
                push(sequence, self);
                return tokens;
            };
            for code in codes.split(';').map(|part| part.parse().unwrap_or(0)) {
                match code {
                    0 => *self = Self::default(),
                    2 => self.dim = true,
                    22 => self.dim = false,
                    30..=37 => self.color = Some(COLORS[code - 30]),
                    39 => self.color = None,
                    90..=97 => self.color = Some(COLORS[code - 90]),
                    _ => {}
                }
            }
            line = rest;
        }
        push(line, self);
        tokens
    }
}

pub(super) fn ansi(source: &str) -> Vec<Vec<Token>> {
    let mut state = Ansi::default();
    source.split('\n').map(|line| state.line(line)).collect()
}

pub fn strip_ansi(source: &str) -> String {
    ansi(source)
        .into_iter()
        .map(|line| line.into_iter().map(|token| token.text).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}
