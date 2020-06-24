console.log("injecting");
body = document.getElementsByTagName('body')[0];
child = document.createElement("script");
child.innerHTML = `
function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function run() {
    try {
        await sleep(7000);
        console.log("tweeting");

        var button = document.evaluate("(//span[text()='Tweet'])[1]", document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null).singleNodeValue;
        button.click();

        console.log("tweeted");
        await sleep(3000);
        window.close();
    }
    catch (ex) {
        console.log("failed tweet");
        await sleep(3000);
        window.close();
    }
}

run()`;
body.appendChild(child);
console.log("injected");