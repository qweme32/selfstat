from glob import glob
from os import mkdir, system
from platform import system as platform


__version__ = "1.0.0"


def main():
    if "Windows" in platform():
        system("cls")
    else:
        system("clear")

    print("Selfstat builder v" + __version__)
    print("With <3 by @qweme32\n")

    url = input("Url: ")
    out = input("Out: ")

    try:
        mkdir(out)
    except:
        print("Out dir already exists, overwriting...")

    print("\nBuilding...")

    for path in glob("template/*"):
        with open(path, "r", encoding="utf-8") as file:
            data = file.read()

        if "Windows" in platform():
            new_path = path.replace("template\\", "")
        else:
            new_path = path.replace("template/", "")           

        data = data.replace("%url%", url)

        with open(out + "/" + new_path, "w", encoding="utf-8") as file:
            file.write(data)

        print(path, "->", out + "/" + new_path)

    print("\nSuccess")


if __name__ == "__main__":
    exit(main())