const defaultSelections = ["I", "II", "III"];
const rotorError = document.getElementById("rotor-error");
const rotorSelectors = document.querySelectorAll(".rotor");
let validRotors = true;

rotorSelectors.forEach((selector, index) => {
    selector.value = defaultSelections[index];
    selector.addEventListener("change", () => {
        const selectedRotors = Array.from(rotorSelectors).map(selector => selector.value);
        const selected = new Set();
        let duplicateRotor = "";
        validRotors = true;
        for (const rotor of selectedRotors) {
            if (selected.has(rotor)) {
                validRotors = false;
                duplicateRotor = rotor;
                break;
            }
            selected.add(rotor);
        }

        if (validRotors) {
            rotorError.classList.add("hide");
        } else {
            rotorError.innerText = `Duplicate rotor: Rotor ${duplicateRotor} is selected multiple times`
            rotorError.classList.remove("hide");
        }
    });
});

const plugboardInput = document.getElementById("plugboard");
const plugError = document.getElementById("plug-error");
let validFormat = true;
let validPairs = true;

plugboardInput.addEventListener("input", () => {
    const value = plugboardInput.value.trim().toLowerCase();
    validFormat = /^(\s*[a-z]{2}\s*)*$/i.test(value);

    const pairs = value.split(/\s+/);
    const usedLetters = new Set();
    let duplicateLetter = "";
    validPairs = true;
    for (const pair of pairs) {
        for (const letter of pair) {
            if (usedLetters.has(letter)) {
                duplicateLetter = letter;
                validPairs = false;
            } else {
                usedLetters.add(letter);
            }
        }
        if (validPairs) {
            break;
        }
    }

    if (validFormat && validPairs) {
        plugError.classList.add("hide");
    } else if (!validFormat) {
        plugError.innerText = `Invalid format: Please enter two-letter pairs separated by spaces "xy ab gh..."`;
        plugError.classList.remove("hide");
    } else if (!validPairs) {
        plugError.innerText = `Duplicate letter: ${duplicateLetter} is plugged multiple times`
        plugError.classList.remove("hide");
    }
});

const ciphertext = document.getElementById("ciphertext");

async function encode(formData) {
    try {
        const response = await fetch("/encode", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(formData),
        });
        if (!response.ok) {
            throw new Error(`HTTP error: status: ${response.status}`);
        }
        const data = await response.json();
        ciphertext.innerText = data;
    } catch (error) {
        console.error(`Error encoding text: ${error}`);
    }
}

const encodeForm = document.getElementById("encode-form");

function getFormData() {
    const formData = new FormData(encodeForm);
    const formDataObj = {};
    formData.forEach((value, key) => formDataObj[key] = value);

    const boxes = ["punc", "case", "space"]
    boxes.forEach((val) => formDataObj[val] = document.getElementById(val).checked);
    delete formDataObj.ciphertext;

    for (let i = 1; i < 4; i++) {
        formDataObj[`rotor${i}-pos`] = parseInt(formDataObj[`rotor${i}-pos`], 10);
    }
    return formDataObj;
}

encodeForm.addEventListener("submit", (event) => {
    event.preventDefault();
    if (!validFormat || !validPairs) {
        alert("Please resolve plugboard errors before encoding");
        return;
    } else if (!validRotors) {
        alert("Please resolve rotor errors before encoding");
        return;
    }
    encode(getFormData());
});