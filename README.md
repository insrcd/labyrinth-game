# Labyrinth Brewery - THE GAME

A few of my buds at my local brewery are making a beer of my own specification, so I decided to make them a video game. I am not a video game developer. I'm a guy who once wrote a MUD in C to learn how it in the 90s. So, One of the main reasons I'm doing this is to learn https://www.rust-lang.org/.

This project is possible because of https://bevyengine.org/, a ECS and rendering engine that's brand new, and super sweet. I'm hoping tht me struggling through how to implement basic stuff (like Zooming, tile placement, mouse movement, sprite interaciton, ECS queries, etc) will help others as Bevy becomes more mature.

The game will be a market similuation / roguelike hybrid. You, the Hero, will decend into the recently discovered caverns below the Labyrinth Brewery in Manchester, Connecticut. In the labyinth, you can gather materials for your next craft beer or cider recipe, new items to destroy epic monsters, and other generic roguelike stuff. If you parish during the depths you will find yourself on the Bar Room floor, with all of your recipes and improvements but all of your items will be gone! You may have to deal with a rowdy bar guest from time to time as well.

The owners of Labyrinth Brewery are highly involved in local community and great causes (like Interval House (https://intervalhousect.org/), Black Lives Matter, the CT Bail Project to name a few) and I am trying to involve our local artist and writing community to make this project successful. If this project is ever monitized, any revenue will go to the same causes that Labyrinth cares about, as well as locally commissioned artists and writers

Currently, this codebase is merely being used to design the basic systems and datatypes needed for the game. It is going to leverage the rust scripting engine RHAI for NPC / Powers / AI development.

So, join us in solving the mystery of the Labynrith Below Labyrinth!

## Contribution

If you want to contribute, please create issues or pull requests. Since I am doing this to learn, I'm doing a lot of things wrong. I've read a lot of documentation on rust and a lot of source code to try and make what I create as idiomatic as possible, but I'm going to do things differently or wrong - a lot! My background is mostly Java / C# and Kotlin so my mindset is heavily inspired by functional and OOP. All I ask for is kindness and understanding. Any suggestions are welcome!

## Quickstart for Development

- clone the repo
- install https://rustup.rs/
- cargo run --release in the repo directory

Depending on your development environment there may be some "gotchyas" based on your video card, operating system, etc, check out https://github.com/bevyengine/bevy for current issues with your environment.

## Goals 

- A roguelike / Market Simulation game. Explore, battle stuff, get stuff, make stuff, sell stuff.
- A custom sprite for every Regular and Employee of Labyrinth
- NPC Mobs based on Labyrinth Brewery myth - Demanding Customers, Celler Trolls, Beer Wizards, and Cantankerous Owners
- A highly customizable scripting / dialog engine so a good story can be told
- Involve local artists to design quests, concept art and sprites.

## What's Done
- Start of a Map / World Editor
- Tile Blueprints
- Sprite Library / Tile Palette for easy access of assets
- Zoom
- Mouse Click Locations
- NPC Movement
- Tile Interactions
- Text Output (this is a workaround for the TextComponent crash, and will be refactored)
- Interaction with Placeables
- Cargo refactoring
- Some of the World Entity definition (Objects, Items, etc)

## TODO

- Dialog System
- NPC Interaction System
- Object Interactoin System
- Combat
- Market Simulation
- Procedural Generation of Labyrinth
- Bar Room Sprites / Map
- Custom Sprites for NPCs / Regulars
- Testing, Testing, Testing (Becasuse Bevy and this is new I'm focusing on code, but I am a TDD wonk so I will be putting out tests once the environment stabilizes)

## Thanks
- My wonderful, supportive wife, Hadria Beth (https://hadriabeth.com) who will be responsible for quite a bit of the sprite art in the game.
- @cart for his hard work on Bevy, thank you for making a engine that even a novice Rust dev can adopt.
- All helpful folks in the Bevy discord channel, they have started a great community.

## Sprite Sheet Thanks

I am using openly available spritesheets for now (and one that is not so open but I'm working on replacing that), and here is the list of thanks to these amazing artists who make their work available (If I missed anyone please contact me and I will update it ASAP).

- http://blog-buch.rhcloud.com
- Stephen Challener (Redshrike) and Jetrel, hosted by OpenGameArt.org
- https://opengameart.org/content/lots-of-free-2d-tiles-and-sprites-by-hyptosis - Hyptosis and Zabin
- Zabin, Daneeklu, Jetrel, Hyptosis, Redshrike, Bertram. http://opengameart.org/content/rpg-tiles-cobble-stone-paths-town-objects 
