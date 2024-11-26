[...document.querySelectorAll("#subscribe-widget")].forEach(async el => {
    const r = await fetch("/mail-handler/is_subscribed");
    if(r.ok && (await r.text()) === "false") {
        const c = document.getElementById("subscribe-button").content.cloneNode(true);
        c.querySelector("button").addEventListener("click", async e => {
            const thisButton = e.target;
            const r = await fetch("/mail-handler/subscribe", { method: "POST" });
            if(r.ok) {
                thisButton.disabled = true;
                thisButton.innerText = "subscribed!";
                setTimeout(() => el.remove(), 1500);
            } else {
                const origText = thisButton.innerText;
                thisButton.innerText = "error subscribing :(";
                setTimeout(() => thisButton.innerText = origText, 2000);
            }
        });
        el.appendChild(c);
    }
})