
import {
    useState,
    useEffect
} from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";

function App() {

    const [ enteredWord, setWord ] = useState("");

    async function onCheckButtonTouch(e: React.TouchEvent<HTMLButtonElement>) {
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        const result = await invoke(
            "is_word_in_dictionary",
            {
                dictionary_name: "",
                word: enteredWord
            }
        );
    }

    return (
        <main className="container">
            <div className="row">
                <h1>Scrabble Checker</h1>
                <TextField
                    id="input"
                    label="Enter a word"
                    placeholder="Enter a word"
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
            </div>

        </main>
    );
}

export default App;
