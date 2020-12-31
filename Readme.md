Raytrace
========

Initially this was the result of me going through
[Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).
Instead of coding it in C++, I did it in Rust to teach myself the language. Now that I've
accomplished that goal I'm creating a UI to manipulate the scene and re-render images without
changing code, recompiling and re-running.

I've committed my run of the [final render](final.ppm) from section 13.1 of the book. The render
took several hours on a Surface Pro 6 so I decided to spare future observers the pain.

## Usage
Check the [License](License.md) file for usage details. The license did change after tag `book-code`.

## Contributing
Just fork it and go on your merry way. Once I'm done learning I will not be touching the repository
again.

## Versions
- Rust toolchain: 1.45.1
- Ray Tracing In One Weekend: v3.2.0

## Roadmap To Goal
- Refactor to library
  - ~~Create binary that essentially calls main~~
- Docs
  - ~~Document the current interface and logic~~
- Refactor
  - Record new design for interfacing with the raytracer library
  - Identify changes needed to facilitate design
    - Create integration test(s) / unit test(s)
    - Refactor section under test
      - Update associated documentation
- GUI prototype
- GUI binary