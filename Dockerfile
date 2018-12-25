FROM alpine:latest

ADD target/x86_64-unknown-linux-musl/release/loadapi_server /loadapi_server
EXPOSE 6767
ENTRYPOINT [ "/loadapi_server" ]
CMD [ ]
