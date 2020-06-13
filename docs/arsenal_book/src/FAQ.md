# FAQ

## Is being cross platform one of your game engine's main focus?

Yes. Arsenal should be able to deploy to every major target out there including:

- Windows
- Linux
- MacOS
- Android
- iOS
- XBox
- PlayStation
- Nintendo Switch

Support for all of these platforms will most likely not exist initially, but it is our ultimate goal that these platforms will be supported in the future.

## What platform will your engine focus on?

To start we will focus on the desktop platforms: Linux, Windows, and Mac. After that we will want to go to web, mobile, and consoles.

## What dimension will your game engine focus on?

Arsenal should be fully suitable for both 3D and 2D games. The goal will be to provide fully featured 3D support while additionally extending Blender's UX with extra features designed to make it easy to create 2D games

## What will your game engine feature?

That is less clear at the present time. We will start by focusing on the core elements needed to make any game and on giving the developer enough control to work anything that they might need into their game. More specific features like terrain and AI editors would be great to work in eventually, but our development effort will initially be focused on creating a solid core that future features can build on.

## When do you intend to release the first version? (1.0)

We have no way of determining an accurate timeline at this point. It will definitely be more of a "work on it until we feel it is stable".

We are going to develop a "living" roadmap that we will update as we figure out what we need to work into the engine. Initially it will likely be rather inaccurate until we figure out more of what needs to be worked into Arsenal.

## How do you support your game engine development?

Work on Arsenal will be funded through [GitHub sponsors](https://github.com/sponsors/katharostech). If we receive enough funding through GitHub sponsors we will be able to pay at least one developer to work full time on Arsenal and other related game development tooling. Every little bit counts and with donation tiers starting at $1/month GitHub sponsors makes it easy to say thanks and contribute to our work!

## What will be your game engine license?

The current plan is for the engine to be licensed under MIT, but the ownership of the copyright will be reserved by Katharos Technology. That means that contributors have to sign a [CLA](https://cla-assistant.io/katharostech/arsenal) and that we may re-license the engine under any other license that we choose if we need to for any reason. This is to protect us, as an organization, from putting a legal restriction on our use of our own software that we may regret later. We have the full intention of always providing the software under a free and Open Source license; part of what we want to accomplish with Katharos Technology is to give people the tools they need to make their own games.

## How Do You Decide What Features to Work On?

Our goal right now is to establish, one layer at a time, the foundation necessary to build quality games. Features worked on next will be those that are foundational and required for every working game. We want to get the engine to a point where it is stable enough to actually produce a game, before we start adding all of the bells and whistles.

After that, development will be focused on a combination of the community's needs and our own needs as we start developing games with the engine.

## Why Does Arsenal Export glTF Instead of Just Using Blender Files?

There are three reasons:

1. **Maintenance:** Keeping up with the Blender file format could be time-consuming as it is a moving target. Comparatively, the glTF format is very simple and standardized.
2. **Storage:** The blender format contains a lot of metadata that we don't need for a game, such as the Blender UI layout.
3. **Efficiency:** If you use the Blender file format, the game would have to decode the .blend file while the game is running. This would involve going through more information than is actually needed in the game and would require more CPU to get the required data. glTF is a very dense format that compactly stores the information we need while also facilitating efficient extraction into a running game.
