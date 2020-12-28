class Block:
    callbacks = []
    args = []
    module = ""
    type = ""

    def __init__(self, block: dict):
        self.module = block["options"]["module"]
        self.type = block["options"]["type"]

    def add_callback(self, func, args=None):
        self.callbacks.append(func)
        self.args.append(args)

    def __str__(self):
        return "[ {} - type: {} ]".format(self.type, self.module)

    def __repr__(self):
        return str(self)

    @staticmethod
    def build(block: dict):
        type = block["options"]["type"]
        if type == "action":
            return Action.build(block)
        if type == "trigger":
            return Trigger.build(block)


class Trigger(Block):

    def __init__(self, block: dict):
        super().__init__(block)

    @staticmethod
    def build(block: dict):
        if block["options"]["module"] == "button":
            return Button(block)


class Button(Trigger):

    def __init__(self, block: dict):
        super().__init__(block)


class Action(Block):

    def __init__(self, block: dict):
        super().__init__(block)

    def run(self):
        pass

    @staticmethod
    def build(block: dict):
        if block["options"]["module"] == "motor":
            return Motor(block)


class Motor(Action):
    pass
