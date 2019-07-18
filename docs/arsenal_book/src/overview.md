# Overview

Arsenal is a Blender integrated game engine written in Rust. The project is in *very* early stages of development, but we are doing the planning, investigation, experimentation, and documentation that is necessary to get the project going. The source code for the project is on [GitHub](https://github.com/katharostech/arsenal).

## Getting Started

If you want to try out Arsenal, you can download the latest release [here](https://github.com/katharostech/arsenal/releases). The [change log](https://github.com/katharostech/arsenal/blob/master/CHANGEFILE.md) will have the list of the updates made with each release. The [getting started](./getting-started.md) page will show you how to install Arsenal and run your first game.

## Motivation

We at [Katharos Technology](https://katharostech.com) have been planning to use [Armory3D](https://armory3d.org/) for our games until recent [concerns](http://forums.armory3d.org/t/armor3d-capability-questions/3118/8?u=zicklag) about Armory3D's Haxe runtimes were brought up and we decided that Armory runs the risk of not being able to handle large games due to the Haxe core. We believe that Rust is the best language for building a game engine and as such have been doing some investigation on how we would build a Rust game engine that offers the same advantages as Armory3D.

Arsenal will be built on top of the [Amethyst](https://amethyst.rs) game engine and will provide a full Blender integration that will allow you to build games in Blender. Arsenal will also come with a Logic Node system that will allow non-programmers to build out game logic.

More details of what we are trying to accomplish with Arsenal can be found in our [Vision](./vision.md).

## Roadmap

You can view our [roadmap](https://github.com/katharostech/arsenal/milestones) by looking at the milestones on GitHub. The roadmap will likely change as we find out what work needs to be done more specifically.

## Feedback

If you have any thoughts or feedback you can open an [issue](https://github.com/katharostech/arsenal/issues/new) or contact us on our [website](https://katharostech.com/contact).
