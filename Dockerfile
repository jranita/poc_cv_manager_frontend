FROM rust:latest

WORKDIR /app/

COPY ./app .

RUN apt-get -yq update \
    && apt-get -yq install nodejs \
    && apt-get -yq install npm \
    && npm install tailwindcss

RUN rustup default
RUN rustup update
RUN rustup target add wasm32-unknown-unknown
RUN rustup component add clippy

RUN cargo install cargo-watch
RUN cargo install dioxus-cli

# CMD ["cargo", "watch", "--why", "--", "echo"]
CMD ["cargo", "watch", "-x", "run --release"]
# CMD ["npx", "tailwindcss", "-i", "./input.css", "-o", "./public/tailwind.css", "--watch"]
