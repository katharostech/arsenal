# FAQ

## Is being cross platform one of your game engine's main focus?

Yes. Arsenal should be able to deploy to every major target out there. This is going to be mostly handled by the [Amethyst](https://amethyst.rs) engine that we are building on top of so it should not be a large burden to the project.

Compared to Armory, Arsenal will probably have less of an issue targeting multiple platforms because we don't have to maintain the Krom ( development, JavaScript ) version of Armory games and the C/C++ version of Armory games, which was what caused so much of a difficulty with the Bullet physics integration. Arsenal will have **one** version that can be used for production and for development so we won't have to manage two different runtimes. WASM support for Amethyst is also being [sponsored by Mozilla](https://amethyst.rs/posts/moss-grant-announce) right now.

Also, because it is written in Rust, Arsenal shouldn't have any problem integrating with native libraries on different targets, something that gave Armory a bit of trouble because of Krom.

## What platform will your engine focus on?

To start we will focus on the desktop platforms: Linux, Windows, and Mac. After that we will want to go to consoles. This is in the roadmap for the Amethyst project so we may not need to do any work on this ourselves.

## What dimension will your game engine focus on?

3D will be the most important for us at first but it should not be hard to add extra controls that allow you to more easily create purely 2D elements after that. We definitely want you to be able to do both.

If we start with 3D, 2.5D is more of a special case of the physics engine, because it is 3D graphics, with 2D physics. The rust [nphysics](https://www.nphysics.org/) library supports both 2D and 3D physics so that might end up being easy to accomplish.

Supporting pure 2D should just be a matter of rendering and making the UX inside of Blender easy enough to place 2D objects so that might also be an easy feature to add when we get to it.

## What will your game engine feature?

That is less clear at the present time. We will start by focusing on the core elements needed to make any game and on giving the developer enough control to work anything that they might need into their game. More specific features like terrain and AI editors would be great to work in eventually, but our development effort will be guided by the features that we need to develop our own games.

We should have no problem accepting contributions for extra features as long as we deem it compliant to our standards.

## When do you intend to release the first version? (1.0)

We have no way of determining an accurate timeline at this point. It will definitely be more of a "work on it until we feel it is stable". But as it is critical to our own  game development, it will have the highest of priorities.

We are going to develop a "living" roadmap that we will update as we figure out what we need to work into the engine. Initially it will likely be rather inaccurate until we figure out more of what needs to be worked into Arsenal and into Amethyst.

## How do you support your game engine development?

Development of Arsenal is a part of the work of Katharos Technology as it will be the engine that we use to make our games. Work at Katharos Technology is funded by trusting God to provide for what we need to continue work. Donations are welcome.

We do not know where funding for the project will come from as the project continues; we are trusting that God will give us what we need to continue the project if that is His will. We will never charge for Arsenal, though. Arsenal will be free and Open Source from the start.

See our [about us](https://katharostech.com/about-us) page to understand more about what our goal is as an organization.

## What will be your game engine license?

The current plan is for the engine to be dual licensed under MIT and Apache 2.0, but the ownership of the copyright will be reserved by Katharos Technology. That means that contributors have to sign a [CLA](https://cla-assistant.io/katharostech/arsenal) and that we may re-license the engine under any other license that we choose if we need to for any reason. This is to protect us, as an organization, from putting a legal restriction on our use of our own software that we may regret later. We have the full intention of always providing the software under a free and Open Source license; part of what we want to accomplish with Katharos Technology is to give people the tools they need to make their own games.

## How Do You Decide What Features to Work On?

The development of the engine will be heavily directed by the need that we have for the engine at Katharos Technolgy. This means that we may not be able to facilitate developing features for everybody's use-cases, but it does mean that we have a vested interest in making the engine capable of producing high quality commercial games.

If other people want to contribute features that we may not have time to work on ourselves, there should be no reason that we cannot merge those features if they fit our vision.

## Is Arsenal A Blender Plugin or a Game Engine?

In reality, even though we call Arsenal an engine, the plan is to build Arsenal almost entirely on top of the [Amethyst](https://amethyst.rs) game engine while creating our own Blender based user experience. We have the documentation for the engine design [here](https://katharostech.github.io/arsenal/architecture.html).
