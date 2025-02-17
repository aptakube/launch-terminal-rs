const { invoke } = window.__TAURI__.core;

function getCommand() {
  var elements = document.getElementsByName("command");

  for (var i=0, len=elements.length; i<len; ++i) {
    if (elements[i].checked) {
      const value = elements[i].value;
      if (value == "custom") {
        return document.getElementById("custom-command").value;
      }
      if (value == "kubectl") {
        return "kubectl get pods";
      }
      return "echo 'Hello World'";
    }
  }
}

async function launchTerminal(terminal) {
  const errorEl = document.getElementById("error");
  try {
    errorEl.innerText = "";
    await invoke("launch", { terminal, command: getCommand() });
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
