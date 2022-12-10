from cv2 import split

class Node:
    parent = None
    path = '/'
    length = 0
    children = []

    def __init__(self, parent, path, length, children):
        self.parent = parent
        self.path = path
        self.length = int(length)
        self.children = children

    def find_by_name(self, name):
        for l in self.children:
            if (l.path == name):
                return l
        raise Exception('no folder')

    def add_child(self, node):
        self.children.append(node)

    def __str__(self) -> str:
        lengg = None
        if self.children != None:
            lengg = len(self.children)
        return f'{self.path} (size: {self.length}, children: {lengg})'

    def traverse(self, size=0, debug=False, sum=[]):
        if self.children == None:
            return 
        
        for child in self.children:
            if child.children != None:                
                sum_, children_length = child.traverse(size + 1, debug, sum)
                child.length = children_length
                sum = sum_
                if debug:
                    strr = (size+1) * '='
                    print(f'{strr}{child}')
            else:
                if debug:
                    strr = (size+1) * '-'
                    print(f'{strr}{child}')
            self.length += child.length

        if self.length >= 30000000 - (70000000 - 40913445):
            sum.append(self.length)
        return sum, self.length
    

root = Node(None, '/', 0, [])

file = open('input7.txt', 'r')
lines = file.readlines()

curr_node = root
curr_path = ''

for index, line in enumerate(lines):
    if index == 0:
        continue
    curr_line = line.strip()
    splitted = curr_line.split(' ')
        
    if (curr_line[0] == '$'):
        
        if (splitted[1] == 'ls'):
            continue
        else:

            curr_path = splitted[2]
            if (curr_path == '..'):
                curr_node = curr_node.parent
            else:
                curr_node = curr_node.find_by_name(splitted[2])

    elif (curr_line[0:3] == 'dir'):
        curr_node.add_child(Node(curr_node, splitted[1], 0, []))    

    else:
        curr_node.add_child(Node(curr_node, splitted[1], splitted[0], None))

sum, _ = root.traverse(debug=True, sum=[])
print(root.length)
print(min(sum))
file.close()