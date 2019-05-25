FROM quay.io/pypa/manylinux2010_x86_64

ENV PATH /root/.cargo/bin:$PATH
# Add all supported python versions
ENV PATH /opt/python/cp35-cp35m/bin/:/opt/python/cp36-cp36m/bin/:/opt/python/cp37-cp37m/bin/:$PATH
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
    && curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && rustup toolchain install nightly \
    && rustup default nightly \
    && rustup target add x86_64-unknown-linux-musl \
    && mkdir /io \
    && python3 -m pip install cffi virtualenv \
    && cargo install pyo3-pack

WORKDIR /rs_pp

ADD requirements-dev.txt .

RUN virtualenv -p python3 venv
RUN . venv/bin/activate && python -m pip install -r requirements-dev.txt

ADD src src
ADD tests tests
ADD Cargo.* ./

RUN . venv/bin/activate && pyo3-pack develop && python -m pytest .

RUN pyo3-pack build

CMD ["pyo3-pack", "publish"]
