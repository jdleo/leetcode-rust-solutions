import os

# get all the folders underneath /solutions
for directory in os.listdir("./solutions/src"):
    # check if starts with problem_
    if directory.startswith("problem_"):
        # append to lib.rs with mod directory minus the .rs;
        with open("./solutions/src/lib.rs", "a") as f:
            f.write(f"mod {directory.replace('.rs', '')};\n")