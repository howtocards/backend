#!/bin/bash

start() {
  kill;
  nohup ./target/release/howtocards_server &
  echo !$ > /tmp/howtocards_instance.pid
}

kill() {
  cat /tmp/howtocards_instance.pid | xargs kill -9 2> /dev/null || true
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
