#!/bin/bash

set -e

COMMIT=${TRAVIS_COMMIT:-immediate}

CRATE_NAME=$1
TAG=$2

if [[ "$CRATE_NAME" == "" ]] || [[ "$TAG" == "" ]]; then
  echo "Please, provide crate name and tag name \`./docker-tag.sh crate-name tag-name\`"
  exit 1
fi

docker pull "howtocards/$CRATE_NAME:$COMMIT"
docker tag "howtocards/$CRATE_NAME:$COMMIT" "howtocards/$CRATE_NAME:$TAG"
docker push "howtocards/$CRATE_NAME:$TAG"
