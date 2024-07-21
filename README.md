# Squash the creeps

[![tests](https://github.com/keeeal/squash-the-creeps/actions/workflows/tests.yaml/badge.svg)](https://github.com/keeeal/squash-the-creeps/actions/workflows/tests.yaml)
[![format](https://github.com/keeeal/squash-the-creeps/actions/workflows/format.yaml/badge.svg)](https://github.com/keeeal/squash-the-creeps/actions/workflows/format.yaml)

This is my implementation of [Godot's 3D tutorial](https://docs.godotengine.org/en/stable/getting_started/first_3d_game/index.html#) in Rust. There are many like it, but this one is mine.

### Setup

This repo assumes that you have Godot 4 installed and available either:

- in your `PATH` as `godot4`,
- or an environment variable called `GODOT4_BIN`, containing the path to the Godot executable.

### Usage

To build the project:

```
make build
```

To run the game:

```
make run
```

#### NOTE

The code in this repo should be familiar to someone folling along with the tutorial. I followed the instructions closely and tried to keep the general structure the same. However, it deviates in the following ways:
- All signals are connected in the code rather than in the Godot project using the editor. I felt this was more natural since the signals (and the methods they are connected to) constitute behaviour, many of which are also defined in the code.
- After completing the tutorial, I changed the way the score is updated. Specifically, hitting the `N`ᵗʰ creep in a combo (without touching the ground) adds `2^N` points. I did this as an example of a signal with an argument - and also because that behaviour can be seen in the gif on the tutorial's first page:

![squash-the-creeps-final.gif](https://docs.godotengine.org/en/stable/_images/squash-the-creeps-final.gif)

#### KNOWN ISSUES

- The shadows in this repo are not as crisp as those seen in the gif above. If you know how to fix this, feel free to [make an issue](https://github.com/keeeal/squash-the-creeps/issues) explaining it or [submit a PR](https://github.com/keeeal/squash-the-creeps/pulls) that fixes it :)
