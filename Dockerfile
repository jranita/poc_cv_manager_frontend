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

#cargo clean is to prevent strange compilation memory errors
RUN cargo clean
RUN cargo install cargo-watch
RUN cargo clean
RUN cargo install dioxus-cli

RUN

CMD ["cargo", "watch", "--why", "--", "echo"]

#it is better not to run these commands, npx --watch sometimes does not catch changes, dx serve --hot-reload sometimes does not see changes
# CMD ["npx", "tailwindcss", "-i", "./input.css", "-o", "./public/tailwind.css", "--watch", "&"]
# CMD ["dx", "serve", "--hot-reload"]
