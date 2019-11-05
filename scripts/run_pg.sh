#!/bin/sh

PATH_ENV="scripts/env.sh"
if [ ! -f $PATH_ENV ]
then
        echo "Script not found: $PATH_ENV"
        exit 1
fi

source $PATH_ENV && \
echo "Running container image $IMAGE_NAME_PG" && \
echo "Mapping port 5432 -> 5432" && \
docker run \
	-it --rm \
	--mount source=$VOLUMNE_NAME,target=/var/lib/postgresql/data \
	-p 5432:5432 \
	-u postgres \
	-e POSTGRES_USER="postgres" \
	-e POSTGRES_PASSWORD="secret" \
	$IMAGE_NAME_PG && \
exit 0

exit 1