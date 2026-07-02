
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

type DictionaryEntry = {
    word: string,
    description: string
}

type LookUpWordResponse = ResponseBase<DictionaryEntry | null>

// type CreateDictionaryResponse = ResponseBase

// type DeleteDictionaryResponse = ResponseBase

function App() {

    const [ dictionaryNames, setDictionaryNames ] = useState<string[]>([]);

    const [ selectedDictionaryIndex, setSelectedDictionaryIndex ] = useState<number | null>(null);

    const [ selectedDictionaryName, setSelectedDictionaryName  ] = useState<string>("No dictionary selected");

    const [ showSelectDictionaryDialogue, setShowSelectDictionaryDialogue ] = useState<boolean>(false);

    const [ enteredWord, setEnteredWord ] = useState<string>("");

    const [ showIsWordValid, setShowIsWordValid ] = useState<boolean>(false);

    const [ isCheckedWordValid, setIsCheckedWordValid ] = useState<boolean>(false);

    const [ checkedWord, setCheckedWord ] = useState<string>("");

    const [ checkedWordDescription, setCheckedWordDescription ] = useState<string>("");

    const [ dialogueSelectedDictionaryIndex, setDialogueSelectedDictionaryIndex ] = useState<number | null >(null);

    useEffect(() => {
        if (selectedDictionaryIndex === null) {
            return;
        };
        setSelectedDictionaryName(
            dictionaryNames[selectedDictionaryIndex]
        );
    }, [dictionaryNames, selectedDictionaryIndex]);

    useEffect(() => {

        async function fetchDictionaries() {

            const response: GetDictionariesResponse = await invoke("get_dictionaries");

            if (!response.ok) {
                console.error(response.error_msg);
                return;
            };

            setDictionaryNames(response.result);

            if (response.result.length >= 1) {
                setSelectedDictionaryIndex(0);
            };

        };

        fetchDictionaries();

    }, []);

    async function handleSelectedDictionaryButtonClick(
        _: React.MouseEvent<HTMLButtonElement>,
        currentSelectedDictionaryIndex: number | null
    ): Promise<void> {

        const response: GetDictionariesResponse = await invoke("get_dictionaries");

        console.log(response);

        if (!response.ok) {
            console.error(response.error_msg);
            return;
        };

        setDictionaryNames(response.result);

        setDialogueSelectedDictionaryIndex(currentSelectedDictionaryIndex);

        setShowSelectDictionaryDialogue(true);

    };

    function handleTextFieldChange(
        e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement, Element>
    ): void {
        const newEnteredWord = e.currentTarget.value.toUpperCase();
        const regexp =/^[A-Z]*$/;
        if (!regexp.test(newEnteredWord)) {
            return;
        };
        setEnteredWord(newEnteredWord);
    };

    async function handleCheckButtonClick(
        _: React.MouseEvent<HTMLButtonElement>,
        dictionaryName: string,
        wordToCheck: string
    ): Promise<void> {

        console.log("handleCheckButtonClick: ", wordToCheck);

        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

        const response: LookUpWordResponse = await invoke(
            "lookup_word",
            {
                dictionaryName: dictionaryName,
                word: wordToCheck
            }
        );

        console.log(response);

        if (!response.ok) {
            console.error(response.error_msg);
            return;
        };

        if (
            response.result &&
            response.result instanceof Object
        ) {
            setIsCheckedWordValid(true);
            setCheckedWordDescription(response.result.description);
        } else {
            setIsCheckedWordValid(false);
            setCheckedWordDescription("");
        };

        setCheckedWord(wordToCheck);
        setShowIsWordValid(true);

    };

    function handleSelectDictionaryDialogueClose(): void {
        setShowSelectDictionaryDialogue(false);
    };

    function handleSelectDictionaryDialogueConfirm(
        _: React.MouseEvent<HTMLButtonElement, MouseEvent>,
        newSelectedDictionaryIndex: number | null
    ): void {
        if (typeof newSelectedDictionaryIndex === "number") {
            setSelectedDictionaryIndex(newSelectedDictionaryIndex);
        };
        setShowSelectDictionaryDialogue(false);
    };

    function handleSelectDictionaryDialogueRadioButtonClick(
        _: React.MouseEvent<HTMLLabelElement, MouseEvent>,
        value: number
    ): void {
        setDialogueSelectedDictionaryIndex(value);
    };

    return (
        <main className="container">

            <h1>Scrabble Checker</h1>

            <Box>
                <Button
                    variant="contained"
                    onClick={(e) => handleSelectedDictionaryButtonClick(e, selectedDictionaryIndex)}
                >
                    {selectedDictionaryName} 
                </Button>
            </Box>

            <div
                style={{
                    width: "100%",
                    height: "15px"
                }}
            ></div>

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
                    value={enteredWord}
                    onChange={(e) => handleTextFieldChange(e)}
                />
                <Button
                    onClick={(e) => handleCheckButtonClick(e, selectedDictionaryName, enteredWord)}
                >
                    Check
                </Button>
            </Box>

            {showIsWordValid
                ? (
                    <Typography>
                        {
                            isCheckedWordValid
                                ? (
                                    <>
                                        <CheckCircleIcon color="success"/> "{checkedWord}" is valid
                                        <br />
                                        {checkedWordDescription}
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
                onClose={() => handleSelectDictionaryDialogueClose()}
            >
                <DialogTitle>Select a Dictionary</DialogTitle>
                <DialogContent>
                    <FormControl>
                        <FormLabel>{selectedDictionaryName}</FormLabel>
                        <RadioGroup
                            defaultValue={dialogueSelectedDictionaryIndex}
                            name="radio-buttons-group"
                        >
                            {(
                                dictionaryNames.map((dictionaryName: string, index: number) => {
                                    return (
                                        <FormControlLabel
                                            value={index}
                                            onClick={(e) => handleSelectDictionaryDialogueRadioButtonClick(e, index)}
                                            control={<Radio />}
                                            label={dictionaryName}
                                            key={dictionaryName}
                                        />
                                    )
                                })
                            )}

                        </RadioGroup>
                    </FormControl>
                </DialogContent>
                <DialogActions>
                    <Button onClick={() => handleSelectDictionaryDialogueClose()}>
                        Cancel
                    </Button>
                    <Button
                        onClick={(e) => handleSelectDictionaryDialogueConfirm(e, dialogueSelectedDictionaryIndex)}
                        color="error"
                        variant="contained"
                    >
                        Confirm
                    </Button>
                </DialogActions>
            </Dialog>

        </main>
    );
}

export default App;
