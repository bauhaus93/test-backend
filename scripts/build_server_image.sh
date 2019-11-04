#!/bin/sh

PATH_ENV="scripts/env.sh"
if [ ! -f $PATH_ENV ]
then
        echo "Script not found: $PATH_ENV"
        exit 1
fi

source $PATH_ENV && \
echo "Building docker image $IMAGE_NAME_SRV from file $PATH_DOCKERFILE_SRV" && \
docker build \
	-t "$IMAGE_NAME_SRV:latest" \
	-f $PATH_DOCKERFILE_SRV \
	$PATH_SERVER
