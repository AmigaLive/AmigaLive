import { invoke } from "@tauri-apps/api/tauri";

const output = document.getElementById("output");

async function fetchGameList() {
  const data = await invoke("fetch_game_list");
  output.textContent = JSON.stringify(data, null, 2);
}

async function downloadGame() {
  const result = await invoke("download_game", { id: 1 });
  output.textContent = result;
}

async function uploadGame() {
  const result = await invoke("upload_game", { filePath: "/path/to/game.adf" });
  output.textContent = result;
}

async function launchFsuae() {
  const result = await invoke("launch_fsuae", { configPath: "amiga.fs-uae" });
  output.textContent = result;
}

document.getElementById("btn-list").onclick = fetchGameList;
document.getElementById("btn-download").onclick = downloadGame;
document.getElementById("btn-upload").onclick = uploadGame;
document.getElementById("btn-launch").onclick = launchFsuae;
