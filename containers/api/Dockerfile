FROM rust:1.53-alpine3.12

WORKDIR /src/app

# adding cargo dependencies
RUN apk add --no-cache gcc musl-dev python3-dev \
    libffi-dev openssl-dev curl wget build-base autoconf \
    automake openssh libffi-dev libressl-dev

# copying and fetching api dependencies
COPY . .
RUN cargo build

EXPOSE 3000
CMD [ "cargo", "run" ]
