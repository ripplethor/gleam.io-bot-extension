console.log("injecting");
body = document.getElementsByTagName('body')[0];
child = document.createElement("script");
child.innerHTML = `function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function run() {
    try {
        await sleep(7000);
        console.log("running");

        var button = document.evaluate("//div[@data-testid='confirmationSheetConfirm']//div[1]", document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null).singleNodeValue;
        button.click();

        console.log("done");
        await sleep(3000);
        window.close();
    }
    catch (ex) {
        console.log("failed run");
        await sleep(3000);
        window.close();
    }
}

run()`;
body.appendChild(child);
console.log("injected");