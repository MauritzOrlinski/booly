#!/usr/bin/env bash

echo "compiling the program..."
cargo build -r 1>/dev/null 2>&1
echo "done!"

BIN=.././target/release/main

rm output.csv

find .././inputs -type f | while IFS= read -r f; do
  filename=$(basename "$f")

  echo "$filename"...

  output=$(timeout 60s $BIN "$f" | grep "^t")

  if [ $? -eq 0 ]; then
    t=$(echo "$output" | grep "^t " | awk '{print $2}')
    echo "$filename;$t" >>output.csv

    echo "done"
  else
    echo "timeout"
  fi
done

sort -t ';' -k2 -n "output.csv" | awk 'BEGIN{count=1} {print $0 ";" count; count++}' >tmp.csv

accumulated_time=0
awk 'BEGIN {FS=";"} {
  accumulated_time += $2;
  print $0 ";" accumulated_time;
}' tmp.csv >tmp2.csv

rm tmp.csv

echo "file;time;accumulated_time;count" | cat - tmp2.csv >output.csv
rm tmp2.csv
