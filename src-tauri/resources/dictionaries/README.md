
# Get scrabble dictionaries

> [!NOTE]
> Dictionary Directory and File Format

All Scrabble dictionary `.txt` files must be placed in the project's `src-tauri/resources/dictionaries` directory.

The directory should have the following structure:

```
ScrabbleChecker/
└── src-tauri/
    └── resources/
        └── dictionaries/
            ├── example_dictionary_1.txt
            ├── example_dictionary_2.txt
            ├── example_dictionary_2.txt
            ├── ...
            ├── example_dictionary_n.txt
            ├── ENABLE.txt
            └── README.md
```

Each dictionary `.txt` file should contain one word per line, followed by its definition or description:

```
word description for word 1
word description for word 2
word description for word 3
...
word description for word n
```

For example:
```
ABACK At the back; backwards.
ABAFT Towards the rear.
ABANDON To give up completely.
```

Multiple dictionary files can be placed in the `dictionaries` directory. ScrabbleChecker uses these files for word validation.

See [ENABLE.txt](ENABLE.txt) for an example of what the dictionary `.txt` file should look like

The script used to generate the `ENABLE.txt` is available in the following repository:

[WordGameDictionary](https://github.com/PastorRowan/WordGameDictionary)
