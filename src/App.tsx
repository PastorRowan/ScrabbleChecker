
import "./App.css";

import React, {
    useState,
    useEffect
} from "react";

import { invoke } from "@tauri-apps/api/core";

import {
    Box,
    TextField,
    Button,
    Typography,
    Dialog,
    DialogTitle,
    DialogContent,
    DialogContentText,
    DialogActions,
    Radio,
    RadioGroup,
    FormControlLabel,
    FormControl,
    FormLabel
} from '@mui/material';

import {
    CheckCircle as CheckCircleIcon,
    Error as ErrorIcon
} from "@mui/icons-material";

type ResponseBase<T = undefined> = {
    ok: boolean,
    error_msg: string | undefined,
    result: T
}

type GetDictionariesResponse = ResponseBase<string[]>

type IsWordInDictionaryResponse = ResponseBase<boolean>

type CreateDictionaryResponse = ResponseBase

type DeleteDictionaryResponse = ResponseBase

function App() {

    const [ dictionaryNames, setDictionaryNames ] = useState<string[]>([]);

    const [ selectedDictionary, setSelectedDictionary ] = useState<string>("");

    const [ showSelectDictionaryDialogue, setShowSelectDictionaryDialogue ] = useState<boolean>(false);

    const [ enteredWord, setWord ] = useState<string>("");

    const [ showIsWordValid, setShowIsWordValid ] = useState<boolean>(false);

    const [ isWordValid, setIsWordValid ] = useState<boolean>(false);

    const [ checkedWord, setCheckedWord ] = useState<string>("");

    async function onSelectedDictionaryButtonClick(
        _: React.MouseEvent<HTMLButtonElement>
    ): Promise<void> {

        const response: GetDictionariesResponse = await invoke("get_dictionaries");

        console.log(response);

        if (!response.ok) {
            console.error(response.error_msg);
            return;
        };

        setDictionaryNames(response.result);

        setShowSelectDictionaryDialogue(true)

    };

    async function onCheckButtonClick(
        e: React.MouseEvent<HTMLButtonElement>,
        wordToCheck: string
    ): Promise<void> {

        console.log("onCheckButtonClick: ", wordToCheck);

        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

        const response: IsWordInDictionaryResponse = await invoke(
            "is_word_in_dictionary",
            {
                dictionaryName: "dictionary",
                word: wordToCheck
            }
        );

        console.log(response);

        if (!response.ok) {
            console.error(response.error_msg);
            return;
        };

        if (response.result) {
            setIsWordValid(true);
        } else {
            setIsWordValid(false);
        };

        setShowIsWordValid(true);
        setCheckedWord(wordToCheck);

    };

    function handleSelectDictionaryDialogueClose() {
        setShowSelectDictionaryDialogue(false)
    }

    function handleSelectDictionaryDialogueConfirm() {
        setShowSelectDictionaryDialogue(false)
    }

    return (
        <main className="container">

            <h1>Scrabble Checker</h1>

            <Box>
                <Button
                    onClick={(e) => onSelectedDictionaryButtonClick(e)}
                >
                    Select dictioanr 
                </Button>
            </Box>

            <Box
                sx={{
                    display: "flex",
                    flexDirection: "row",
                    justifyContent: "center"
                }}
            >
                <TextField
                    id="input"
                    label="Enter a word"
                    variant="outlined"
                    onChange={
                        (e) => {
                            setWord(e.currentTarget.value)
                        }
                    }
                />
                <Button
                    onClick={(e) => onCheckButtonClick(e, enteredWord)}
                >
                    Check
                </Button>
            </Box>

            {showIsWordValid
                ? (
                    <Typography>
                        {
                            isWordValid
                                ? (
                                    <>
                                        <CheckCircleIcon color="success"/> "{checkedWord}" is valid
                                    </>
                                )
                                : (
                                    <>
                                        <ErrorIcon color="error"/> "{checkedWord}" is invalid
                                    </>
                                )
                        }
                    </Typography>
                )
                : (
                    <></>
                )
            }

            <Dialog
                open={showSelectDictionaryDialogue}
                onClose={handleSelectDictionaryDialogueClose}
            >
                <DialogTitle>Select a Dictionary</DialogTitle>
                <DialogContent>
                    <DialogContentText>
                        Select a dictionary to verify words
                    </DialogContentText>
                    <FormControl>
                        <FormLabel>Selected dictionary</FormLabel>
                        <RadioGroup
                            defaultValue={selectedDictionary}
                            name="radio-buttons-group"
                        >
                            {(
                                dictionaryNames.map((dictionaryName: string) => {
                                    return (
                                        <FormControlLabel
                                            value={dictionaryName}
                                            control={<Radio />}
                                            label={dictionaryName}
                                        />
                                    )
                                })
                            )}

                        </RadioGroup>
                    </FormControl>
                </DialogContent>
                <DialogActions>
                    <Button onClick={handleSelectDictionaryDialogueClose}>
                        Cancel
                    </Button>
                    <Button
                        onClick={handleSelectDictionaryDialogueConfirm}
                        color="error"
                        variant="contained"
                    >
                        Delete
                    </Button>
                </DialogActions>
            </Dialog>

        </main>
    );
}

export default App;
