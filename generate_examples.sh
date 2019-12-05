#!/bin/bash
function chaos {
    time cargo run params/$1.json viewer/$2/ $3
}

chaos helix helix 100000
chaos sierpinski sierpinski 1000000
chaos tree tree_10k 10000
chaos tree tree_1m 1000000
chaos dragon dragon 1000000
chaos random_walk random_walk 1000000
