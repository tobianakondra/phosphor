# Phosphor

Phosphor is a planned custom graphical environment for Redox OS. Its visual
identity is the dark teal desktop design in `Bureau et lanceur@2x.png`: a file
manager with a contextual assistant panel, a large app launcher, and a persistent
bottom bar. Phosphor will provide its own rendering and window management using
Redox's existing hardware drivers. Orbital is not a runtime dependency.

## Current state

The first piece is a Rust software-rendering core: an owned RGBA surface,
vertical gradients, clipped shapes, alpha blending, and a static desktop mockup
following the selected design. Text and final icons are still placeholders. It
is not yet connected to a screen or input device.

## Roadmap

1. Identify and implement a supported Redox display and input path that does
   not require Orbital.
2. Present the static desktop mockup on the Redox display in QEMU.
3. Add pointer and keyboard event handling.
4. Add application surfaces and window management.

Redox's build configuration includes a `base` package with essential services
and drivers, while its display/input drivers are separate user-space components.
The exact public screen interface must be established before the backend is
implemented; Phosphor does not assume that it can simply write to a framebuffer.

## Build

Generate a host-side preview image with:

```sh
cargo run --example preview -- /tmp/phosphor-preview.ppm
```

This preview exercises the backend-independent renderer only. Displaying
Phosphor inside Redox still requires the Redox display/input backend and a
matching Redox target toolchain.
