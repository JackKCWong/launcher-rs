const { invoke } = window.__TAURI__.tauri;

let inputEl;

async function greet() {
  // send out input to the Rust side
  await invoke("greet", { name: inputEl.value });
  // reset the input box
  inputEl.value = "";
}

window.addEventListener("DOMContentLoaded", () => {
  inputEl = document.getElementById("launcher");
  // add a keyup listener & wait for "Enter" to be
  // pressed.
  inputEl.addEventListener(
    "keyup",
    (event) => {
      if (event.key === "Enter") {
        greet();
      }
    });
});