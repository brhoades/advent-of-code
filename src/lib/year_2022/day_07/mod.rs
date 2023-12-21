mod tree;

use crate::prelude::*;
use std::collections::HashMap;

pub fn run(input: String) -> Result<()> {
    let fs = repl(input.split('\n').map(str::to_string).collect())?;

    let path_to_size: HashMap<_, _> = fs
        .iter()
        .filter_map(|(p, n)| match n {
            tree::Child::Folder(n) => Some((p, n)),
            _ => None,
        })
        .map(|(p, n)| (p, n.size()))
        .collect();

    // part 1: sum all directories with total size <= 100_000
    // this solution sucks... O(n^2)
    let soln = path_to_size
        .values()
        .filter(|s| **s <= 100_000)
        .sum::<usize>();

    println!("combined size of nodes <= 100k: {}", soln);

    // part 2: of our 70M, we need 30M free. Determine free space and find smallest node
    // to delete to achieve 30M free.
    let deficit = fs.size() - 40_000_000;
    let soln = path_to_size.values().filter(|s| **s >= deficit).min();
    println!("smallest directory size >= 30_000_000: {}", soln.unwrap());

    Ok(())
}

// repl runs some given input and returns the filesystem as represented
// by the input.
pub fn repl(input: Vec<String>) -> Result<tree::Filesystem> {
    let mut cwd = "/".to_string();
    // where the command targets
    let mut target: Option<String> = None;
    let mut fs = tree::Filesystem::default();

    for line in input {
        let parts: Vec<_> = line.split(' ').collect();

        match parts.as_slice() {
            &["$", "ls", path] | &["$", "dir", path] => {
                target = Some(path.to_string());
                println!("cwd: {}\ttarget: {:?}", cwd, target);
            }
            &["$", "dir"] => {
                target = None;
            }
            &["$", "ls"] => {
                target = None;
            }
            &["$", "cd", "/"] => {
                cwd = "/".to_string();
                target = None;
            }
            &["$", "cd", path] => {
                let mut cwdparts = cwd.split('/').collect::<Vec<_>>();
                cwd = if path == ".." {
                    cwdparts.pop();
                    cwdparts.join("/")
                } else {
                    cwdparts.extend(vec![path]);
                    cwdparts.join("/")
                };

                target = None;
            }
            &["dir", _] => {} // FIXME: ok? do I do something with this???
            &[size, relpath] => {
                let base = if let Some(ref dir) = target {
                    dir.clone()
                } else {
                    cwd.clone()
                };
                let relpath = base + "/" + relpath;
                fs.add(
                    &relpath,
                    size.parse()
                        .map_err(|e| anyhow!("failed to parse '{}': {}", size, e))?,
                )?;
            }
            &[] | &[""] => (),
            other => bail!("failed to parse line: {:?}", other),
        }
    }

    Ok(fs)
}
