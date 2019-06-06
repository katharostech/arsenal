# Architecture Draft 1

The first Arsenal architecture draft is shown below:

![Architecture Diagram](./assets/arsenalArchitecture.svg)

## Core Components

### Kinc

We are considering building on [Kinc](https://github.com/Kode/Kinc), which is the C/C++ library that Armory is built on. Kinc handles the platform and graphics abstraction that allows Armory to target different graphics API's like Vulcan, DirectX, and Metal, and allows Armory to target game consoles as well. The maintainer of Kinc is also working on creating Rust bindings which will make it easier to integrate with. 

We like Kinc because we know who the maintainer is and we could potentially port Armory's renderer by building on top of Kinc, but going with a full Rust solution is probably more pragmatic, and we are going to consider both options.

### Amethyst

[Amethyst](https://github.com/amethyst/amethyst) is along the lines of what we want the core of the engine to be. The ECS and highly parallel design will allow the engine to scale well from small games to very large games. One of the reasons we don't want to simply rewrite Armory in Rust is because we see building the engine itself on an ECS as a large advantage in engine design.

### Renderer

Something we may want to port from Armory is its renderer. Armory has a very good renderer and if we built on Kinc it would mean that we wouldn't have to change the API that it uses to communicate to the graphics hardware. It would be very nice if we could leverage the already awesome Armory renderer, but it may still may be better to go with Amethysts pure Rust [renderer](https://github.com/amethyst/rendy) and instead take just the shaders from Armory or just make our own.

### Arsenal Engine

Additionally we would need our own components and systems that we add on top of Amethyst to add physics simulation and Blender elements like objects, lights, speakers, etc.

All of these elements, Kinc, Amethyst, the renderer, and the Arsenal specific systems would be built together to make up the Arsenal shared library which will be utilized by the Blender plugin and the Arsenal executable.

## Blender Integration

The Arsenal Blender integration will be provided by two components: the Python Blender Plugin, and the Rust Python Extension. The Python Blender Plugin, is used to provide all of the extra Blender UI elements and integrations through the Blender Python API. The Rust Python Extension is a native Python module that is loaded by the Blender plugin and used to do parallel asset conversion, to build the Arsenal game, and to run the Arsenal game from inside blender ( in a separate window, but in the same process ).

The Rust Python Extension will be dynamically linked to the Arsenal library, which will give it full access to the Arsenal engine API while simultaneously acting as a Python module that allows the Python Blender Plugin the ability to do advanced integrations with the engine such as live-sync, hot reload, etc. Tools such as the [Krafix](https://github.com/Kode/krafix) GLSL cross-compiler could also be embedded into the Rust Python Extension to make it more self-contained and hopefully eliminate most of the need to install other tools on the user's system.

### Arsenal Games

The Arsenal game itself will be a Rust crate that is created alongside the .blend file by the Blender plugin, very similar to the way that Armory creates a Kha project adjacent to Armory blends. The game crate is built as a shared library that acts like an Arsenal "plugin" and is loaded by the Arsenal library as directed by the Blender plugin. The game crate also gets dynamically linked to the Arsenal library to give it full access to the game engine API.

To run the Arsenal game standalone, you use the Arsenal executable, which links to the Arsenal library and directs it to load the game crate and run the game.

### Arsenal Mods

In addition to the game crate, there may be any number of "mod" crates that are nearly identical to game crates, except that they are loaded after the game crate. Mod crates are created in different blends than the game crate and can be loaded dynamically, if the game allows, to let other players add their own functionality and extensions to the game.

## Experimentation Done So Far

All of the above architecture is theoretical. We have been working on proving out different elements of the architecture to make sure that it is feasible. So far we have successfully made a Rust Python extension that can be imported into a Blender plugin with a single operator that executes a Rust function.

The largest problem we are having is getting Rust crates to dynamically link to other Rust crates. It should be possible, but it may require that you use the exact same version of Rust for all libraries involved. This could cause problems with things like the fact that Amethyst only supports Rust stable and the PyO3 library we are using for the Python extension requires Rust nightly. We need to do some research to see if there is a way to provide a stable ABI over which to link Rust crates to each-other conveniently.

## Draft Status

This design draft has been decided against due to the fact that there is no way in Rust to do dynamic linking between crates built by different versions of Rust. A Rust [forum topic](https://internals.rust-lang.org/t/dynamically-linking-rust-crates-to-rust-crates/10369?u=zicklag) was brought up to discuss this. This design relied heavily on being able to dynamically Rust crates and is therefore unfeasible.
