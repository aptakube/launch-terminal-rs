const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

async function launchTerminal(name) {
  await invoke("launch_terminal", { name});
}

window.addEventListener("DOMContentLoaded", () => {
  const bt = document.getElementById("launch");
  const terminal = document.getElementById("terminal");
  bt.addEventListener("click", (e) => {
    launchTerminal(terminal.value);
  });
});
