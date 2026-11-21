# Phosphor

Phosphor's rendering core is a `no_std` Rust library using `alloc`. Its visual
identity is the dark teal desktop design in `Bureau et lanceur@2x.png`: a file
manager with a contextual assistant panel, a large app launcher, and a persistent
bottom bar. Phosphor will provide its own rendering and window management using
Redox's existing hardware drivers. Orbital is not a runtime dependency.

## Current state

The Rust software-rendering core now supports TrueType text and draws labels,
file names, assistant messages, and distinct launcher pictograms in the static
desktop mockup. Window interaction and Redox screen/input access remain to be
implemented. The library requires an allocator supplied by its host; the preview
example uses the standard library and supplies that environment on the desktop.

## Rendering backends

`scene::render_desktop` and `text::Typeface::draw` accept any implementation of
`RenderTarget`. A backend provides the target dimensions, alpha-blended pixels,
rectangles, and rounded rectangles; the trait supplies clipped helpers for
gradients, ellipses, and lines. `Surface` is the owned RGBA implementation used
by the host preview.

PrismOS can implement `RenderTarget` for a small local wrapper around
`prism_core::gfx::Surface`. That wrapper can map Phosphor's RGBA colors to
Prism's RGB pixel operations and alpha blending without making Phosphor depend
on PrismOS. The adapter belongs in the integrating kernel because Rust's
orphan rules prevent implementing Phosphor's trait directly for Prism's type.
For the crate structure and render flow, see
[`docs/architecture.md`](docs/architecture.md). For the current PrismOS
adapter, its activation command, and its limits, see
[`docs/prism-os-integration.md`](docs/prism-os-integration.md).

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

DejaVu Sans is loaded from the host's standard font paths. To select it
explicitly, pass its path after the output path. On Redox, the Redox `dejavu`
package installs it under `/usr/share/fonts/Sans/DejaVu/DejaVuSans.ttf`.

This preview exercises the backend-independent renderer only. Displaying
Phosphor inside Redox still requires the Redox display/input backend and a
matching Redox target toolchain.
