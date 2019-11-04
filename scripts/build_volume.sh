#!/bin/sh

PATH_ENV="scripts/env.sh"
if [ ! -f $PATH_ENV ]
then
        echo "Script not found: $PATH_ENV"
        exit 1
fi

source $PATH_ENV && \
docker volume rm -f $VOLUMNE_NAME && \
docker volume create \
	--label "postgres-data" \
	--name $VOLUMNE_NAME
