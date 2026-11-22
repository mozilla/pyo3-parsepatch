# Almost a copy/paste from: https://github.com/PyO3/pyo3-pack/blob/master/Dockerfile

FROM quay.io/pypa/manylinux2014_x86_64@sha256:e6a785657009c612fe2a85fa933051d03a50ba62b1c5e47d62abcab546a2e423

ENV PATH=/root/.cargo/bin:$PATH
# Add all supported python versions
ENV PATH=/opt/python/cp310-cp310/bin:/opt/python/cp311-cp311/bin:/opt/python/cp312-cp312/bin:/opt/python/cp313-cp313/bin:/opt/python/cp314-cp314/bin:/opt/python/cp315-cp315/bin:$PATH
# Otherwise `cargo new` errors
ENV USER=root
ENV MATURIN_VERSION=1.16.0
# sha256 of maturin-${MATURIN_VERSION}-py3-none-manylinux2010_x86_64.manylinux_2_12_x86_64.musllinux_1_1_x86_64.whl
ARG MATURIN_SHA256=837db2f4d93d4899c3ac393fb3cc9eebf35b6002336e5d574f4a4fb6c49fe283
ARG RUSTUP_VERSION=1.29.3
ARG RUSTUP_INIT_SHA256=b1c8cb14e30889e7e2999e6b69e62d9e9b9ffc0f4df8180564d306d9680e6c55

RUN curl -fsSL https://static.rust-lang.org/rustup/archive/${RUSTUP_VERSION}/x86_64-unknown-linux-gnu/rustup-init -o rustup-init \
    && echo "${RUSTUP_INIT_SHA256}  rustup-init" | sha256sum -c - \
    && chmod +x rustup-init \
    && ./rustup-init --default-toolchain none -y \
    && rm -f rustup-init \
    && rustup set profile minimal \
    && rustup toolchain install stable \
    && rustup default stable \
    && pip3.14 install --no-cache-dir virtualenv \
    && echo "maturin==${MATURIN_VERSION} --hash=sha256:${MATURIN_SHA256}" > maturin-requirements.txt \
    && pip3.14 install --no-cache-dir --only-binary :all: --require-hashes -r maturin-requirements.txt \
    && rm -f maturin-requirements.txt

WORKDIR /rs_pp

COPY requirements-dev.txt .

COPY src src
COPY tests tests
COPY Cargo.* ./
COPY pyproject.toml ./
COPY README.md LICENSE-MPL-2.0 ./

RUN for py in 3.10 3.11 3.12 3.13 3.14 3.15; do \
      venv="/venv${py/./}"; \
      virtualenv -p "python${py}" "${venv}" \
      && ( . "${venv}/bin/activate" \
      && python -m pip install -r requirements-dev.txt \
      && maturin develop \
      && python -m pytest . ) \
      && rm -r "${venv}"; \
    done
