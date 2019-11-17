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

sed "s/{{CRATE_NAME}}/$CRATE_NAME/g" Dockerfile.template > "$CRATE_NAME.Dockerfile"

docker build -t "howtocards/$CRATE_NAME:$TAG" -f "$CRATE_NAME.Dockerfile" .
docker push "howtocards/$CRATE_NAME:$TAG"

rm -f "$CRATE_NAME.Dockerfile"
