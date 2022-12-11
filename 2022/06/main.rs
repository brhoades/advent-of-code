use std::ops::Range;
use std::borrow::Borrow;
use std::collections::HashSet;

use anyhow::Result;

enum Either<T> {
    Left(T),
    Right(T),
}

use Either::*;

const WINDOW_SIZE: usize = 14;

/// we solve 06 pt 2 by splitting the bytes (no fancy utf8, sorry!)
/// and comparing them in windows until 12 are different.
pub fn run(input: String) -> Result<()> {
    let input = input.trim();

    // fold keeps tracks of # of windows we've looked at,
    // which is the answer: at what character does the message
    // begin?
    // A HashSet is passed along to avoid extra reallocs.
    let (_, result): (HashSet<&u8>, Either<usize>) = input
        .as_bytes()
        .windows(WINDOW_SIZE)
        // sets acc to Right after finding, which halts further adds
        // otherwise acc is Left(#) after run
        .fold((HashSet::with_capacity(WINDOW_SIZE), Left(WINDOW_SIZE)), |acc, sl| {
            match acc {
                (mut hs, Left(acc)) => {
                    hs.clear();
                    for c in sl {
                        if hs.contains(c) {
                            return (hs, Left(acc + 1));
                        }
                        hs.insert(c);
                    }

                    (hs, Right(acc))
                },
                other => other,
            }
        });

    match result {
        Left(pos) => println!("read {} characters and failed to find start message", pos),
        Right(pos) => {
            println!("found message after {} characters:", pos);
            println!("...{}_{}_{}...",
                     str_with_context(input, pos-WINDOW_SIZE*2..pos),
                     input[pos-WINDOW_SIZE..pos].to_string(),
                     str_with_context(input, pos..pos+WINDOW_SIZE * 2));

        }
    }
    Ok(())
}

fn str_with_context<I: Borrow<str>>(s: I, mut rng: Range<usize>) -> String {
    let s = s.borrow();

    if rng.end > s.len() {
        rng.end = s.len()
    }

    s[rng].to_string()
}
