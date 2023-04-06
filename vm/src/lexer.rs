pub struct Lexer<'a> {
    pub source: &'a str,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Word<'a> {
    pub data: &'a str,
    pub line: usize,
    pub column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            line: 1,
            column: 1,
        }
    }

    pub fn skip_whitespace(&mut self) {
        let index = self.source.find(|c| c != '\t' && c != ' ' && c != '\n');

        if let Some(index) = index {
            self.source.chars().into_iter().take(index).for_each(|c| {
                match c {
                    '\t' | ' ' => self.increment_column(),
                    '\n' => self.increment_line(),
                    _ => { unreachable!() }
                }
            });
            self.source = &self.source[index..self.source.len()]
        } else {
            self.source = ""
        }
    }

    pub fn next_word(&mut self) -> Option<Word<'a>> {
        self.skip_whitespace();
        if self.source == "" { return None }
        let index = self.source.find(|c| c == '\t' || c == ' ' || c == '\n');

        let word = if let Some(index) = index {
            let word = &self.source[0..index];
            self.source = &self.source[index..self.source.len()];
            word
        } else {
            let word = &self.source[0..self.source.len()];
            self.source = "";

            word
        };

        let word = Word {
            data: word,
            line: self.line,
            column: self.column,
        };

        self.column += word.data.len();
        Some(word)
    }

    fn increment_line(&mut self) {
        self.column = 1;
        self.line += 1;
    }

    fn increment_column(&mut self) {
        self.column += 1;
    }
}