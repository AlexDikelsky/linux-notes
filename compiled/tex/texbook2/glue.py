class Glue():
    def __init__(self, _space, _stretch, _shrink):
        self.space = _space
        self.stretch = _stretch
        self.shrink = _shrink

    def __str__(self):
        print(

class Box():
    def __init__(self, _width):
        self.width = width

class Line():
    def __init__(self):
        self.line = []
        #THis is a list of Boxes and Glue
    
    def get_glue_values(self, newlen):
        for x in [x for x in self.line if type(x) == type(Glue(0,0,0))]:
            print(x)

a = Line()
a.line.append(Glue(1, 2, 3)


