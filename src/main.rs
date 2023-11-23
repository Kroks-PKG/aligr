use std::{
    env::args,
    io::{stdin, stdout, Write},
    process::exit,
};

fn align_normal(
    lines: &mut dyn Iterator<Item = String>,
    align_word: &str,
    is_taild: bool,
) -> (Vec<(&'static str, &'static str)>, usize) {
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

    let (lines_split, max_align) =
        align_normal(&mut stdin().lines().flatten(), &align_word, is_taild);

    let mut stdout = stdout().lock();

    for (left, right) in lines_split {
        writeln!(
            stdout,
            "{:<width$}{}{}",
            left,
            align_word,
            right,
            width = max_align
        )
        .unwrap_or_else(|err| {
            eprintln!("Error writing to stdout: {}", err);
            exit(1);
        })
    }
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

        let (lines_split, max_align) = align_normal(&mut lines, "world", false);

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

        let (lines_split, max_align) = align_normal(&mut lines, "world", true);

        assert_eq!(max_align, 13);

        assert_eq!(lines_split[0].0, "Hello, world ");
        assert_eq!(lines_split[0].1, "!");

        assert_eq!(lines_split[1].0, "Hello, ");
        assert_eq!(lines_split[1].1, "!");

        assert_eq!(lines_split[2].0, "Hello, ");
        assert_eq!(lines_split[2].1, "!");
    }
}
