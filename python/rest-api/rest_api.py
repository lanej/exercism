import json


class RestAPI:
    def __init__(self, database=None):
        self.database = database
        self.repo = Repository(database)

    def get(self, url, payload=None):
        response = None
        body = {}
        if payload:
            body = json.loads(payload)

        if url == "/users":
            users = self.repo.get_users(only=body.get('users', None))
            response = {'users': users}
        else:
            raise ValueError(url, 'not found')
        return json.dumps(response)

    def post(self, url, payload) -> str:
        response = None
        body = json.loads(payload)

        if url == "/add":
            username = body["user"]

            user = self.repo.add_user(username)

            return json.dumps(user)
        elif url == "/iou":
            lender_name = body['lender']
            borrower_name = body['borrower']

            self.repo.borrow(lender_name, borrower_name, float(body['amount']))

            return json.dumps({'users': self.repo.get_users(only=[lender_name, borrower_name])})
        else:
            raise ValueError(url, 'not found')

        return json.dumps(response)


class Repository:
    def __init__(self, database: dict):
        self.database = database

    def get_users(self, only=None):
        users = self.database['users']
        if only != None:
            users = list(filter(lambda record: record['name'] in only, users))
        return users

    def add_user(self, username: str):
        user = {"name": username, "owes": {}, "owed_by": {}, "balance": 0.0}
        self.database['users'].append(user)
        return user

    def get_user(self, username: str):
        return next(filter(lambda user: user['name'] == username, self.database['users']))

    def borrow(self, lender_name: str, borrower_name: str, amount: float):
        """
        transfer money between users
        """

        lender = self.get_user(lender_name)
        borrower = self.get_user(borrower_name)

        self.move(lender, borrower_name, amount, deposit=True)
        self.move(borrower, lender_name, amount, deposit=False)

    def move(self, from_user: dict, to_username: str, amount: float, deposit=True):
        if deposit:
            from_user['balance'] += amount
        else:
            from_user['balance'] -= amount

        remaining_amount = amount

        if deposit:
            from_ledger, to_ledger = 'owes', 'owed_by'
        else:
            from_ledger, to_ledger = 'owed_by', 'owes'

        if to_username in from_user[from_ledger]:
            owed_amount = from_user[from_ledger][to_username]
            if owed_amount < amount:
                from_user[from_ledger].pop(to_username)
                remaining_amount = amount - owed_amount
            else:
                from_user[from_ledger][to_username] -= amount
                remaining_amount = 0.0

        if remaining_amount > 0.0:
            from_user[to_ledger].setdefault(to_username, 0.0)
            from_user[to_ledger][to_username] += remaining_amount

        if from_user[from_ledger].get(to_username, None) == 0.0:
            from_user[from_ledger].pop(to_username)
