
# Get scrabble dictionaries

> [!NOTE]
> Dictionary Directory and File Format

All Scrabble dictionary `.txt` files must be placed in the project's `src-tauri/resources/dictionaries` directory.

The directory should have the following structure:

ScrabbleChecker/
└── src-tauri/
    └── resources/
        └── dictionaries/
            ├── example_dictionary_1.txt
            ├── example_dictionary_2.txt
            ├── example_dictionary_2.txt
            ├── ...
            ├── example_dictionary_n.txt
            └── README.md

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

Multiple dictionary files can be placed in the dictionaries directory.

ScrabbleChecker uses these files for word validation.

## Option 1: Download from GitHub

### 1. Open the Scrabble Words repository

Open the following page in your web browser:
https://github.com/scrabblewords/scrabblewords/tree/main/words

You should see this:
![scrabble words github repository](screenshots/scrabble_words_github_repository.png)

### 2. Open the British folder

Select the `British` folder.
![scrabble words github repository](screenshots/scrabble_words_github_repository_british_folder.png)

### 3. Select `CSW21.txt`

Select the **CSW21.txt** dictionary file.
![scrabble words github repository select CSW21.txt](screenshots/scrabble_words_github_repository_select_csw21txt.png)

### 4. Download the file

Download the `CSW21.txt` file.
![scrabble words github repository download CSW21.txt](screenshots/scrabble_words_github_repository_download_csw21txt.png)

### 5. Copy the file into the dictionaries directory

Copy the downloaded file into the project's src-tauri/resources/dictionaries directory.

The directory structure should look like this:
```
ScrabbleChecker/
└── dictionaries/
    └── resources/
        └── dictionaries
            └── CSW21.txt
```

## Option 2: Use NASPA Zyzzyva

### 1. Install Zyzzyva

Download and install [NASPA Zyzzyva](https://scrabbleplayers.org/w/NASPA_Zyzzyva_Download).

### Locate the Dictionaries
After installing Zyzzyva, locate its application data directory and find the dictionary `.txt` files.

### 3. Copy the Dictionaries

Copy the required .txt dictionary files from the Zyzzyva application data directory into the dictionaries directory.

The dictionaries directory should look something like:
```
ScrabbleChecker/
└── dictionaries/
    └── resources/
        └── dictionaries
            ├── CSW24.txt
            ├── NWL2023.txt
            └── ...
```

These dictionary files are used by ScrabbleChecker for word validation.
