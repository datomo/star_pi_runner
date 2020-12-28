from Block import Block


class Configurator:
    blocks = {}

    def __init__(self, blueprint: dict):
        self.generate_blocks(blueprint["blocks"].values())

    def generate_blocks(self, blocks: []):
        for block in blocks:

            self.blocks[block["id"]] = Block.build(block)

        print(self.blocks)
