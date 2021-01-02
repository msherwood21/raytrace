# Goal
Raytracer interface capable of modifying the scene, starting a render, canceling an in progress
render and providing real time render updates.

## Requirements
- ~The raytracer shall output an image in the Portable Pixmap (PPM) format~
- The raytracer shall define an interface for a client to consume the render output
- The raytracer shall output image information as soon as it is available
- The raytracer shall perform operations in a thread separate from the one initializing the render
- The raytracer shall expose an interface to modify the following scene values:
  - Image width
  - Aspect ratio
  - Camera location
  - Rays per pixel
  - Number of spheres
  - Material type of spheres
- The raytracer shall only allow editing the scene values when a render is not in progress
- The raytracer shall define an interface capable of canceling an in progress render

# Current Deficiencies
- Incapable of modifying scene data without editing code
- Can't cancel an in progress render due to lack of threading
- Can't capture real time render updates due to a lack of threading and output options

# Proposed Design (Updated as more info becomes clear)
- Create `render` module in `lib.rs`
  - Rename `lib::run` to `render::init` to setup scene data
  - Move the rendering loop and supported operations to public `render` functions