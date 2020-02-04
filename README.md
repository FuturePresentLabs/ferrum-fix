# Fasters

Fasters is a library written in pure Rust to consume and receive FIX and FAST

Copyright for terminology and documentation is FIX Protocol Ltd..

Fasters is, at the time of writing, **not** production ready. I currently can't afford to put in the hours and make it robust and fully standard-compliant. Rigor is nevertheless a core goal of the project and it might serve as a useful foundation for others' work.

See the following feature roadmap:

**FIX:**

- [ ] FIXML parser.
- [ ] Compact notation parser.
- [ ] Code generation (Rust).
- [ ] Standard (4.1 and following) tags.
- [ ] Custom fields.

**FAST:**

- [ ] FAST XML template definition.
- [ ] FAST compact notation.

As dictated by [SemVer 2.0](https://semver.org/), I will bump the major version to `1` once I've settled on sensible APIs.

Fasters is intended to be an all-in-one tool for everything concerning FIX & FAST data. You can either use it for code generation at build-time or at runtime to help you (de)serialize schemaless FAST data streams. FIX is more static and you'll generally want to use code generation.

- [Official documentation.](https://www.fixtrading.org/standards/fast/)
- [FAST on Wikipedia.](https://en.wikipedia.org/wiki/FAST_protocol)

---

Available under the terms of the MIT license.