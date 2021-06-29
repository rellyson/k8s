FROM rust:1.53-alpine3.12

WORKDIR /src/app

# copying and fetching api dependencies
COPY . .
RUN cargo fetch

EXPOSE 3000
CMD [ "cargo", "run" ]
