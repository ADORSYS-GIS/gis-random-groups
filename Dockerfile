
FROM rust:latest as builder


WORKDIR /usr/src/app


COPY . .


RUN cargo build --release


FROM debian:buster-slim


COPY --from=builder /usr/src/app/target/release/gis_random_groups /usr/local/bin/gis_random_groups

# run application
CMD ["gis_random_groups"]