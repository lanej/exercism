class Garden:
    STUDENTS = [
        "Alice",
        "Bob",
        "Charlie",
        "David",
        "Eve",
        "Fred",
        "Ginny",
        "Harriet",
        "Ileana",
        "Joseph",
        "Kincaid",
        "Larry",
    ]

    PLANTS = {
        'C': 'Clover',
        'R': 'Radishes',
        'G': 'Grass',
        'V': 'Violets',
    }

    def __init__(self, diagram: str, students=STUDENTS):
        self.diagram = []
        self.students = sorted(students)

        for line in diagram.split("\n"):
            self.diagram.append([c for c in line])

    def plants(self, name) -> list:
        plants = []
        start_index = self.students.index(name) * 2
        indexes = [start_index, start_index + 1]

        for row in self.diagram:
            [plants.append(row[index]) for index in indexes]

        return [self.PLANTS.get(plant) for plant in plants]
