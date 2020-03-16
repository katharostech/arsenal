# The Vision

In the shortest terms, we want to **make an engine that is approachable by as wide an audience as possible while supporting games of any size or complexity**. Here are some specific design goals:

- We need the engine to scale to a game of any size, even up to AAA games, with no technological bottleneck.
    - The engine core should be written in Rust
        - In order to make sure that engine performance is not a bottleneck, it needs to be written in a language that has multi-threading and that can integrate with native libraries.
        - Rust allows full control while eliminating entire classes of bugs and preventing vulnerabilities and hard to debug issues.
- We want the engine to be as easy to use and get started with as absolutely possible.
    - Armory successfully hit a lot of good points on ease of use and Arsenal will take a lot of pointers from Armory in that respect. We want to take all of the best things about Armory's user experience and make sure that Arsenal is not at a disadvantage to Armory in that respect.
    - We want to support visual scripting to allow non-programmers to make their own games.
    - The engine should provide a full Blender integration where you build your game scenes in Blender and export them instantly to a game. ( Textual scripting initially will only be supported in an any external editor such as VSCode, however visual scripting we hope will be done directly within Blender )
- We want the engine to be efficient.
    - The engine should, as much as possible, not use an absurd amount of system resources to run or develop games with.
        - We want the engine to be usable by as many people as possible and on as many different computers as possible without compromising on the quality of the engine and its capabilities.
    - The games in the engine should run on lower-end hardware as well as high-end hardware
        - Even though Arsenal should be able to support super large games that require a lot of system resources, it should not be limited in its ability to create small lightweight games that run on less modern and/or lower resource hardware.
- We want the engine to be mod-able.
    - Modding is a very big deal to us, partially because modding was one of the major things that inspired us to want to make games in the first place. Allowing people to make games that can dynamically load user content is high on our priorities.
- We want the engine to be cross-platform
    - The engine should be able to run on all of the major platforms including game consoles and the web:
        - Windows
        - Linux
        - Mac
        - Web
        - Android
        - iPhone
        - PlayStation 4
        - XBox One
        - Nintendo Switch
- We want the engine to be Open Source and built on Open Source tools so that anybody can contribute and/or modify the engine to their own use-case.

The engine should be able to compete with Unity, Unreal, Cryengine, etc. in the kinds of games you can create with it. It may or may not have all of the features of those game engines, though. The features that the engine comes with will be, at least initially, dependent on what we need to develop our own games at Katharos Technology. That doesn't mean that we won't support or integrate other features if somebody else wants to contribute them, and it doesn't mean that other features may not come later.

Even though the engine needs to be able to support large games, it should be easy to use and still be perfectly suitable for small indie games as well.

> **See Also:** [FAQ](./FAQ.md)
