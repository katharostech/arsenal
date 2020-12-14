# Arsenal

![arsenal_banner]

<!--
  Comment out build status for now because the builds are failing and we don't use them for anything but docs at the momen:
  [![Build Status][build_status_img]][build_status_lnk]
-->

[arsenal_banner]: ./assets/logo-3d-banner-with-bg.png
[build_status_img]: https://cloud.drone.io/api/badges/katharostech/arsenal/status.svg
[build_status_lnk]: https://cloud.drone.io/katharostech/arsenal

## Project Status

We're getting back into Arsenal development! For the last year our company has been focused on other initiatives, but we've finally decided it's time to start investing in Arsenal.

We currently have one person working full time on Arsenal, primarily at our own expense, with the goal of releasing Arsenal 0.2. We want to finally produce a full prototype that can actually be used to make basic games and prove that the plan we have is feasible.

We are sprinting towards this goal as fast as we can, for we can't keep this up forever. We will need your help to keep up development! Getting [sponsored on Open Collective][sponsor] is the only way that we will be able to continue full time development of Arsenal. Our hope is to make Arsenal 0.2 a release that will spark the interest of the community and show that Arsenal will do what we say it will.

We are posting weekly updates on our progress on the [Arsenal channel][arsenal_channel] on our website, so stay tuned!

[arsenal_channel]: https://katharostech.com/tag/arsenal

## The Vision

Arsenal is a Blender game engine written in Rust and developed by [Katharos Technology][katharostech.com]. With Arsenal we want to create an Open Source game engine that will bring the ability to make games of any scale, to users of any experience level. Arsenal will combine the powerful user interface of [Blender] with a high performance game engine core written in [Rust].

Friendly user-experience and beginner accessibility will be important on the list of features for Arsenal, but all of that ease of use must not come at the disadvantage of performance: Arsenal should be able to produce games of any size or complexity.

To program your games, Arsenal will incorporate scripting languages such as Python, in addition to being able to use logic nodes inside of Blender for a graphical programming experience. And if you want, you will always be able to write Rust to get the maximum performance.

Arsenal is our dream for the ultimate game engine, but there is a lot of work that needs to be done. By sponsoring us on [Open Collective][sponsor] you help bring that dream to reality. You can learn more about the [vision of Arsenal][vision] or check out the [FAQ] in the [Arsenal Documentation][docs]. We are also posting any updates on Arsenal development in the [Arsenal category][category] of our website.

[blender]: https://blender.org
[rust]: https://rust-lang.org
[sponsor]: https://opencollective.com/arsenal
[faq]: https://katharostech.github.io/arsenal/FAQ.html
[docs]: https://katharostech.github.io/arsenal/index.html
[vision]: https://katharostech.github.io/arsenal/vision.html
[category]: https://katharostech.com/tag/arsenal
[katharostech.com]: https://katharostech.com

## Proof of Concept

We have [downloads] of our first Arsenal proof-of-concept for 64bit Windows, Mac, and Linux, and a [Getting Started][started] guide that will walk you through testing it out. This is only a proof of concept and cannot actually produce games yet. In the next release of Arsenal you will be able to make simple games with scripting.

[downloads]: https://github.com/katharostech/arsenal/releases
[started]: ./getting-started.html

### Design & Architecture

The current design plan is to build Arsenal as a Blender plugin that uses the [Bevy] game engine at its core. The [architecture] documentation explains the components in more detail.

[bevy]: https://bevyengine.org/
[architecture]: ./development-guide/architecture.html

## Bugs, Feature Requests, and Questions

If you have any questions or ideas, don't hesitate to ask! Open a [GitHub issue][gh_issue] and let us know what you're thinking.

[gh_issue]: https://github.com/katharostech/arsenal/issues/new/choose

