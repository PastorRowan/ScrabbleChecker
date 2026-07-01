
import {
    useState,
    useEffect
} from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Box from '@mui/material/Box';
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";

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

    const [ enteredWord, setWord ] = useState<string>("");

    const [ fetchingDictionaries, setFetchingDictionaries ] = useState<boolean>(false);

    useEffect(() => {
        async function fetchDictionaries() {
            if (fetchingDictionaries) {
                return
            }
            setFetchingDictionaries(true)
            const response: GetDictionariesResponse = await invoke("get_dictionaries");
            console.log(response);
            if (response.ok) {
                setDictionaryNames(response.result);
            } else {
                console.error(response.error_msg);
            }
            setFetchingDictionaries(false)
        };
        fetchDictionaries();
    }, []);

    async function onCheckButtonTouch(e: React.TouchEvent<HTMLButtonElement>) {
        console.log("onCheckButtonTouch");
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        const result = await invoke(
            "is_word_in_dictionary",
            {
                dictionary_name: "dictionary.txt",
                word: enteredWord
            }
        );
        console.log(result);
    };

    return (
        <main className="container">
            <h1>Scrabble Checker</h1>
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
                    onTouchEnd={(e) => onCheckButtonTouch(e)}
                >
                    Check
                </Button>
            </Box>
        </main>
    );
}

export default App;
