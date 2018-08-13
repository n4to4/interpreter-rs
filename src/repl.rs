use super::lexer::Lexer;
use std::io::{BufRead, Write};

const PROMPT: &[u8] = b">> ";

pub fn start<R, W>(mut reader: R, mut writer: W)
where
    R: BufRead,
    W: Write,
{
    loop {
        writer.write(PROMPT).unwrap();
        writer.flush().unwrap();

        let mut line = String::new();
        if let Ok(_) = reader.read_line(&mut line) {
            let mut l = Lexer::new(line);
            l.for_each(|tok| {
                writer.write_fmt(format_args!("{:?}\n", tok)).unwrap();
                writer.flush().unwrap();
            });
        } else {
            return;
        }
    }
}
