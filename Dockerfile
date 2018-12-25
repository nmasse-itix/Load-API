FROM rust:slim

ADD . /opt/src
RUN cd /opt/src && cargo build --release && cp target/release/loadapi_server /loadapi_server && rm -rf target
EXPOSE 6767
ENTRYPOINT [ "/loadapi_server" ]
CMD [ ]
