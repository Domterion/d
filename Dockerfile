
FROM rust:latest

#RUN apk add --no-cache gcc lld musl-dev curl openssl-dev
RUN mkdir /d
WORKDIR /d
COPY . .
RUN cargo install --path .
EXPOSE 8087

CMD ["d"]
