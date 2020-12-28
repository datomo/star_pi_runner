class Trigger:
    callbacks = []
    module = ""

    def __init__(self, block: dict):
        self.module = block["options"]["module"]

    def add_callback(self, func):
        self.callbacks.append(func)

    def __str__(self):
        return "[ Action: Type: {}]".format(self.module)

    def __repr__(self):
        return "[ Action: Type: {}]".format(self.module)


class Button(Trigger):
    pass
