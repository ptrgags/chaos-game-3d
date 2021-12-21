#!/bin/bash
set -e

function chaos {
    time cargo run params/$1.json viewer/$1/
}

chaos helix
chaos sierpinski
chaos dragon
chaos random_walk
chaos grid_3d
chaos zigzag
