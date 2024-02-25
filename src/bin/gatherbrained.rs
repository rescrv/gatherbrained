use rustyline::config::EditMode;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{Config, Editor, Result};

use gatherbrained::{history_for, Gatherbrained};

fn help() {
    eprintln!("
gatherbrained is a tool for telling stories.

Invoke gatherbrained with the name of a gatherbrained file.

First, use the add, edit, and search functions to gather ideas in a file.  This
file should contain one concrete thought---approximately a paragraph worth---per
entry.  Once equipped with the components of a story, create the narrative in a
second file that corresponds to searches in the first file.  Each line in this
second file constitutes a story arc or a chapter; a set of related ideas.

The general idea is to enable progressive refinement of a story by adding
entries to the gatherbrained, and searching for them by key word in the
narrative.  The whole story can be reworked quickly and efficiently by shuffling
entries in the narrative file or by editing entries that match a search in the
gatherbrained file.  Together these two mechanisms allow refactoring of stories.

help .... display this help menu
add ..... add an entry to the gatherbrained file.
edit .... perform a search and edit the retrieved entries in $EDITOR
search .. search the story by hash tag
narrate . output the story according to a narrative file
missing . output the gatherbrained entries missing from the narrative
");
}

fn help_add() {
    eprintln!("
Add an entry to the gatherbrained.

gatherbrained will spawn an editor in which to construct a new entry.  Upon exit
of the editor, it will parse the entry and add it to the gatherbrained file.

A typical entry will look something like:

    Under heaven all can see beauty as beauty only because there is ugliness.
    All can know good as good only because there is evil.

    Therefore having and not having arise together;
    Difficult and easy complement each other;
    Long and short contrast each other;
    High and low rest upon each other;
    Voice and sound harmonize each other;
    Front and back follow each other.

    #taoteching #taoism

This entry will appear in searches and narratives for taoteching or taoism.
");
}

fn help_edit() {
    eprintln!("
Edit all entries matching all provided search terms.

gatherbrained will select for editing all entries which are tagged with every
tag provided as an argument to the edit command.  It will print all selected
entries to a temporary gatherbrained file and open it for editing in $EDITOR.
When the editor exits, gatherbrained will validate the file is well-formed and
merge the entries back into the gatherbrained, replacing the entries that were
initially selected with the new entries.
");
}

fn help_search() {
    eprintln!("
Search the gatherbrained for entries tagged with the search terms.

gatherbrained will select all entries which contain every search term provided
as an argument to the search command.  It will print the entries to stdout as a
valid gatherbrained.
");
}

fn help_narrate() {
    eprintln!("
Narrate a story using the current gatherbrained as source material.

gatherbrained will use the file(s) provided as an argument to gatherbrained to
narrate a story.  It will do so by taking each line in the narrative as a
search, and will output the results of searching to stdout as a valid
gatherbrained.
");
}

fn help_missing() {
    eprintln!("
Output the gatherbrained entries not present in any part of the narrative.

gatherbrained will use the file(s) provided as an argument to gatherbrained to
as in the narrative command, but instead of outputting the entries that match
the narrative, it will output the entries that do not match any part of the
narrative.
");
}

fn main() -> Result<()> {
    // Argument parsing.
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        help();
        std::process::exit(1);
    }

    // Create the line editor.
    let config = Config::builder()
        .max_history_size(1_000_000)?
        .history_ignore_dups(true)?
        .history_ignore_space(true)
        .edit_mode(EditMode::Vi)
        .auto_add_history(true)
        .build();
    let hist = FileHistory::with_config(config);
    let mut rl: Editor<(), FileHistory> = Editor::with_history(config, hist)?;
    let history = history_for(&args[1]);
    if history.exists() {
        rl.load_history(&history)?;
    }

    // Instantiate a gatherbrained.
    let mut gatherbrained = Gatherbrained::new(&args[1]).expect("gatherbrained should open");

    // Loop the shell
    loop {
        let line = rl.readline("gatherbrained> ");
        match line {
            Ok(line) => {
                let line = line.trim();
                let args: Vec<&str> = line.split_whitespace().filter(|s| !s.is_empty()).collect();
                if args.is_empty() {
                    continue;
                }
                match args[0] {
                    "add" => {
                        if let Err(err) = gatherbrained.add() {
                            eprintln!("error: {err:?}");
                        }
                    },
                    "edit" => {
                        if let Err(err) = gatherbrained.edit(&args[1..]) {
                            eprintln!("error: {err:?}");
                        }
                    },
                    "search" => {
                        match gatherbrained.search(&args[1..]) {
                            Ok(results) => {
                                println!("{}", results);
                            },
                            Err(err) => {
                                eprintln!("error: {err:?}");
                            }
                        }
                    },
                    "narrate" => {
                        match gatherbrained.narrate(&args[1..]) {
                            Ok(results) => {
                                println!("{}", results);
                            },
                            Err(err) => {
                                eprintln!("error: {err:?}");
                            }
                        }
                    },
                    "missing" => {
                        match gatherbrained.missing(&args[1..]) {
                            Ok(results) => {
                                println!("{}", results);
                            },
                            Err(err) => {
                                eprintln!("error: {err:?}");
                            }
                        }
                    },
                    "help" => {
                        if args.len() == 1 {
                            help();
                        } else {
                            for arg in &args[1..] {
                                match *arg {
                                    "add" => help_add(),
                                    "edit" => help_edit(),
                                    "search" => help_search(),
                                    "narrate" => help_narrate(),
                                    "missing" => help_missing(),
                                    _ => eprintln!("unknown command: {arg}"),
                                }
                            }
                        }
                    },
                    cmd => {
                        eprintln!("unknown command: {cmd}\nhere's the help menu instead\n");
                        help();
                    },
                }
            }
            Err(ReadlineError::Interrupted) => {
            }
            Err(ReadlineError::Eof) => {
                rl.save_history(&history)?;
                return Ok(());
            }
            Err(err) => {
                rl.save_history(&history)?;
                eprintln!("could not read line: {}", err);
            }
        }
    }
}
