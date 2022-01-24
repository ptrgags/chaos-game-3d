import os
import json

def get_tileset_paths():
    paths = []
    with os.scandir() as it:
        for entry in it:
            if entry.is_dir():
                tileset_path = os.path.join(entry.name, "tileset.json")
                if os.path.exists(tileset_path):
                    paths.append(tileset_path)
    return paths

def extract_fractal_info(tileset_json):
    try:
        tileset = tileset_json["extensions"]["3DTILES_metadata"]["tileset"]
        properties = tileset["properties"]

        # We only need the id, name and description for populating the UI
        return {
            "id": properties["id"],
            "name": properties["name"],
            "description": properties.get("description", "")
        }
    except KeyError:
        return None

def get_fractal_infos(tileset_paths):
    info = []
    for tileset_path in tileset_paths:
        with open(tileset_path, "r") as f:
            tileset_json = json.load(f)
            fractal_info = extract_fractal_info(tileset_json)
            if fractal_info is not None:
                info.append(fractal_info)
    return info


def main():
    paths = get_tileset_paths()
    fractal_info = get_fractal_infos(paths)
    index_json = {
        "fractals": fractal_info
    }
    with open("fractals.json", "w") as f:
        json.dump(index_json, f)

if __name__ == "__main__":
    main()