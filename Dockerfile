# Almost a copy/paste from: https://github.com/PyO3/pyo3-pack/blob/master/Dockerfile

FROM quay.io/pypa/manylinux2014_x86_64@sha256:4e399b6b7f2eb9cfd8a769efb2d0268061df939fa95ea12008b89b44dea56ce3

ENV PATH /root/.cargo/bin:$PATH
# Add all supported python versions
ENV PATH /opt/python/cp310-cp310/bin:/opt/python/cp311-cp311/bin:/opt/python/cp312-cp312/bin:/opt/python/cp313-cp313/bin:/opt/python/cp314-cp314/bin:$PATH
# Otherwise `cargo new` errors
ENV USER root
ENV MATURIN_VERSION=1.12.2
ARG MUSL_SHA256=44be8771d0e6c6b5f82dd15662eb2957c9a3173a19a8b49966ac0542bbd40d61
ARG RUSTUP_VERSION=1.28.2
ARG RUSTUP_INIT_SHA256=20a06e644b0d9bd2fbdbfd52d42540bdde820ea7df86e92e533c073da0cdd43c

RUN curl -fsSL https://www.musl-libc.org/releases/musl-1.1.20.tar.gz -o musl.tar.gz \
    && echo "${MUSL_SHA256}  musl.tar.gz" | sha256sum -c - \
    && tar -xzf musl.tar.gz \
    && rm -f musl.tar.gz \
    && cd musl-1.1.20 \
    && ./configure \
    && make install -j$(expr $(nproc) \+ 1) \
    && cd .. \
    && rm -rf x86_64-unknown-linux-musl \
    && curl -fsSL https://static.rust-lang.org/rustup/archive/${RUSTUP_VERSION}/x86_64-unknown-linux-gnu/rustup-init -o rustup-init \
    && echo "${RUSTUP_INIT_SHA256}  rustup-init" | sha256sum -c - \
    && chmod +x rustup-init \
    && ./rustup-init --default-toolchain none -y \
    && rm -f rustup-init \
    && rustup set profile minimal \
    && rustup toolchain install stable --target x86_64-unknown-linux-musl \
    && rustup default stable \
    && yum install -y libffi-devel \
    && pip3.14 install --no-cache-dir virtualenv \
    && cargo install --locked maturin --version ${MATURIN_VERSION}

WORKDIR /rs_pp

ADD requirements-dev.txt .

ADD src src
ADD tests tests
ADD Cargo.* ./
ADD pyproject.toml ./
ADD README.md README.md

RUN virtualenv -p python3.10 /venv310 && . /venv310/bin/activate && python -m pip install -r requirements-dev.txt && maturin develop && python -m pytest . && rm -r /venv310
RUN virtualenv -p python3.11 /venv311 && . /venv311/bin/activate && python -m pip install -r requirements-dev.txt && maturin develop && python -m pytest . && rm -r /venv311
RUN virtualenv -p python3.12 /venv312 && . /venv312/bin/activate && python -m pip install -r requirements-dev.txt && maturin develop && python -m pytest . && rm -r /venv312
RUN virtualenv -p python3.13 /venv313 && . /venv313/bin/activate && python -m pip install -r requirements-dev.txt && maturin develop && python -m pytest . && rm -r /venv313
RUN virtualenv -p python3.14 /venv314 && . /venv314/bin/activate && python -m pip install -r requirements-dev.txt && maturin develop && python -m pytest . && rm -r /venv314


ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN maturin build --target x86_64-unknown-linux-musl --manylinux off --interpreter python3.10 python3.11 python3.12 python3.13 python3.14

CMD ["maturin", "publish", "--interpreter", "python3.10", "python3.11", "python3.12", "python3.13", "python3.14"]
