# Ping

A libptp example program that has two nodes pinging each other.

In one terminal start a node:
```
wink@3900x:~/prgs/rust/myrepos/expr-libp2p/ping (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `/home/wink/prgs/rust/myrepos/expr-libp2p/target/debug/ping`
Local peer id: PeerId("12D3KooWRbwBdqkzHJ2FhkR9xS63zLatNZKV9AF2PF4UATjCR9Ks")
transport: BoxedTransport
behaviour created
swarm created
swarm listening on /ip4/0.0.0.0/tcp/0
std::env::args len: 1
std::env::args Args { inner: ["/home/wink/prgs/rust/myrepos/expr-libp2p/target/debug/ping"] }
Looper waiting on some event
Listening on "/ip4/127.0.0.1/tcp/42907"
Looper waiting on some event
Listening on "/ip4/192.168.1.101/tcp/42907"
Looper waiting on some event
Listening on "/ip4/172.19.0.1/tcp/42907"
Looper waiting on some event
Listening on "/ip4/172.17.0.1/tcp/42907"
Looper waiting on some event
Listening on "/ip4/172.18.0.1/tcp/42907"
Looper waiting on some event
```

The node above is now listening on several interfaces, but we'll start
another node that will connect to the above node on "/ipv4/127.0.0.1/tcp/4290+y7"
in another terminal. And we see it "ConnectionEstablished" then a Pong and a Ping:
```
$ cargo run /ip4/127.0.0.1/tcp/42907
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `/home/wink/prgs/rust/myrepos/expr-libp2p/target/debug/ping /ip4/127.0.0.1/tcp/42907`
Local peer id: PeerId("12D3KooWA3YV5zuB3aLpq9V1nKNUFuPdFC3oB3b43ifHka4TExDW")
transport: BoxedTransport
behaviour created
swarm created
swarm listening on /ip4/0.0.0.0/tcp/0
std::env::args len: 2
std::env::args Args { inner: ["/home/wink/prgs/rust/myrepos/expr-libp2p/target/debug/ping", "/ip4/127.0.0.1/tcp/42907"] }
Dialed /ip4/127.0.0.1/tcp/42907
Looper waiting on some event
Listening on "/ip4/127.0.0.1/tcp/34015"
Looper waiting on some event
Listening on "/ip4/192.168.1.101/tcp/34015"
Looper waiting on some event
Listening on "/ip4/172.19.0.1/tcp/34015"
Looper waiting on some event
Listening on "/ip4/172.17.0.1/tcp/34015"
Looper waiting on some event
Listening on "/ip4/172.18.0.1/tcp/34015"
Looper waiting on some event
Ignoring swarm_event: ConnectionEstablished { peer_id: PeerId("12D3KooWRbwBdqkzHJ2FhkR9xS63zLatNZKV9AF2PF4UATjCR9Ks"), endpoint: Dialer { address: "/ip4/127.0.0.1/tcp/42907", role_override: Dialer }, num_established: 1, concurrent_dial_errors: Some([]) }
Looper waiting on some event
Behaviour event: Event { peer: PeerId("12D3KooWRbwBdqkzHJ2FhkR9xS63zLatNZKV9AF2PF4UATjCR9Ks"), result: Ok(Pong) }
Looper waiting on some event
Behaviour event: Event { peer: PeerId("12D3KooWRbwBdqkzHJ2FhkR9xS63zLatNZKV9AF2PF4UATjCR9Ks"), result: Ok(Ping { rtt: 435.27µs }) }
Looper waiting on some event
```

Back on the first node we see "IncomingConnection" followed by a "ConnectionEstablished" and then a Pong and Ping:
```
Ignoring swarm_event: IncomingConnection { local_addr: "/ip4/127.0.0.1/tcp/42907", send_back_addr: "/ip4/127.0.0.1/tcp/47526" }
Looper waiting on some event
Ignoring swarm_event: ConnectionEstablished { peer_id: PeerId("12D3KooWA3YV5zuB3aLpq9V1nKNUFuPdFC3oB3b43ifHka4TExDW"), endpoint: Listener { local_addr: "/ip4/127.0.0.1/tcp/42907", send_back_addr: "/ip4/127.0.0.1/tcp/47526" }, num_established: 1, concurrent_dial_errors: None }
Looper waiting on some event
Behaviour event: Event { peer: PeerId("12D3KooWA3YV5zuB3aLpq9V1nKNUFuPdFC3oB3b43ifHka4TExDW"), result: Ok(Pong) }
Looper waiting on some event
Behaviour event: Event { peer: PeerId("12D3KooWA3YV5zuB3aLpq9V1nKNUFuPdFC3oB3b43ifHka4TExDW"), result: Ok(Ping { rtt: 475.5µs }) }
Looper waiting on some event
```

Type Ctrl-C in the two termainals to kill the two nodes.
