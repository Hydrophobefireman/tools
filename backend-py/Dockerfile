FROM alpine:3.18 as prepare

RUN mkdir /GeoLite2/
WORKDIR /GeoLite2/

ENV MAXMIND_BASE_URL "https://download.maxmind.com/app/geoip_download?license_key="


RUN --mount=type=secret,id=MAXMIND_LICENSE_KEY \
    export MAXMIND_LICENSE_KEY="$(cat /run/secrets/MAXMIND_LICENSE_KEY)" && wget "${MAXMIND_BASE_URL}${MAXMIND_LICENSE_KEY}&edition_id=GeoLite2-ASN&suffix=tar.gz" -O GeoLite2-ASN.tar.gz

RUN --mount=type=secret,id=MAXMIND_LICENSE_KEY \
    export MAXMIND_LICENSE_KEY="$(cat /run/secrets/MAXMIND_LICENSE_KEY)" && wget "${MAXMIND_BASE_URL}${MAXMIND_LICENSE_KEY}&edition_id=GeoLite2-ASN&suffix=tar.gz.sha256" -O GeoLite2-ASN.tar.gz.sha256
RUN sed 's/GeoLite2-ASN_[0-9]*.tar.gz/GeoLite2-ASN.tar.gz/g' -i GeoLite2-ASN.tar.gz.sha256
RUN sha256sum -c GeoLite2-ASN.tar.gz.sha256
RUN tar xvf GeoLite2-ASN.tar.gz --strip 1

RUN --mount=type=secret,id=MAXMIND_LICENSE_KEY \
    export MAXMIND_LICENSE_KEY="$(cat /run/secrets/MAXMIND_LICENSE_KEY)" && wget "${MAXMIND_BASE_URL}${MAXMIND_LICENSE_KEY}&edition_id=GeoLite2-City&suffix=tar.gz" -O GeoLite2-City.tar.gz
RUN --mount=type=secret,id=MAXMIND_LICENSE_KEY \
    export MAXMIND_LICENSE_KEY="$(cat /run/secrets/MAXMIND_LICENSE_KEY)" && wget "${MAXMIND_BASE_URL}${MAXMIND_LICENSE_KEY}&edition_id=GeoLite2-City&suffix=tar.gz.sha256" -O GeoLite2-City.tar.gz.sha256
RUN sed 's/GeoLite2-City_[0-9]*.tar.gz/GeoLite2-City.tar.gz/g' -i GeoLite2-City.tar.gz.sha256
RUN sha256sum -c GeoLite2-City.tar.gz.sha256
RUN tar xvf GeoLite2-City.tar.gz --strip 1

FROM python:3.11-alpine as release
RUN mkdir -p /data/GeoIP
COPY --from=prepare /GeoLite2/*.mmdb /data/GeoIP

WORKDIR /app

COPY . .
RUN pip install --no-cache-dir -U pip wheel
RUN pip install --no-cache-dir -r requirements.txt 

EXPOSE 80
CMD ["gunicorn", "app.main:app", "-c", "gunicorn.conf.py","--access-logfile","-"]