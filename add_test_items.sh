#!/bin/bash

set -e

for i in `seq 1 100`; do 
    curl -H 'Content-Type: application/json' -d "{\"id\":\"$(uuidgen)\", \"tags\":{\"_description\":\"Test Item $i\"}}" localhost:8080/api/items
done