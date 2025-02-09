# ----------------------------------------------
# Install Dependencies and Build
# ----------------------------------------------
FROM rust:1-slim as builder

# Install system dependencies.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        build-essential \
        curl \
        tar && \
    rm -rf /var/lib/apt/lists/*

# Install Binaryen from source (Debian’s version is too old).
ENV BINARYEN_VERSION=120_b
ENV BINARYEN_URL=https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz

RUN curl -L -o binaryen.tar.gz ${BINARYEN_URL} && \
    tar -xzf binaryen.tar.gz && \
    mv binaryen-version_${BINARYEN_VERSION} /opt/binaryen && \
    ln -s /opt/binaryen/bin/wasm-opt /usr/local/bin/wasm-opt && \
    rm binaryen.tar.gz

# Add the wasm target and install Trunk.
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

WORKDIR /app

# Copy the project files.
COPY Cargo.toml Cargo.lock ./
COPY index.html ./
COPY styles.css ./
COPY src ./src
COPY static/resources ./static/resources

# Build the application with Trunk (the generated files will be included in the build).
RUN trunk build --release --dist=dist

# ----------------------------------------------
# Launch with Nginx and the Image List Generator
# ----------------------------------------------
FROM nginx:bookworm

# Install inotifywait
RUN apt update && apt install -y inotify-tools

# Remove the default Nginx static assets.
RUN rm -rf /usr/share/nginx/html/*

# Copy the built assets from the builder stage.
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy the image list generator script into the final image.
COPY generate_images_json.sh /usr/share/nginx/html/static/generate_images_json.sh
RUN chmod +x /usr/share/nginx/html/static/generate_images_json.sh

# Set the working directory.
WORKDIR /usr/share/nginx/html/static

EXPOSE 80

# Start the image list generator in the background and launch Nginx.
CMD sh -c "./generate_images_json.sh & nginx -g 'daemon off;'"
