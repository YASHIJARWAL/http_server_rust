console.log("Rust server JS loaded");

document.addEventListener("DOMContentLoaded", function () {
    const title = document.querySelector("h1");

    title.addEventListener("click", function () {
        title.style.color = "red";
        title.innerText = "Rust Server Working!";
    });
})