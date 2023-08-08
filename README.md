# Snake Game!

**[press here to play](https://blockinhead.github.io/bevy_snake/)**  

The game's code is based on Chris Biscardi's [workshop](https://www.rustadventure.dev/snake-with-bevy-ecs), but updated to bevy 0.11. The UI was built with standard bevy UI API. Variable game speed was introduced to the gameplay.  
The game can be built to wasm for web and I could not find out how to accomplish it without [this](https://github.com/NiklasEi/bevy_game_template)  Niklas Eicker's project.   
Long story short
```shell
trunk serve
trunk build --release
```
