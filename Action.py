class Action:
    callbacks = []
    module = ""

    def __init__(self, block: dict):
        self.module = block["options"]["module"]

    def __str__(self):
        return "[ Action: Type: {}]".format(self.module)

    def __repr__(self):
        return "[ Action: Type: {}]".format(self.module)


class Motor(Action):
    pass
