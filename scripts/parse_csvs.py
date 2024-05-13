import os
import pandas as pd


def convert_csv_files(directory):
    for filename in os.listdir(directory):
        if filename.endswith(".csv"):
            file_path = os.path.join(directory, "wsn_simulation", "assets", "data", filename)
            df = pd.read_csv(file_path, delim_whitespace=True, header=None)
            df.to_csv(file_path, index=False, header=None)


if __name__ == "__main__":
    current_directory = os.path.dirname(os.path.realpath(__file__))
    convert_csv_files(current_directory)
