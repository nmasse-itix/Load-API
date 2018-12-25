FROM alpine:latest

ADD target/release/loadapi_server /loadapi_server
EXPOSE 6767
ENTRYPOINT [ "/loadapi_server" ]
CMD [ ]
