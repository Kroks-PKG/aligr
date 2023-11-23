use std::{
    env::args,
    io::{stdin, stdout, Write},
    process::exit,
};

type Aligr = (Vec<(&'static str, &'static str)>, usize);

fn aligr(lines: &mut dyn Iterator<Item = String>, align_word: &str, is_taild: bool) -> Aligr {
    let mut max_align = 0;
    let mut lines_split = Vec::new();

    for pair in lines.map(|l| {
        let l = String::leak(l);
        if is_taild {
            l.rsplit_once(align_word).unwrap_or_else(|| (l, ""))
        } else {
            l.split_once(align_word).unwrap_or_else(|| (l, ""))
        }
    }) {
        if pair.0.len() > max_align {
            max_align = pair.0.len();
        }

        lines_split.push(pair);
    }

    (lines_split, max_align)
}

fn write_on(writer: &mut dyn Write, aligr: Aligr, word: &str) {
    for (left, right) in aligr.0 {
        writeln!(writer, "{:<width$}{}{}", left, word, right, width = aligr.1).unwrap_or_else(
            |err| {
                eprintln!("Error writing to stdout: {}", err);
                exit(1);
            },
        )
    }
}

fn main() {
    let mut args = args();

    let program_name = args.next().unwrap();

    let mut align_word = args.next().unwrap_or_else(|| {
        eprintln!("Please provide an alignment word!");
        eprintln!("Usage: {} <word>", program_name);
        exit(1);
    });

    let is_taild = if align_word == "-t" {
        align_word = args.next().unwrap_or_else(|| {
            eprintln!("Please provide an alignment word!");
            eprintln!("Usage: {} <word>", program_name);
            exit(1);
        });
        true
    } else {
        false
    };

    let aligr = aligr(&mut stdin().lines().flatten(), &align_word, is_taild);
    let mut stdout = stdout().lock();

    write_on(&mut stdout, aligr, &align_word);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_normal() {
        let mut lines = vec![
            "Hello, world world!".to_string(),
            "Hello, world!".to_string(),
            "Hello, world!".to_string(),
        ]
        .into_iter();

        let (lines_split, max_align) = aligr(&mut lines, "world", false);

        assert_eq!(max_align, 7);

        assert_eq!(lines_split[0].0, "Hello, ");
        assert_eq!(lines_split[0].1, " world!");

        assert_eq!(lines_split[1].0, "Hello, ");
        assert_eq!(lines_split[1].1, "!");

        assert_eq!(lines_split[2].0, "Hello, ");
        assert_eq!(lines_split[2].1, "!");
    }

    #[test]
    fn test_align_normal_taild() {
        let mut lines = vec![
            "Hello, world world!".to_string(),
            "Hello, world!".to_string(),
            "Hello, world!".to_string(),
        ]
        .into_iter();

        let (lines_split, max_align) = aligr(&mut lines, "world", true);

        assert_eq!(max_align, 13);

        assert_eq!(lines_split[0].0, "Hello, world ");
        assert_eq!(lines_split[0].1, "!");

        assert_eq!(lines_split[1].0, "Hello, ");
        assert_eq!(lines_split[1].1, "!");

        assert_eq!(lines_split[2].0, "Hello, ");
        assert_eq!(lines_split[2].1, "!");
    }

    #[test]
    fn test_write_on() {
        let mut lines = vec![
            "Hello, world world!".to_string(),
            "Hello, world!".to_string(),
            "Hello, world!".to_string(),
        ]
        .into_iter();

        let aligr = aligr(&mut lines, "world", false);

        let mut output = Vec::new();

        write_on(&mut output, aligr, "world");

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "Hello, world world!\nHello, world!\nHello, world!\n"
        );
    }

    #[test]
    fn test_write_on_taild() {
        let mut lines = vec![
            "Hello, world world!".to_string(),
            "Hello, world!".to_string(),
            "Hello, world!".to_string(),
        ]
        .into_iter();

        let aligr = aligr(&mut lines, "world", true);

        let mut output = Vec::new();

        write_on(&mut output, aligr, "world");

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "Hello, world world!\nHello,       world!\nHello,       world!\n"
        );
    }
}
