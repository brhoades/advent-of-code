use std::ops::Range;
use std::borrow::Borrow;

use anyhow::Result;

enum Either<T> {
    Left(T),
    Right(T),
}

use Either::*;

/// we solve 06 pt 1 by splitting the bytes (no fancy utf8, sorry!)
/// and comparing them in windows until 4 are different.
pub fn run(input: String) -> Result<()> {
    let input = input.trim();

    // fold keeps tracks of # of windows we've looked at,
    // which is the answer: at what character does the message
    // begin?
    let result: Either<usize> = input
        .as_bytes()
        .windows(4)
        // sets acc to Right after finding, which halts further adds
        // otherwise acc is Left(#) after run
        .fold(Left(4), |acc, sl| {
            match acc {
                Left(acc) => {
                    if sl[0] != sl[1]
                        && sl[0] != sl[2]
                        && sl[0] != sl[3]
                        && sl[1] != sl[2]
                        && sl[1] != sl[3]
                        && sl[2] != sl[3] {
                            Right(acc)
                        } else {
                            Left(acc + 1)
                        }
                },
                other => other,
                }
        });

    match result {
        Left(pos) => println!("read {} characters and failed to find start message", pos),
        Right(pos) => {
            println!("found message after {} characters:", pos);
            println!("message: {}\n", input[(pos-4 as usize)..(pos as usize)].to_string());
            println!("neighboring input: ...{}...", str_with_context(input, pos-8..pos+12));

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
