#!/bin/bash
function chaos {
    time cargo run params/$1.json viewer/$1/
}

chaos helix
chaos sierpinski
chaos tree
chaos dragon
chaos random_walk
chaos grid_3d
