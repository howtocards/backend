#!/bin/bash

# https://medium.com/swlh/incremental-docker-builds-for-monolithic-codebases-2dae3ea950e

set -e

default_tag=${TRAVIS_COMMIT:-immediate}

CRATE_NAME=$1
TAG=${2:-$default_tag}

if [[ "$CRATE_NAME" == "" ]]; then
  echo "Please, provide crate name \`./docker-build.sh crate-name\`"
  echo "      Optional: \`./docker-build.sh crate-name latest\`"
  exit 1
fi

docker build -t "howtocards/$CRATE_NAME:$TAG" --build-arg "CRATE_NAME=$CRATE_NAME" .
docker push "howtocards/$CRATE_NAME:$TAG"
