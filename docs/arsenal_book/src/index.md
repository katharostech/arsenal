# Overview

Arsenal is a Blender integrated game engine written in Rust. The project is in *very* early stages of development, but we are doing the planning, investigation, experimentation, and documentation that is necessary to get the project going. The source code for the project is on [GitHub][gh].

[gh]: https://github.com/katharostech/arsenal

## Getting Started

If you want to try out Arsenal, you can download the latest release [here][releases]. The [change log][changelog] will have the list of the updates made with each release. The [getting started](./getting-started.md) page will show you how to install Arsenal and run your first game.

[releases]: https://github.com/katharostech/arsenal/releases
[changelog]: https://github.com/katharostech/arsenal/blob/master/CHANGELOG.md

## Motivation

We at [Katharos Technology][kt] have been planning to use [Armory3D][arm] for our games until recent [concerns] about Armory3D's Haxe runtimes were brought up and we decided that Armory runs the risk of not being able to handle large games due to the Haxe core. We believe that Rust is the best language for building a game engine and as such have been doing some investigation on how we would build a Rust game engine that offers the same advantages as Armory3D.

Arsenal will be built on top of the [Amethyst][amethyst] game engine and will provide a full Blender integration that will allow you to build games in Blender. Arsenal will also come with a Logic Node system that will allow non-programmers to build out game logic.

More details of what we are trying to accomplish with Arsenal can be found in our [Vision](./vision.md).

[kt]: https://katharostech.com
[arm]: https://armory3d.org/
[concerns]: http://forums.armory3d.org/t/armor3d-capability-questions/3118/8?u=zicklag
[amethyst]: https://amethyst.rs

## Roadmap

You can view our [roadmap] by looking at the Sprints on Taiga. Each sprint represents an upcoming release and the stories that we plan on working on for that release. You can also see what work is in progress on the [Kanban board][kanban]. See the Taiga [wiki] for more information on our issue workflow.

[roadmap]: https://tree.taiga.io/project/zicklag-arsenal/backlog
[workboard]: https://github.com/katharostech/arsenal/projects/1
[kanban]: https://tree.taiga.io/project/zicklag-arsenal/kanban
[wiki]: https://tree.taiga.io/project/zicklag-arsenal/wiki

## News

We will be posting updates on the development progress of Arsenal on the [Arsenal feed][arsenal_feed] at [Katharostech.com]. Releases of Arsenal will be tracked in the [changelog]. Notable milestones or events will be put on the [News] page.

[arsenal_feed]: https://katharostech.com/tag/arsenal
[Katharostech.com]: https://katharostech.com
[changelog]: https://github.com/katharostech/arsenal/blob/master/CHANGELOG.md
[News]: ./news.md

## Feedback

If you have any thoughts or feedback you can open an [issue] or contact us on our [website][contact].

[issue]: https://tree.taiga.io/project/zicklag-arsenal/issues
[contact]: https://katharostech.com/contact
