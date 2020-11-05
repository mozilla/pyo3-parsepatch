# Almost a copy/paste from: https://github.com/PyO3/pyo3-pack/blob/master/Dockerfile

FROM quay.io/pypa/manylinux2010_x86_64

ENV PATH /root/.cargo/bin:$PATH
# Add all supported python versions
ENV PATH /opt/python/cp35-cp35m/bin/:/opt/python/cp36-cp36m/bin/:/opt/python/cp37-cp37m/bin/:/opt/python/cp38-cp38/bin/:/opt/python/cp39-cp39/bin/:$PATH
# Otherwise `cargo new` errors
ENV USER root

RUN curl https://www.musl-libc.org/releases/musl-1.1.20.tar.gz -o musl.tar.gz \
    && tar -xzf musl.tar.gz \
    && rm -f musl.tar.gz \
    && cd musl-1.1.20 \
    && ./configure \
    && make install -j$(expr $(nproc) \+ 1) \
    && cd .. \
    && rm -rf x86_64-unknown-linux-musl \
    && curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain none -y \
    && rustup set profile minimal \
    && rustup toolchain install nightly --target x86_64-unknown-linux-musl \
    && rustup default nightly \
    && python3 -m pip install cffi virtualenv \
    && cargo install maturin

WORKDIR /rs_pp

ADD requirements-dev.txt .

RUN virtualenv -p python3 venv
RUN . venv/bin/activate && python -m pip install -r requirements-dev.txt

ADD src src
ADD tests tests
ADD Cargo.* ./
ADD pyproject.toml ./

RUN . venv/bin/activate && maturin develop && python -m pytest .

ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN maturin build --target x86_64-unknown-linux-musl --manylinux off

CMD ["maturin", "publish"]
