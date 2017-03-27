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
[ "$(curl -sS "$SUMOTIME_URL")" = '{"msg":"sumotime","key":"foo.bar","code":1,"timeout":false,"duration":0}' ] || die "didn't POST failure properly"

sumotime -k 'foo.bar' -t 1 sleep 3 && die "expected timeout"
[ "$(curl -sS "$SUMOTIME_URL")" = '{"msg":"sumotime","key":"foo.bar","code":124,"timeout":true,"duration":1}' ] || die "didn't POST timeout properly"

f=$(mktemp)
( echo 'hello world' | sumotime -k 'foo.bar' cat > "$f" ) || die "expected success"
[ "$(cat "$f")" = "hello world" ] || die "expected STDIN, STDOUT to passthru"
[ "$(curl -sS "$SUMOTIME_URL")" = '{"msg":"sumotime","key":"foo.bar","code":0,"timeout":false,"duration":0}' ] || die "didn't POST success properly"

kill -9 "$pid"
