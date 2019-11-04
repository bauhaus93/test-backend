PATH_ENV="scripts/env.sh"
if [ ! -f $PATH_ENV ]
then
        echo "Script not found: $PATH_ENV"
        exit 1
fi

source $PATH_ENV && \
echo "Running container image $IMAGE_NAME_SRC" && \
echo "Mapping port 10001 -> 10001" && \
docker run \
	-it --rm \
	-p 10001:10001 \
	$IMAGE_NAME_SRV && \
exit 0

exit 1 
