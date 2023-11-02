FROM alpine:latest as prepare 
RUN mkdir /GeoLite2/
WORKDIR /GeoLite2/
ENV MAXMIND_BASE_URL "https://download.maxmind.com/app/geoip_download?license_key="


RUN --mount=type=secret,id=maxmind_license_key \
    URL="${MAXMIND_BASE_URL}$(cat /run/secrets/maxmind_license_key)" && \
    wget "${URL}&edition_id=GeoLite2-ASN&suffix=tar.gz" -O GeoLite2-ASN.tar.gz && \
    wget "${URL}&edition_id=GeoLite2-ASN&suffix=tar.gz.sha256" -O GeoLite2-ASN.tar.gz.sha256 && \
    wget "${URL}&edition_id=GeoLite2-City&suffix=tar.gz" -O GeoLite2-City.tar.gz && \
    wget "${URL}&edition_id=GeoLite2-City&suffix=tar.gz.sha256" -O GeoLite2-City.tar.gz.sha256

RUN sed 's/GeoLite2-ASN_[0-9]*.tar.gz/GeoLite2-ASN.tar.gz/g' -i GeoLite2-ASN.tar.gz.sha256
RUN sha256sum -c GeoLite2-ASN.tar.gz.sha256
RUN tar xvf GeoLite2-ASN.tar.gz --strip 1

RUN sed 's/GeoLite2-City_[0-9]*.tar.gz/GeoLite2-City.tar.gz/g' -i GeoLite2-City.tar.gz.sha256
RUN sha256sum -c GeoLite2-City.tar.gz.sha256
RUN tar xvf GeoLite2-City.tar.gz --strip 1
#

FROM rust:latest as builder
WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

#
FROM alpine:latest
WORKDIR /app
RUN mkdir -p /data/GeoIP
COPY --from=prepare /GeoLite2/*.mmdb /data/GeoIP
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/tool-api ./

EXPOSE 8000
CMD ["./tool-api"]
