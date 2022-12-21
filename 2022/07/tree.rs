use std::collections::{hash_map, HashMap};
use std::default::Default;
use std::fmt;
use std::path::PathBuf;

use anyhow::{anyhow, bail, Result};

// Folder is a specialized trie, separated by /. Files are pushed onto it by path.
// Nodes are folders and can have files.
#[derive(Debug)]
pub struct Filesystem {
    root: Node,
}

#[derive(Debug, PartialEq)]
pub enum Child {
    Folder(Node),
    File(usize),
}

use Child::*;

impl Default for Filesystem {
    fn default() -> Self {
        Filesystem {
            root: Node {
                path: PathBuf::from("/"),
                ..Default::default()
            },
        }
    }
}

impl Filesystem {
    /// push adds a value (size for this problem) onto the file system
    /// at the given path
    pub fn add(&mut self, path: &str, data: usize) -> Result<()> {
        self.root
            .add(self.canonical_path_segments(path)?.as_slice(), data)
    }

    /// get returns a file's size for a given path
    #[allow(dead_code)]
    pub fn get(&self, path: &str) -> Option<usize> {
        self.root
            .get(self.canonical_path_segments(path).unwrap().as_slice())
    }

    /// size returns the total file system size
    pub fn size(&self) -> usize {
        self.root.size()
    }

    pub fn iter<'a>(&'a self) -> FilesystemIter<'a> {
        FilesystemIter {
            rem: vec![self.root.iter()],
        }
    }

    // bunch of copies, but canonicalizes requests and removes relative segments
    fn canonical_path_segments(&self, path: &str) -> Result<Vec<String>> {
        let pb = PathBuf::from(path);
        Ok(pb
            .to_str()
            .ok_or_else(|| anyhow!("path is not valid utf8"))?
            .split("/")
            .map(str::to_owned)
            .collect())
    }
}

#[derive(Debug, PartialEq)]
pub struct Node {
    path: PathBuf, // the full path to this node
    children: HashMap<String, Child>,
}

impl Node {
    /// add adds the node to this node's children. path is assumed relative to
    /// this node.
    ///
    /// If the path contains one item, it's added to our children as a file.
    /// Otherwise, add continues. Repeat values overwrite.
    fn add(&mut self, path: &[String], data: usize) -> Result<()> {
        match path {
            [] => Ok(()),
            [file] => {
                self.children.insert(file.clone(), File(data));
                Ok(())
            }
            [folder, rem @ ..] => {
                if !self.children.contains_key(folder) {
                    self.children.insert(
                        folder.clone(),
                        Folder(Node {
                            path: self.path.join(folder),
                            ..Default::default()
                        }),
                    );
                }
                match self.children.get_mut(folder).unwrap() {
                    File(_) => bail!(
                        "file '{}' already exists but path '{:?}' implies it's a folder",
                        folder,
                        rem
                    ),
                    Folder(node) => node.add(rem, data),
                }
            }
        }
    }

    /// get returns a value specified by the path or None if it does not exist / is a folder.
    fn get(&self, path: &[String]) -> Option<usize> {
        match path {
            [] => None,
            [file] => self.children.get(file).and_then(|c| match c {
                File(size) => Some(size.clone()),
                _ => None,
            }),
            [folder, rem @ ..] => self.children.get(folder).and_then(|c| {
                match c {
                    File(_size) => None, // there's a remainder
                    Folder(f) => f.get(rem),
                }
            }),
        }
    }

    /// len returns the number of children at this node
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// returns recursive size of node's children
    pub fn size(&self) -> usize {
        self.iter().fold(0, |acc, n| match n {
            (_, File(size)) => size + acc,
            (_, Folder(f)) => f.size() + acc,
        })
    }

    /// iter iterates through this node's children
    pub fn iter(&self) -> NodeIter {
        NodeIter {
            path: self.path.clone(),
            iter: self.children.iter(),
        }
    }
    // nested_fmt returns this nodes files and child files based on
    // the provided path prefix
    fn nested_fmt(&self, builtpath: String) -> Vec<String> {
        self.children
            .iter()
            .flat_map(|(seg, c)| {
                let path = builtpath.clone() + "/" + seg;
                match c {
                    File(size) => vec![size.to_string() + &"\t".to_string() + &path],
                    Folder(next) => next.nested_fmt(path),
                }
            })
            .collect()
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            path: Default::default(),
            children: Default::default(),
        }
    }
}

impl fmt::Display for Filesystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", self.root.nested_fmt("".to_string()).join("\n"))
    }
}

#[test]
fn test_build_fs() {
    let mut fs = Filesystem::default();
    fs.add("/foo", 5).unwrap();
    assert!(fs.add("/foo/bar/baz", 10).is_err());
    fs.add("/foobar/baz", 3).unwrap();

    assert_eq!(Some(5), fs.get("/foo"));
    assert_eq!(Some(3), fs.get("/foobar/baz"));
    assert_eq!(None, fs.get("/bla"));
    assert_eq!(None, fs.get("/foo/bar"));

    println!("{}", fs);
}

pub struct NodeIter<'a> {
    // the full path of this node
    path: PathBuf,
    // the children of the node we are iterating
    iter: hash_map::Iter<'a, String, Child>,
}

impl<'a> Iterator for NodeIter<'a> {
    // FullPath => Child
    type Item = (PathBuf, &'a Child);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|(seg, c)| match c {
            File(_size) => Some((self.path.join(seg), c)),
            Folder(next) => Some((next.path.clone(), c)),
        })
    }
}

// FilesystemIter walks all children in a file system.
// Breadth First
pub struct FilesystemIter<'a> {
    // iterators to process, last elem first
    rem: Vec<NodeIter<'a>>,
}

impl<'a> Iterator for FilesystemIter<'a> {
    // FullPath => Child
    type Item = (PathBuf, &'a Child);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut iter) = self.rem.pop() {
            match iter.next() {
                None => (),
                Some((path, c @ File(_))) => {
                    self.rem.push(iter);
                    return Some((path, c));
                }
                Some((_, c @ Folder(next))) => {
                    self.rem.push(iter);
                    self.rem.push(next.iter());
                    return Some((next.path.clone(), c));
                }
            }
        }

        None
    }
}

#[test]
fn test_iter_fs() {
    let mut fs = Filesystem::default();
    let mut it = fs.iter();
    assert_eq!(None, it.next());

    fs.add("/foo", 5).unwrap();
    let mut it = fs.iter();
    assert_folder_eq("/", 1, it.next());
    assert_file_eq("/foo", 5, it.next());
    assert_eq!(None, it.next());

    fs.add("/foobar/baz/snap", 10).unwrap();
    fs.add("/foobar/baz/two", 15).unwrap();
    let mut it = fs.iter();
    // path => size | children #
    let mut seen: HashMap<String, usize> = HashMap::default();
    while let Some((p, n)) = it.next() {
        match n {
            File(size) => seen.insert(p.display().to_string(), size.clone()),
            Folder(n) => seen.insert(p.display().to_string(), n.len()),
        };
    }

    let mut expect: HashMap<String, usize> = vec![
        ("/", 2),
        ("/foo", 5),
        ("/foobar", 1),
        ("/foobar/baz", 2),
        ("/foobar/baz/snap", 10),
        ("/foobar/baz/two", 15),
    ]
    .into_iter()
    .map(|(p, c)| (p.to_owned(), c as usize))
    .collect();
    for (p, n) in seen {
        match expect.remove(&p) {
            None => panic!("'{}' was present in iteration but absent in expect", p),
            Some(c) => assert_eq!(n, c, "values are not equal for '{}'", p),
        };
    }
}

#[cfg(test)]
fn assert_file_eq(path: &str, size: usize, actual: Option<(PathBuf, &Child)>) {
    let c = File(size);
    assert_eq!(Some((PathBuf::from(path), &c)), actual);
}

// shallow assert a folder is equal, only counting children
#[cfg(test)]
fn assert_folder_eq(path: &str, children: usize, actual: Option<(PathBuf, &Child)>) {
    if let Some((actualpath, Folder(n))) = actual {
        assert_eq!(children, n.len(), "node: {:?}", n);
        assert_eq!(PathBuf::from(path), actualpath);
    } else {
        panic!("{:?} is not a folder", actual);
    }
}
