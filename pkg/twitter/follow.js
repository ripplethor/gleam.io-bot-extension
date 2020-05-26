console.log("injecting");
body = document.getElementsByTagName('body')[0];
child = document.createElement("script");
child.innerHTML = `function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function run() {
    try {
        console.log("following");
        follow_form = document.getElementById('follow_btn_form');
        follow_button = follow_form.getElementsByTagName("button")[0];
        follow_button.click();
        console.log("followed");
        await sleep(10000);
        window.close();
    }
    catch (ex) {window.close();}
}

run()`;
body.appendChild(child);
console.log("injected");