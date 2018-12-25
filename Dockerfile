FROM centos:7

ADD target/release/loadapi_server /loadapi_server

ENTRYPOINT [ "/loadapi_server" ]
CMD [ ]
