import os
import hashlib
import json
import subprocess

cc = ["clang++", "-Iinclude"]
build = "build/"

hashes: dict

try:
  with open(os.path.join(build, "hashes.json"), 'r') as f:
    hashes = json.load(f)
except:
  hashes = dict()


def hashed(name) -> str:
  hasher = hashlib.sha256()
  with open(name, 'rb') as f:
    for chunk in iter(lambda: f.read(8192), b""):
        hasher.update(chunk)
  return hasher.hexdigest()


for (dirpath, dirnames, filenames) in os.walk("src/"):
  for file in filenames:  
    if not file.endswith('.cpp'): continue
    path = os.path.join(dirpath, file)
    hash_val = hashed(path)

    if path in hashes:
      if hashes[path] == hash_val: 
        print("skip: ", path)
        continue
    
    print("recompile:", path)
    hashes[path] = hash_val

    out_file = file[:-3] + "o"
    out_file = dirpath[4:].replace("/", "_") + out_file
    output = os.path.join(build, out_file)
    cmd: list[str] = cc.copy()
    cmd.extend(["-c", path, "-o", output])
    subprocess.run(cmd)


output_command: list[str] = cc.copy()
output_command.extend(['-o', 'kette.bin'])
for (dirpath, dirnames, filenames) in os.walk(build):
  for file in filenames: 
    if not file.endswith('.o'): continue
    output_file = os.path.join(dirpath, file)
    output_command.append(output_file)
  break

subprocess.run(output_command)

with open(os.path.join(build, "hashes.json"), 'w') as f:
  json_data = json.dumps(hashes, indent=2)
  f.write(json_data)
