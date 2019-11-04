#!/bin/sh

scripts/build_volume.sh && \
scripts/build_pg_image.sh && \
scripts/build_server_image.sh
