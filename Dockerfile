# Almost a copy/paste from: https://github.com/PyO3/pyo3-pack/blob/master/Dockerfile

FROM quay.io/pypa/manylinux2014_x86_64@sha256:e6a785657009c612fe2a85fa933051d03a50ba62b1c5e47d62abcab546a2e423

ENV PATH=/root/.cargo/bin:$PATH
# Add all supported python versions
ENV PATH=/opt/python/cp310-cp310/bin:/opt/python/cp311-cp311/bin:/opt/python/cp312-cp312/bin:/opt/python/cp313-cp313/bin:/opt/python/cp314-cp314/bin:/opt/python/cp315-cp315/bin:$PATH
# Otherwise `cargo new` errors
ENV USER=root
ENV MATURIN_VERSION=1.16.0
ARG MUSL_VERSION=1.2.6
ARG MUSL_SHA256=d585fd3b613c66151fc3249e8ed44f77020cb5e6c1e635a616d3f9f82460512a
ARG RUSTUP_VERSION=1.29.3
ARG RUSTUP_INIT_SHA256=b1c8cb14e30889e7e2999e6b69e62d9e9b9ffc0f4df8180564d306d9680e6c55

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

RUN for py in 3.10 3.11 3.12 3.13 3.14 3.15; do \
      venv="/venv${py/./}"; \
      virtualenv -p "python${py}" "${venv}" \
      && ( . "${venv}/bin/activate" \
      && python -m pip install -r requirements-dev.txt \
      && maturin develop \
      && python -m pytest . ) \
      && rm -r "${venv}"; \
    done


ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN maturin build --target x86_64-unknown-linux-musl --manylinux off --interpreter python3.10 python3.11 python3.12 python3.13 python3.14 python3.15

CMD ["maturin", "publish", "--interpreter", "python3.10", "python3.11", "python3.12", "python3.13", "python3.14", "python3.15"]
