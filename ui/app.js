const button = document.querySelector("#test-button");
const status = document.querySelector("#status");

button.addEventListener("click", () => {
  status.textContent = "JavaScript is working.";
});
