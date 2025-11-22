[![Tests](https://github.com/Emilinya/empc/actions/workflows/tests.yml/badge.svg)](https://github.com/Emilinya/empc/actions/workflows/tests.yml)

# Emilie's Media PC

The first version of Emilie's Media PC. This is a rewrite of and improvement upon [Mort's Media PC](https://github.com/mortie/mmpc2).

## Goal

EMPC makes it possible to use a web browser to control a computer remotely. With it, you can browse files, play and stream videos while controlling the playback remotely, and move the mouse and input text while looking at a live screen capture.

## Compiling

EMPC uses the rust web framework [dioxus](https://dioxuslabs.com/), which has a custom build tool, `dx`, that you can install by following [these instructions](https://dioxuslabs.com/learn/0.7/getting_started/).

To capture and share the screen, EMPC depends on [pipewire](https://pipewire.org/). Note that this is only required on linux; when running EMPC on MacOS or Windows, the remote desktop functionality is disabled.

Once the dependencies are installed, you simply need to run `dx serve` to compile and run both the backend and frontend. To create an optimized build, run `dx bundle --release` instead.
