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

    def generate_action(self, block: dict):
        pass

    def generate_trigger(self, block: dict):
        pass
