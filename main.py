import json
from Configurator import Configurator


def main():
    # load file into blueprint class
    with open("config/config.json") as f:
        blueprint = f.read()
        blueprint = json.loads(blueprint)

    # send to configurator
    configurator = Configurator(blueprint)

    # configurator builds blocks from blueprint
    # builds workflow with block according to blueprint


if __name__ == '__main__':
    main()
