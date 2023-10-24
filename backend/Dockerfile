FROM alpine:latest as prepare 
RUN mkdir /GeoLite2/
WORKDIR /GeoLite2/
ENV MAXMIND_BASE_URL "https://download.maxmind.com/app/geoip_download?license_key="


RUN --mount=type=secret,id=maxmind_license_key \
    export MAXMIND_LICENSE_KEY="$(cat /run/secrets/maxmind_license_key)" && \
    export BASE=${MAXMIND_BASE_URL}${MAXMIND_LICENSE_KEY} \
    wget "${BASE}&edition_id=GeoLite2-ASN&suffix=tar.gz" -O GeoLite2-ASN.tar.gz \
    wget "${BASE}&edition_id=GeoLite2-ASN&suffix=tar.gz.sha256" -O GeoLite2-ASN.tar.gz.sha256  \
    wget "${BASE}&edition_id=GeoLite2-City&suffix=tar.gz" -O GeoLite2-City.tar.gz \
    wget "${BASE}&edition_id=GeoLite2-City&suffix=tar.gz.sha256" -O GeoLite2-City.tar.gz.sha256

RUN sed 's/GeoLite2-ASN_[0-9]*.tar.gz/GeoLite2-ASN.tar.gz/g' -i GeoLite2-ASN.tar.gz.sha256
RUN sha256sum -c GeoLite2-ASN.tar.gz.sha256
RUN tar xvf GeoLite2-ASN.tar.gz --strip 1

RUN sed 's/GeoLite2-City_[0-9]*.tar.gz/GeoLite2-City.tar.gz/g' -i GeoLite2-City.tar.gz.sha256
RUN sha256sum -c GeoLite2-City.tar.gz.sha256
RUN tar xvf GeoLite2-City.tar.gz --strip 1
#

FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

#
FROM debian:stable-slim
WORKDIR /app
RUN mkdir -p /data/GeoIP
COPY --from=prepare /GeoLite2/*.mmdb /data/GeoIP
COPY --from=builder /app/target/release/hpfm-tool-api /app/hpfm-tool-api
EXPOSE 80
CMD ["./hpfm-tool-api"]
