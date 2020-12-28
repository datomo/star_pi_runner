from Action import Action
from Trigger import Trigger

class Configurator:
    blocks = {}

    def __init__(self, blueprint: dict):
        self.generate_blocks(blueprint["blocks"].values())

    def generate_blocks(self, blocks: []):
        for block in blocks:
            type = block["options"]["type"]
            if type == "action":
                self.generate_action(block)
            if type == "trigger":
                self.generate_trigger(block)

        print(self.blocks)

    def generate_action(self, block: dict):
        self.blocks[block["id"]] = Action(block)

    def generate_trigger(self, block: dict):
        self.blocks[block["id"]] = Trigger(block)
