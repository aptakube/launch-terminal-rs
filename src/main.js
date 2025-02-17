const { invoke } = window.__TAURI__.core;

async function launchTerminal(terminal) {
  const errorEl = document.getElementById("error");
  try {
    errorEl.innerText = "";
    await invoke("launch", { terminal });
  } catch (err) {
    errorEl.innerText = err;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  const bt = document.getElementById("launch");
  const terminal = document.getElementById("terminal");
  bt.addEventListener("click", (e) => {
    launchTerminal(terminal.value);
  });
});
