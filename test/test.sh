#!/bin/sh

die() {
  echo "$*"
  kill -9 "$pid"
  exit 1
}

node server.js &
pid=$!

alias sumotime=../target/debug/sumotime

sleep 1

# shellcheck disable=SC2034
export SUMOTIME_URL="http://localhost:8080"

sumotime -k 'foo.bar' false && die "expected exit code to passthru"
[ "$(curl -sS "$SUMOTIME_URL")" = '{"msg":"sumotime","key":"foo.bar","code":1,"timeout":false,"duration":0}' ] || die "didn't POST properly"

sumotime -k 'foo.bar' -t 1 sleep 3 && die "expected timeout"
[ "$(curl -sS "$SUMOTIME_URL")" = '{"msg":"sumotime","key":"foo.bar","code":124,"timeout":true,"duration":1}' ] || die "didn't timeout properly"

kill -9 "$pid"
