#!/bin/bash

echo "[START] process";

mkdir "./exercices";
mkdir "./solutions";

for i in $(seq 1 3)
do
    touch "./exercices/if${i}.rs";
    touch "./solutions/if${i}.rs";
done

echo "[END] process";
