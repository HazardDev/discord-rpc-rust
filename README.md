# discord-rpc-rust
Discord Rich Presence Library written in Rust

# CURRENTLY WINDOWS ONLY

I'll switch to building discord-rpc from source for each platform once I get it working on my primary platform.


## Building DiscordRPC For Linking

Note: Use the same instruction set as your rust compiler. I'm using 64 bit, hence the 64 bit compiler

```
git clone https://github.com/discordapp/discord-rpc
cd discord-rpc
mkdir build
cd build

for windows:
cmake .. -DCMAKE_INSTALL_PREFIX=build -G 'Visual Studio 15 2017 Win64'
cmake --build . --config Release --target install
```