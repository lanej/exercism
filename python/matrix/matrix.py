class Matrix:
    def __init__(self, matrix_string):
        self.data = []

        for row_string in matrix_string.splitlines():
            row = []
            for col_string in row_string.split(" "):
                row.append(int(col_string))
            self.data.append(row)

    def row(self, index):
        return self.data[index - 1]

    def column(self, index):
        column_data = []
        for row in self.data:
            column_data.append(row[index - 1])
        return column_data
