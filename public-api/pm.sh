#!/bin/bash

start() {
  kill;
  nohup ../target/release/howtocards-public-api &
  echo !$ > /tmp/howtocards-public-api.pid
}

kill() {
  cat /tmp/howtocards-public-api.pid | xargs kill -9 2> /dev/null || true
}

case "$1" in
  "start")
    start
  ;;

  "stop")
    kill
  ;;

  "restart")
    kill
    start
  ;;

  "test")
    echo "Hello Test Pm"
  ;;
esac
