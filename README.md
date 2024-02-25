gatherbrained
=============

gatherbrained is a tool for helping organize written works.  It provides a command-line interface to maintain and
refactor stories.  The general idea is that one file, the "gatherbrained" file captures individual plot points to be
included in the story, while another file, the "narrative" organizes the gatherbrained thoughts into a coherent story.

The format of a gatherbrained file is a series of short text snippets separated by lines consisting solely of '-'
characters.  For eaxmple, the following is a valid gatherbrained file:

```ignore
This would be the first entry.  #first
----
This would be the second entry.  #second #gatherbrained
```

The corresponding narrative file could look like this:

```ignore
#first
#second #gatherbrained.
```

In this way one person can brain dump for days, months, or years, adding plot points and details to the story as they
go.  The narrative will automatically adapt to include new gatherbrained entries that match the search terms.  In this
way, entire stories can be written and reworked without losing the plot points, and without having to edit actual text.

The interactive shell is self-documenting with the in-built "help command":

```ignore
gatherbrained> help

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
```

It's possible to get help with the individual commands as well:

```ignore
gatherbrained> help add

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
```

Status
------

Maintained.  gatherbrained is in active use, but will likely see no further major updates.

Documentation
-------------

The latest documentation is always available at [docs.rs](https://docs.rs/gatherbrained/latest/gatherbrained/).
