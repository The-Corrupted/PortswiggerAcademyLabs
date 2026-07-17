import json

passwords = []
with open("$HOME/Documents/BurpNotesAndLabs/password.txt") as f:
    passwords = [x.strip() for x in f.readlines()]

print(json.dumps(passwords))
