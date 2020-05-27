# Gleam.io bot

Automated clicks on gleam.io's giveaways.

## Building

Wasm-pack and cargo are needed to build the extension.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack
```

Now you can build the extension with:

```sh
wasm-pack build --target=web
```

## Usage requirements

Before use, you have to set up your web browser and social media. Otherwise, the bot will do strange things because it assumes that everything is setup. First, you need to be connected to a gleam.io account. To create an account, participate in a competition for the first time and enter your information. You can find a random competition using [Googleam](https://googleam.mubelotix.dev/). The bot will follow automatically on the websites Twitter, Twitch, and Mixer. All these accounts are optional (except the gleam.io's account), but you will get a lot more entries if you are connected, and that means that you will be more likely to win the giveaways. I suggest you create a new account on all these platforms, because the bot will follow everything asked by the giveaways, and that can taint your personal accounts.