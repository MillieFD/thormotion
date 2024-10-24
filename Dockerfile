FROM python:latest AS base
ARG BUILD_CONFIGURATION=release
ARG PYPI_TOKEN
WORKDIR /app

FROM base AS build

# Install system dependencies
RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y \
        libudev-dev \
        curl \
        patchelf \
	    # Check which packages are included in your base image \
	    # Add additional packages here if required for the build process \
	    # If no packages are required, this step is automatically skipped \
    && apt-get clean \
    && apt-get autoremove \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install python dependencies
RUN pip install --no-cache-dir maturin twine

# Copy source files
COPY ./src/ ./src/
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Build and publish
RUN maturin build --$BUILD_CONFIGURATION --strip
RUN twine upload --repository pypi -u __token__ -p $PYPI_TOKEN target/wheels/* || true
