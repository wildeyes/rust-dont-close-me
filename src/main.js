const { invoke } = window.__TAURI__.tauri;

async function main() {
  const rawData = await invoke("getZeData");
  console.log(rawData)
  // data comes in string, so we need to parse it
  const data = JSON.parse(rawData);

  opt1El.textContent = data.option_1
  opt2El.textContent = data.option_2
  h1Title.textContent = data.text
}

async function nextPrank() {
  await invoke("nextPrank");
  void main();
}

window.addEventListener("DOMContentLoaded", () => {
  window.opt1El = document.querySelector("#opt1-button");
  window.opt2El = document.querySelector("#opt2-button");
  window.h1Title = document.querySelector("#title");

  [...document
    .querySelectorAll("button")].forEach((el) => el.addEventListener("click", () => nextPrank()));

  main();
});
