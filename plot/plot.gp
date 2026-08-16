set output 'plot.png'
set terminal pngcairo

set datafile separator ";"
set xlabel "Count"
set ylabel "CPU Time (s)"
set title "Cactus Plot"

set datafile separator ";"

plot "data/np_luby.csv" using 3:4 with linespoints title "no preprocess", \

set output
