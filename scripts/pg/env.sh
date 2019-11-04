#!/bin/sh

IMAGE_NAME="backend-postgresql"
VOLUMNE_NAME="vol-pg-data"

PATH_PG="$PWD/postgresql"
PATH_CONFIG="$PATH_PG/config"
PATH_INIT="$PATH_PG/init"
PATH_DOCKERFILE="$PATH_PG/Dockerfile"

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
