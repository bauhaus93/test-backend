#!/bin/sh

PATH_ENV="scripts/server/env.sh"
if [ ! -f $PATH_ENV ]
then
        echo "Script not found: $PATH_ENV"
        exit 1
fi

source $PATH_ENV && \
echo "Building docker image $IMAGE_NAME from file $PATH_DOCKERFILE" && \
docker build \
	-t "$IMAGE_NAME:latest" \
	-f $PATH_DOCKERFILE \
	$PATH_SERVER
