# Petfall: A json tool

## What?

Petfall is a simple tool that helps with json inspection. It is inspired by an older Ruby script of mine that does the same job.

It was created in a couple of hours, so don't expect much.

## How?

- `petfall [json_file] explain`: Traverses the json file and prints type information about the structure. It will become somewhat cleverer in the future.
- `petfall [json_file] format`: Every language has a "pretty print" JSON feature. Rust is no exception.
- `petfall [json_file] shell`: I considered creating a FUSE filesystem for JSON files but got bored, so this shell is the poor man's version.

## Shell
```
> ls
cities people animals
> cd cities
cities> cat
[
    {
        "name": "Athens",
        "country": "Greece"
    },
    {
        "name": "Lisbon",
        "country": "Portugal"
    },
    {
        "name": "Paris",
        "country": "France"
    }
]
cities> cd 0
cities/0> cd name
cities/0/name> cat
"Athens"
```
Supported commands, subject to change:
- `cat`: Prints the contents of the current JSON subtree.
- `cat-except [key1 key2 ...]`: Like cat, but accepts ignored keys as arguments (e.g. if there is a key called "useless_junk" that contains, well, useful stuff). This command will probably be deprecated soon.
- `cd [key]`: Change directory. Use `..` for the parent. Directory chaining (i.e. `../cities/0`) is not yet accepted.
- `discard`: If there has been a change, exit will nag you until you either hit discard or save.
- `edit [editor_name]` (e.g. `edit vim`): The reason to actually use this thing. Opens the JSON subtree in the specified editor. If the saved file is valid JSON, it applies the changes to the current subtree. Currently, there is no logic for non-blocking editor commands (i.e. `edit code` will probably fail spectacularly).
- `exit`: Exits this nonsense.
- `ls`: List the keys of the current subtree.
- `save [filename]`: Save the changed JSON to the specified filename.

## Why?

https://xkcd.com/1319/

It has already been used once to moderate success, so I call that a win.

## Why Rust?

Too grumpy to write in dynamically typed languages. Not enough free time to write in Haskell.

## Why is it called Petfall?

Because when I realized I had to manually edit JSON files, my first thought was "Oh no, not again".

## Future work

- My Ruby script could convert JSON to CSV utilizing some black magic, therefore I will port this nonsense at some point.
- Better shell usability.
- Expose commands outside shell (like [xsv](https://github.com/BurntSushi/xsv)).
- If this project becomes useful, I should write a more formal Readme.
