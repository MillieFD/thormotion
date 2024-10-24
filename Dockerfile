FROM rust:latest AS base
ARG BUILD_CONFIGURATION=release
ARG BUILD_TARGET=aarch64-unknown-linux-gnu
ARG PYTHON_VERSIONS="3.9 3.10 3.11 3.12 3.13"
WORKDIR /app

FROM base AS system
# Install system dependencies
RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y \
        wget \
        python3-pip \
        patchelf \
        libudev-dev \
        # Check which packages are included in your base image \
        # Add additional packages here if required for the build process \
        # If no packages are required, this step is automatically skipped \
    && apt-get clean \
    && apt-get autoremove \
    && rm -rf /var/lib/apt/lists/*

FROM system AS python
# Install Python versions
RUN for version in $PYTHON_VERSIONS; do \
    if [ ! -d "/usr/local/python-$version" ]; then \
        wget https://www.python.org/ftp/python/$version.0/Python-$version.0.tgz \
        && tar -xvf Python-$version.0.tgz \
        && cd Python-$version.0 \
        && ./configure --enable-optimizations \
        && make -j $(nproc) \
        && make install \
        && cd .. \
        && rm -rf Python-$version.0 Python-$version.0.tgz; \
    fi \
    done

FROM python AS venv
# Create a python virtual environment and install dependencies
ARG VENV_PATH=/app/.venv
RUN python3.13 -m venv $VENV_PATH
RUN /app/.venv/bin/pip install maturin twine

FROM venv AS source
# Copy source files in layers to take advantage of caching
COPY ./src/ ./src/
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./pyproject.toml ./pyproject.toml
COPY ./README.md ./README.md
COPY ./LICENSE ./LICENSE

FROM source AS build
# Build for each Python version
RUN for version in $PYTHON_VERSIONS; do \
        $VENV_PATH/bin/maturin build --$BUILD_CONFIGURATION --strip --target $BUILD_TARGET --interpreter /usr/local/bin/python$version; \
    done

FROM build AS publish
# Publish to PyPI
ARG PYPI_API
RUN $VENV_PATH/bin/twine upload --repository pypi -u __token__ -p $PYPI_API --skip-existing target/wheels/*
