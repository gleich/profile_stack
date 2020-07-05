import os
import subprocess


def main():
    os.chdir('/')
    print(subprocess.check_output(['ls']))


if __name__ == "__main__":
    main()
