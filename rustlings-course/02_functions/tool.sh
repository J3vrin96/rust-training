#!/bin/bash

rm -rf './exercices' && rm -rf './solutions'

mkdir './exercices'
mkdir './solutions'

for i in $(seq 1 5) 
do
    touch "./exercices/functions${i}.rs"
    touch "./solutions/functions${i}.rs"
done

echo "Files created"