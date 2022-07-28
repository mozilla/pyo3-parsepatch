# Almost a copy/paste from: https://github.com/PyO3/pyo3-pack/blob/master/Dockerfile

FROM quay.io/pypa/manylinux2010_x86_64

ENV PATH /root/.cargo/bin:$PATH
# Add all supported python versions
ENV PATH /opt/python/cp37-cp37m/bin/:/opt/python/cp38-cp38/bin/:/opt/python/cp39-cp39/bin/:/opt/python/cp310-cp310/bin:$PATH
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
    && yum install -y libffi-devel \
    && python3 -m pip install --no-cache-dir virtualenv \
    && cargo install maturin

WORKDIR /rs_pp

ADD requirements-dev.txt .

ADD src src
ADD tests tests
ADD Cargo.* ./
ADD pyproject.toml ./
ADD README.md README.md

RUN virtualenv -p python3.7 /venv37 && . /venv37/bin/activate && python -m pip install --no-cache-dir -r requirements-dev.txt && maturin develop && python -m pytest . && rm -r /venv37
RUN virtualenv -p python3.8 /venv38 && . /venv38/bin/activate && python -m pip install --no-cache-dir -r requirements-dev.txt && maturin develop && python -m pytest . && rm -r /venv38
RUN virtualenv -p python3.9 /venv39 && . /venv39/bin/activate && python -m pip install --no-cache-dir -r requirements-dev.txt && maturin develop && python -m pytest . && rm -r /venv39
RUN virtualenv -p python3.10 /venv310 && . /venv310/bin/activate && python -m pip install -r requirements-dev.txt && maturin develop && python -m pytest . && rm -r /venv310

ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN maturin build --target x86_64-unknown-linux-musl --manylinux off

CMD ["maturin", "publish"]
