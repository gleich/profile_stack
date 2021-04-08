FROM ghcr.io/matt-gleich/api:platform

ENV RUST_LOG info
ENV RUST_BACKTRACE 1

CMD ["./profile_stack"]
