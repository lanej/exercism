class Zipper:
    @staticmethod
    def from_tree(tree):
        return Zipper(tree)

    def __init__(self, tree, parent=None):
        self.tree = tree
        self.parent = parent

    def value(self):
        return self.tree['value']

    def set_value(self):
        pass

    def left(self):
        node = self.tree['left']
        if node == None:
            return None

        return Zipper(node, parent=self.tree)

    def set_left(self, node):
        self.tree['left'] = node
        return Zipper(self.parent)

    def right(self):
        node = self.tree['right']
        if node == None:
            return None

        return Zipper(node, parent=self.tree)

    def set_right(self, node):
        self.tree['right'] = node
        return Zipper(self.parent)

    def up(self):
        if self.parent == None:
            return None
        return Zipper(self.parent)

    def to_tree(self):
        return self.tree
