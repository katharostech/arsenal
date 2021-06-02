# Arsenal

![arsenal_banner]

<!--
  Comment out build status for now because the builds are failing and we don't use them for anything but docs at the momen:
  [![Build Status][build_status_img]][build_status_lnk]
-->

[arsenal_banner]: ./docs/logo/logo-3d-banner-with-bg.png
[build_status_img]: https://cloud.drone.io/api/badges/katharostech/arsenal/status.svg
[build_status_lnk]: https://cloud.drone.io/katharostech/arsenal

## Project Status

Arsenal development is currently on hold while we at Katharos Technology focus on producing our first game prototype, [Bounty Bros.][bounty_bros], which is a 2D, retro-style game built on the [Bevy] game engine and our own [Bevy Retro][bevy_retro] plugin.

Because Arsenal will also be built on top of the Bevy game engine, much of what we are learning while creating Bevy Retro and Bounty Bros. will go directly to helping us prepare for making Arsenal. Arsenal is absolutely still something we plan on building and using in the future, but are a small team and have shifted our focus for the time being in order to narrow our scope and help ensure that we will be able to actually get started making commercial games.

[bounty_bros]: https://katharostech.com/post/bounty-bros-on-web
[bevy_retro]: https://github.com/katharostech/bevy_retro
[arsenal_channel]: https://katharostech.com/tag/arsenal

## The Vision

Arsenal is a Blender game engine written in Rust and developed by [Katharos Technology][katharostech.com]. With Arsenal we want to create an Open Source game engine that will bring the ability to make games of any scale, to users of any experience level. Arsenal will combine the powerful user interface of [Blender] with a high performance game engine core written in [Rust].

Friendly user-experience and beginner accessibility will be important on the list of features for Arsenal, but all of that ease of use must not come at the disadvantage of performance: Arsenal should be able to produce games of any size or complexity.

To program your games, Arsenal will incorporate scripting languages such as Python, in addition to being able to use logic nodes inside of Blender for a graphical programming experience. And if you want, you will always be able to write Rust to get the maximum performance.

Arsenal is our dream for the ultimate game engine, but there is a lot of work that needs to be done. By sponsoring us on [Open Collective][sponsor] you help bring that dream to reality. You can learn more about the [vision of Arsenal][vision] or check out the [FAQ] in the [Arsenal Documentation][docs]. We also posting any updates on Arsenal development in the [Arsenal category][category] of our website.

[blender]: https://blender.org
[rust]: https://rust-lang.org
[sponsor]: https://opencollective.com/arsenal
[faq]: https://katharostech.github.io/arsenal/FAQ.html
[docs]: https://katharostech.github.io/arsenal/index.html
[vision]: https://katharostech.github.io/arsenal/vision.html
[category]: https://katharostech.com/tag/arsenal

## Documentation & Proof of Concept

The documentation can be found at [katharostech.github.io/arsenal][docs]. We have [downloads] of our first Arsenal proof-of-concept for 64bit Windows, Mac, and Linux, and a [Getting Started][started] guide that will walk you through testing it out. This is only a proof of concept and cannot actually produce games yet. In the next release of Arsenal you will be able to make simple games with scripting.

[arsenal_feed]: https://katharostech.com/tag/arsenal
[katharostech.com]: https://katharostech.com
[downloads]: https://github.com/katharostech/arsenal/releases
[started]: https://katharostech.github.io/arsenal/getting-started.html

### Design & Architecture

The current design plan is to build Arsenal as a Blender plugin that uses the [Bevy] game engine at its core. The [architecture] documentation explains the components in more detail.

[bevy]: https://bevyengine.org/
[architecture]: https://katharostech.github.io/arsenal/development-guide/architecture.html

## Bugs, Feature Requests, and Questions

If you have any questions or ideas, don't hesitate to ask! Open a GitHub issue and let us know what you're thinking.

## License

Arsenal is licensed under the [Katharos License][k_license] which places certain restrictions on what you are allowed to use Arsenal for. Please read and understand the terms before using Arsenal for your project.

[k_license]: https://github.com/katharostech/katharos-license
