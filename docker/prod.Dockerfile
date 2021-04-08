FROM ghcr.io/matt-gleich/profile_stack:platform

ENV RUST_LOG info
ENV RUST_BACKTRACE 1

CMD ["./profile_stack"]
