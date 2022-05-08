FROM ubuntu:20.04 as builder

## Install build dependencies.
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y cmake clang curl
RUN curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN ${HOME}/.cargo/bin/rustup default nightly
RUN ${HOME}/.cargo/bin/cargo install -f cargo-fuzz

ADD . /repo
WORKDIR /repo

## TODO: ADD YOUR BUILD INSTRUCTIONS HERE.
# RUN ${HOME}/.cargo/bin/cargo build --all
RUN cd fuzz && ${HOME}/.cargo/bin/cargo fuzz build

# Package Stage
FROM ubuntu:20.04


## TODO: Change <Path in Builder Stage>
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/mutate /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/incremental-parse /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/print-valid-module /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/print /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/roundtrip-valid-module /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/roundtrip /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/text-lexer /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/text-parser /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/validate-ensure-termination /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/validate-valid-module /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/validate /