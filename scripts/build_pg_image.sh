#!/bin/sh

PATH_ENV="scripts/env.sh"
if [ ! -f $PATH_ENV ]
then
        echo "Script not found: $PATH_ENV"
        exit 1
fi

source $PATH_ENV && \
echo "Building docker image $IMAGE_NAME_PG from file $PATH_DOCKERFILE_PG" && \
docker build \
	-t "$IMAGE_NAME_PG:latest" \
	-f $PATH_DOCKERFILE_PG \
	$PATH_PG && \
exit 0

exit 1

