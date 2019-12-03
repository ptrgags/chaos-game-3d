#!/bin/bash
function chaos {
    time cargo run params/$1.json viewer/$2/ $3
}

chaos sierpinski sierpinski 1000000
chaos tree tree_10k 10000
chaos tree tree_1m 1000000
