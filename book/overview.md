# Overview

Arsenal is the idea for a Blender integrated game engine written in Rust. Currently there is no game engine, it is just an idea, but we are currently doing the planning, investigation, experimentation, and documentation that is necessary to get the project started. The source code for the project is on [GitHub](https://github.com/katharostech/arsenal).

To start, we are developing an [architecture outline](./architecture.md) that lays out how we want to structure the components of the project.

## Motivation

We at [Katharos Technology](https://katharostech.com) have been planning to use [Armory3D](https://armory3d.org/) for our games until recent [concerns](http://forums.armory3d.org/t/armor3d-capability-questions/3118/8?u=zicklag) about Armory3D's Haxe runtimes were brought up and we decided that Armory runs the risk of not being able to handle large games due to the Haxe core. We believe that Rust is the best language for building a game engine and as such have been doing some investigation on how we would build a Rust game engine that offers the same advantages as Armory3D.

We have decided that it would not be the best route to simply rewrite Armory in Rust. Instead we think it would be better to design a new engine, potentially built on existing Rust tools like Amethyst, and provide a rich Blender integration and Logic Node system similar to what Armory has today.

## Feedback

If you have any thoughts or feedback you can open an [issue](https://github.com/katharostech/arsenal/issues/new) or contact us on our [website](https://katharostech.com/contact).
