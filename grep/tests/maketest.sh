#!/bin/bash 


function make_string {
    let n=0
    final_string=""
    while [ $n -lt 100 ]
    do
        final_string+=$RANDOM
        n=$(echo $n + 1 | bc)
    done
    echo $final_string;
}


let i=0
while [ $i -lt 10 ]
do
    echo $i
    make_string > testfile$(echo $i).txt
    i=$(echo $i + 1 | bc)
    
done
