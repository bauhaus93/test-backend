#!/bin/sh

IMAGE_NAME_PG="backend-postgresql"
IMAGE_NAME_SRV="backend-server"
VOLUMNE_NAME="vol-pg-data"

PATH_DOCKER="scripts/docker"
PATH_DOCKERFILE_PG="$PATH_DOCKER/postgresql/Dockerfile"
PATH_DOCKERFILE_SRV="$PATH_DOCKER/server/Dockerfile"

PATH_PG="$PWD/postgresql"
PATH_SERVER="$PWD/server"

PATH_PG_CONFIG="$PATH_PG/config"
PATH_PG_INIT="$PATH_PG/init"

if [ ! -d $PATH_CONFIG ]
then
	echo "PATH PATH_CONFIG not existing: $PATH_CONFIG"
	exit 1
fi

if [ ! -d $PATH_INIT ]
then
	echo "Directory PATH_INIT not existing: $PATH_INIT"
	exit 1
fi
