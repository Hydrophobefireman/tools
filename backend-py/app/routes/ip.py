from pathlib import Path

import geoip2.database
from app.decorators.api_response import api
from app.internal.context import Context
from flask import Blueprint, request


GEO_IP_PATH = Path("/data/GeoIP/")

router = Blueprint("ip", __name__, url_prefix="/ip")


def ip_info(ip: str):
    data = {}
    with geoip2.database.Reader(GEO_IP_PATH / "GeoLite2-ASN.mmdb") as r:
        response = r.asn(ip)

    data["ASN"] = response.autonomous_system_organization
    data["ip"] = response.ip_address

    with geoip2.database.Reader(GEO_IP_PATH / "GeoLite2-City.mmdb") as r:
        response = r.city(ip)

    data["country"] = {
        "confidence": response.country.confidence,
        "name": response.country.name,
    }
    data["city"] = {"confidence": response.city.confidence, "name": response.city.name}
    return data


@router.get("/")
@api.none
def get_ip_info():
    print(
        f"{request.headers.get('Fly-Client-IP')=} {request.headers.get('X-Real-IP')=} {request.remote_addr=}"
    )
    ip = (
        request.headers.get("Fly-Client-IP")
        or request.headers.get("X-Real-IP")
        or request.remote_addr
    )
    return ip_info(ip)


@router.post("/")
@api.none
def get_particular_ip_info():
    ctx = Context()
    ip = ctx.json["ip"]
    return ip_info(ip)
