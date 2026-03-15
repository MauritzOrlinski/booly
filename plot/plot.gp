set output 'plot.png'
set terminal pngcairo

set datafile separator ";"
set xlabel "Count"
set ylabel "CPU Time (s)"
set title "Cactus Plot"

set datafile separator ";"

plot "data/pre.csv" using 3:2 with linespoints title "preprocess", \
     "data/nopre.csv" using 3:2 with linespoints title "no preprocess"

set output
