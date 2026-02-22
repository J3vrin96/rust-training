#!/bin/bash

for i in $(seq 1 6)
do
    touch "./src/primitive_types/primitive_types${i}.rs";
done

echo "DONE"