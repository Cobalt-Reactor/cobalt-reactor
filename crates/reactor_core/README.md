# `reactor_core`

A suite of tools for the Bevy engine combined into a single crate. See individual crates README.md files for more information.

## Overview of Contents

### `reactor_random`

A crate for generating random numbers using fastrand, used with feature `random`, on by default.

### `reactor_spatial`

A crate for handling and propagating 2d transforms (position, rotation, scale, draw order), used with feature `spatial`, on by default.

### `reactor_camera`

A crate for handling 2d camera functionality inspired by STALKER-X, used with feature `camera`, on by default.

### `reactor_proto`

A crate for turning on-disk assets into entities, used with feature `proto`, on by default.

### `reactor_serial`

A crate for handling saving and loading, used with feature `serial`, on by default.

### `reactor_ui`

A UI crate wrapping `sickle_ui` and adding a bunch of useful widgets, used with feature `ui`, on by default

### `reactor_perf_ui`

Adds a performance counter and hierarchy view panel, used with feature `perf_ui`, off by default.
