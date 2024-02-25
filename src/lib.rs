#![doc = include_str!("../README.md")]

use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::{read_to_string, remove_file, write};
use std::path::{Path, PathBuf};

/////////////////////////////////////////////// Error //////////////////////////////////////////////

/// A gatherbrained error.  Self-describing, if a little debuggy.
#[derive(Debug)]
pub enum Error {
    System {
        what: std::io::Error,
    },
    EnvVar {
        what: std::env::VarError,
    },
    EditorFailed,
    TempfileExists {
        path: PathBuf,
    },
}

impl From<std::io::Error> for Error {
    fn from(what: std::io::Error) -> Self {
        Self::System {
            what,
        }
    }
}

impl From<std::env::VarError> for Error {
    fn from(what: std::env::VarError) -> Self {
        Self::EnvVar {
            what,
        }
    }
}

/////////////////////////////////////////// Gatherbrained //////////////////////////////////////////

/// An active corpus of gatherbrained ideas.
pub struct Gatherbrained {
    entries: Vec<String>,
    path: PathBuf,
}

impl Gatherbrained {
    /// Create a new gatherbrained instance by reading and parsing the provided file.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let content = read_to_string(path.as_ref())?;
        let mut entries = vec![];
        let mut entry = String::new();
        for line in content.split('\n') {
            if !line.is_empty() && line.chars().all(|c| c == '-') {
                if !entry.trim().is_empty() {
                    let entry = entry.trim().to_string();
                    entries.push(entry);
                }
                entry = String::new();
            } else {
                entry += line;
                entry.push('\n');
            }
        }
        if !entry.trim().is_empty() {
            let entry = entry.trim().to_string();
            entries.push(entry);
        }
        let path = path.as_ref().to_path_buf();
        Ok(Self {
            entries,
            path,
        })
    }

    /// Add to a gatherbrained by invoking $EDITOR.
    pub fn add(&mut self) -> Result<(), Error> {
        let editor = std::env::var("EDITOR")?;
        let tmpfile = tmpfile_for(&self.path);
        if tmpfile.exists() {
            return Err(Error::TempfileExists {
                path: tmpfile,
            });
        }
        let status = std::process::Command::new(editor)
            .args([&tmpfile])
            .status()?;
        if Some(0) == status.code() {
            if !tmpfile.exists() {
                return Ok(());
            }
            let new = Gatherbrained::new(&tmpfile)?;
            if !new.entries.is_empty() {
                self.merge(new);
                self.save()?;
            }
            let _ = remove_file(tmpfile);
            Ok(())
        } else {
            let _ = remove_file(tmpfile);
            Err(Error::EditorFailed)
        }
    }

    /// Edit all of the gatherbrained entries that match all of the provided needles.
    pub fn edit(&mut self, needles: &[&str]) -> Result<(), Error> {
        let mut to_edit = vec![];
        let mut to_keep = vec![];
        for entry in self.entries.iter() {
            if needles.iter().all(|n| entry.to_lowercase().contains(&n.to_lowercase())) {
                to_edit.push(entry.as_str());
            } else {
                to_keep.push(entry.clone());
            }
        }
        let editor = std::env::var("EDITOR")?;
        let tmpfile = tmpfile_for(&self.path);
        if tmpfile.exists() {
            return Err(Error::TempfileExists {
                path: tmpfile,
            });
        }
        write(&tmpfile, Self::generate(&to_edit))?;
        let status = std::process::Command::new(editor)
            .args([&tmpfile])
            .status()?;
        if Some(0) == status.code() {
            let new = Gatherbrained::new(&tmpfile)?;
            *self = Gatherbrained {
                entries: to_keep,
                path: self.path.clone(),
            };
            self.merge(new);
            self.save()?;
            let _ = remove_file(tmpfile);
            Ok(())
        } else {
            let _ = remove_file(tmpfile);
            Err(Error::EditorFailed)
        }
    }

    /// Search this gatherbrained for entries that match all of the provided needles.  Returns a
    /// string that represents the selected entries in valid gatherbrained format.
    pub fn search(&self, needles: &[&str]) -> Result<String, Error> {
        let mut results = vec![];
        for entry in self.entries.iter() {
            if needles.iter().all(|n| entry.to_lowercase().contains(&n.to_lowercase())) {
                results.push(entry.as_str());
            }
        }
        Ok(Self::generate(&results))
    }

    /// Narrate the gatherbrained.  This will return a string in valid gatherbrained format that
    /// corresponds to searching for the needles provided by the narratives.  This is shorthand for
    /// writing a loop that searches for sets of needles, one after the other.
    pub fn narrate(&self, narratives: &[&str]) -> Result<String, Error> {
        let mut output = String::new();
        for narrative in narratives.iter() {
            let narrative = read_to_string(narrative)?;
            for line in narrative.split('\n') {
                let needles: Vec<&str> = line.split_whitespace().filter(|s| !s.is_empty()).collect();
                if needles.is_empty() {
                    continue;
                }
                let arc = self.search(&needles)?;
                let arc = arc.trim();
                if !arc.is_empty() {
                    if !output.is_empty() {
                        output += "----\n";
                    }
                    output += &arc;
                    output.push('\n');
                }
            }
        }
        Ok(output)
    }

    /// Construct a valid gatherbrained of entries not currently referred to by any of the provided
    /// narratives.
    pub fn missing(&self, narratives: &[&str]) -> Result<String, Error> {
        let mut entries: HashSet<String> = self.entries.iter().cloned().collect();
        for narrative in narratives.iter() {
            let narrative = read_to_string(narrative)?;
            for line in narrative.split('\n') {
                let needles: Vec<&str> = line.split_whitespace().filter(|s| !s.is_empty()).collect();
                if needles.is_empty() {
                    continue;
                }
                for entry in entries.clone().into_iter() {
                    if needles.iter().all(|n| entry.to_lowercase().contains(&n.to_lowercase())) {
                        entries.remove(&entry);
                    }
                }
            }
        }
        let entries: Vec<&str> = self.entries.iter().map(String::as_str).filter(|e| entries.contains(*e)).collect();
        Ok(Self::generate(&entries))
    }

    fn merge(&mut self, mut other: Gatherbrained) {
        self.entries.append(&mut other.entries);
    }

    fn save(&self) -> Result<(), Error> {
        let entries: Vec<&str> = self.entries.iter().map(|s| s.as_str()).collect();
        write(&self.path, Self::generate(&entries))?;
        Ok(())
    }

    fn generate(entries: &[&str]) -> String {
        let mut output = String::new();
        for (idx, entry) in entries.iter().enumerate() {
            let entry = entry.trim();
            if idx > 0 {
                output += "----\n";
            }
            output += entry;
            output.push('\n');
        }
        output
    }
}

//////////////////////////////////////////// path utils ////////////////////////////////////////////

/// Compute the history file for the given gatherbrained.
pub fn history_for<P: AsRef<Path>>(path: P) -> PathBuf {
    append_extension(path, ".history")
}

fn tmpfile_for<P: AsRef<Path>>(path: P) -> PathBuf {
    append_extension(path, ".tmp")
}

fn append_extension<P: AsRef<Path>>(path: P, ext: &str) -> PathBuf {
    let path = path.as_ref().to_path_buf();
    let mut path = path.as_os_str().to_os_string();
    path.push::<&OsStr>(ext.as_ref());
    PathBuf::from(path)
}
