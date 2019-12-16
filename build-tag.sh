#!/bin/bash

set -e

COMMIT=${TRAVIS_COMMIT:-immediate}

CRATE_NAME=$1
TAG=$2

if [[ "$CRATE_NAME" == "" ]] || [[ "$TAG" == "" ]]; then
  echo "Please, provide crate name and tag name \`./docker-tag.sh crate-name tag-name\`"
  exit 1
fi

docker pull "docker.pkg.github.com/howtocards/backend/$CRATE_NAME:$COMMIT"
docker tag "docker.pkg.github.com/howtocards/backend/$CRATE_NAME:$COMMIT" "docker.pkg.github.com/howtocards/backend/$CRATE_NAME:$TAG"
docker push "docker.pkg.github.com/howtocards/backend/$CRATE_NAME:$TAG"
