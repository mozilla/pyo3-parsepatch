# pyo3-parsepatch

A Python wrapper for https://github.com/mozilla/rust-parsepatch.  
The goal of this library is to be able to parse the patches coming from mercurial.  
All the patches in https://hg.mozilla.org/mozilla-central/ have been successfully parsed !  
It's used in https://github.com/mozilla/bugbug to get some metrics on patches.

## License

Published under the MPL 2.0 license.

## Publish

Install docker and then:
```sh
docker build -t rs_pp
```
It will compile everything and run tests in a manylinux environment.

And then:
```sh
docker run -it rs_pp
```
to publish the packages on Pypi.

## Contact

Email: calixte@mozilla.com
