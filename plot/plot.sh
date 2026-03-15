#!/usr/bin/env bash

echo "compiling the program..."
cargo build -r 1>/dev/null 2>&1
echo "done!"

BIN=.././target/release/main

generate_data() {
  rm -f data/"$1"

  find .././inputs -type f | while IFS= read -r f; do
    filename=$(basename "$f")

    echo "$filename"...

    output=$(timeout 60s $BIN "${@:2}" "$f" | grep "^t")

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

generate_data "luby.csv"
generate_data "never.csv" "--restart-heuristic" "never"
generate_data "fi.csv" "--restart-heuristic" "fixed-interval"
generate_data "geo.csv" "--restart-heuristic" "geometric"
generate_data "ps_luby.csv" "--phase-saving"
generate_data "ps_never.csv" "--restart-heuristic" "never" "--phase-saving"
generate_data "ps_fi.csv" "--restart-heuristic" "fixed-interval" "--phase-saving"
generate_data "ps_geo.csv" "--restart-heuristic" "geometric" "--phase-saving"
generate_data "ps_nopre.csv" "--disable-preprocess"
generate_data "np_luby.csv" "--disable-preprocess"
generate_data "np_never.csv" "--restart-heuristic" "never" "--disable-preprocess"
generate_data "np_fi.csv" "--restart-heuristic" "fixed-interval" "--disable-preprocess"
generate_data "np_geo.csv" "--restart-heuristic" "geometric" "--disable-preprocess"
generate_data "np_ps_luby.csv" "--phase-saving" "--disable-preprocess"
generate_data "np_ps_never.csv" "--restart-heuristic" "never" "--phase-saving" "--disable-preprocess"
generate_data "np_ps_fi.csv" "--restart-heuristic" "fixed-interval" "--phase-saving" "--disable-preprocess"
generate_data "np_ps_geo.csv" "--restart-heuristic" "geometric" "--phase-saving" "--disable-preprocess"
generate_data "np_ps_nopre.csv" "--disable-preprocess" "--disable-preprocess"
