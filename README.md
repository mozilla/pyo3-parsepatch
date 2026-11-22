# pyo3-parsepatch

A Python wrapper for https://github.com/mozilla/rust-parsepatch.  
The goal of this library is to be able to parse the patches coming from mercurial.  
All the patches in https://hg.mozilla.org/mozilla-central/ have been successfully parsed !  
It's used in https://github.com/mozilla/bugbug to get some metrics on patches.

## License

Published under the MPL 2.0 license.

## Publish

Bump the version in `Cargo.toml` and `pyproject.toml`, then push a matching tag:
```sh
git tag v0.4.8
git push origin v0.4.8
```
The [release workflow](.github/workflows/release.yml) builds the wheels (manylinux, macOS
universal2 and Windows x86_64) and the sdist, tests them and publishes them on PyPI.

## Contact

Email: calixte@mozilla.com
