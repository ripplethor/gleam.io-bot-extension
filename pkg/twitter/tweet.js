console.log("injecting");
body = document.getElementsByTagName('body')[0];
child = document.createElement("script");
child.innerHTML = `
function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function run() {
    try {
        console.log("tweeting");
        tweet_button = document.getElementsByClassName("button selected submit")[0];
        tweet_button.click();
        console.log("tweeted");
        await sleep(10000);
        window.close();
    }
    catch (ex) {window.close();}
}

run()`;
body.appendChild(child);
console.log("injected");