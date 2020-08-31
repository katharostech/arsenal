# Arsenal

![arsenal_banner]

<!--
  Comment out build status for now because the builds are failing and we don't use them for anything but docs at the momen:
  [![Build Status][build_status_img]][build_status_lnk]
-->

[arsenal_banner]: ./docs/logo/logo-3d-banner-with-bg.png
[build_status_img]: https://cloud.drone.io/api/badges/katharostech/arsenal/status.svg 
[build_status_lnk]: https://cloud.drone.io/katharostech/arsenal

## The Vision

Arsenal is a Blender game engine written in Rust and developed by [Katharos Technology][Katharostech.com]. With Arsenal we want to create an Open Source game engine that will bring the ability to make games of any scale, to users of any experience level. Arsenal will combine the powerful user interface of [Blender] with a high performance game engine core written in [Rust].

Friendly user-experience and beginner accessibility will be important on the list of features for Arsenal, but all of that ease of use must not come at the disadvantage of performance: Arsenal should be able to produce games of any size or complexity.

To program your games, Arsenal will incorporate scripting languages such as Python, in addition to being able to use logic nodes inside of Blender for a graphical programming experience. And if you want, you will always be able to write Rust to get the maximum performance.

Arsenal is our dream for the ultimate game engine, but there is a lot of work that needs to be done. By [sponsoring us][sponsor] on GitHub you help bring that dream to reality. You can learn more about the [vision of Arsenal][vision] or check out the [FAQ] in the [Arsenal Documentation][docs]. We also posting any updates on Arsenal development in the [Arsenal category][category] of our website.

[Blender]: https://blender.org
[Rust]: https://rust-lang.org
[sponsor]: https://github.com/sponsors/katharostech
[FAQ]: https://katharostech.github.io/arsenal/FAQ.html
[docs]: https://katharostech.github.io/arsenal/index.html
[vision]: https://katharostech.github.io/arsenal/vision.html
[category]: https://katharostech.com/tag/arsenal

## Documentation & Proof of Concept

The documentation can be found at [katharostech.github.io/arsenal][docs]. We have [downloads] of our first Arsenal proof-of-concept for 64bit Windows, Mac, and Linux, and a [Getting Started][started] guide that will walk you through testing it out. This is just the proof-of concept, though, and you cannot yet make real games with it.

[arsenal_feed]: https://katharostech.com/tag/arsenal
[Katharostech.com]: https://katharostech.com
[downloads]: https://github.com/katharostech/arsenal/releases
[started]: https://katharostech.github.io/arsenal/getting-started.html

### Design & Architecture

The current design plan is to build Arsenal as a Blender plugin that uses the [Bevy] game engine at its core. The [architecture] page in the docs shows the basic design idea, but we have recently decided to replace [Amethyst] with Bevy.

[Amethyst]: https://amethyst.rs
[Bevy]: https://bevyengine.org/
[architecture]: https://katharostech.github.io/arsenal/development-guide/architecture.html

## Bugs, Feature Requests, and Questions

We use [Taiga] for issue tracking. If you have any feature requests, bug reports, or questions you can submit an issue on our [Arsenal Taiga project][project].

[Taiga]: https://taiga.io/
[project]: https://tree.taiga.io/project/zicklag-arsenal/issues
[wiki]: https://tree.taiga.io/project/zicklag-arsenal/wiki/home

## GitHub Stars Over Time

![starcharts stargazers over time](https://starchart.cc/katharostech/arsenal.svg)
