#!/usr/bin/env bash
fly deploy --build-secret maxmind_license_key=$(cat ../maxmind_license.txt)
