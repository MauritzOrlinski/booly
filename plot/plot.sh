#!/usr/bin/env bash

echo "compiling the program..."
cargo build -r 1>/dev/null 2>&1
echo "done!"

BIN=.././target/release/main

rm output.csv

find .././inputs -type f | while IFS= read -r f; do
  filename=$(basename "$f")

  echo "$filename"...

  output=$(timeout 1s $BIN "$f" | grep "^t")

  if [ $? -eq 0 ]; then
    tp=$(echo "$output" | grep "^tp " | awk '{print $2}')
    t=$(echo "$output" | grep "^t " | awk '{print $2}')
    time=$(awk "BEGIN {printf \"%.7f\", $t + $tp}")
    echo "$filename;$tp;$t;$time" >>output.csv

    echo "done"
  else
    echo "timeout"
  fi
done

sort -t ';' -k4 -n "output.csv" | awk 'BEGIN{count=1} {print $0 ";" count; count++}' >tmp.csv

accumulated_time=0
awk 'BEGIN {FS=";"} {
  accumulated_time += $4;
  print $0 ";" accumulated_time;
}' tmp.csv >tmp2.csv

rm tmp.csv

echo "file;preprocess;solve;time;accumulated_time;count" | cat - tmp2.csv >output.csv
rm tmp2.csv

gnuplot <<EOF
set output 'plot.png'
set terminal pngcairo

set datafile separator ";"
set xlabel "Count"
set ylabel "CPU Time (s)"
set title "Cactus Plot"

set datafile separator ";"

plot "output.csv" using 5:6 with linespoints title "Solver Performance"

set output
EOF
