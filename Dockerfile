# Almost a copy/paste from: https://github.com/PyO3/pyo3-pack/blob/master/Dockerfile

FROM quay.io/pypa/manylinux2014_x86_64@sha256:82c97be7faa3329f267bda90372dc0208175cef2b1ea20bfa72f13a955fdba01

ENV PATH /root/.cargo/bin:$PATH
# Add all supported python versions
ENV PATH /opt/python/cp310-cp310/bin:/opt/python/cp311-cp311/bin:/opt/python/cp312-cp312/bin:/opt/python/cp313-cp313/bin:/opt/python/cp314-cp314/bin:$PATH
# Otherwise `cargo new` errors
ENV USER root
ENV MATURIN_VERSION=1.12.2
ARG MUSL_VERSION=1.2.5
ARG MUSL_SHA256=a9a118bbe84d8764da0ea0d28b3ab3fae8477fc7e4085d90102b8596fc7c75e4
ARG RUSTUP_VERSION=1.28.2
ARG RUSTUP_INIT_SHA256=20a06e644b0d9bd2fbdbfd52d42540bdde820ea7df86e92e533c073da0cdd43c

RUN curl -fsSL https://www.musl-libc.org/releases/musl-${MUSL_VERSION}.tar.gz -o musl.tar.gz \
    && echo "${MUSL_SHA256}  musl.tar.gz" | sha256sum -c - \
    && tar -xzf musl.tar.gz \
    && rm -f musl.tar.gz \
    && cd musl-${MUSL_VERSION} \
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

COPY requirements-dev.txt .

COPY src src
COPY tests tests
COPY Cargo.* ./
COPY pyproject.toml ./
COPY README.md README.md

RUN for py in 3.10 3.11 3.12 3.13 3.14; do \
      venv="/venv${py/./}"; \
      virtualenv -p "python${py}" "${venv}" \
      && ( . "${venv}/bin/activate" \
      && python -m pip install -r requirements-dev.txt \
      && maturin develop \
      && python -m pytest . ) \
      && rm -r "${venv}"; \
    done


ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN maturin build --target x86_64-unknown-linux-musl --manylinux off --interpreter python3.10 python3.11 python3.12 python3.13 python3.14

CMD ["maturin", "publish", "--interpreter", "python3.10", "python3.11", "python3.12", "python3.13", "python3.14"]
