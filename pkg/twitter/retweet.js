console.log("injecting");
body = document.getElementsByTagName('body')[0];
child = document.createElement("script");
child.innerHTML = `function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function run() {
    try {
        console.log("retweeting");
        retweet_button = document.getElementsByClassName("button submit selected")[0];
        retweet_button.click();
        console.log("retweeted");
        await sleep(10000);
        window.close();
    }
    catch (ex) {window.close();}
}

run()`;
body.appendChild(child);
console.log("injected");