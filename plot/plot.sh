#!/usr/bin/env bash

echo "compiling the program..."
cargo build -r 1>/dev/null 2>&1
echo "done!"

BIN=.././target/release/main

generate_data() {
  rm data/"$1"

  find .././inputs -type f | while IFS= read -r f; do
    filename=$(basename "$f")

    echo "$filename"...

    if [ -z "$2" ]; then
      output=$(timeout 60s $BIN "$f" | grep "^t")
    else
      output=$(timeout 60s $BIN "-d" "$f" | grep "^t")
    fi

    if [ $? -eq 0 ]; then
      t=$(echo "$output" | grep "^t " | awk '{print $2}')
      echo "$filename;$t" >>data/"$1"

      echo "done"
    else
      echo "timeout"
    fi
  done

  sort -t ';' -k2 -n data/"$1" | awk 'BEGIN{count=1} {print $0 ";" count; count++}' >tmp.csv

  accumulated_time=0
  awk 'BEGIN { FS=";" } { accumulated_time += $2; print $0 ";" accumulated_time; }' tmp.csv >data/"$1"

  rm tmp.csv
}

generate_data "pre.csv"
generate_data "nopre.csv" 0
