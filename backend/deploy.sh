#!/usr/bin/env bash
MAXMIND_LICENSE_KEY=$(cat ../maxmind_license.txt) fly deploy --build-secret MAXMIND_LICENSE_KEY=$MAXMIND_LICENSE_KEY
