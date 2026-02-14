#!/bin/bash 

for i in $(seq 1 6)
do 
    touch "./exercices/variables${i}.rs"
    touch "./solutions/variables${i}.rs"
done