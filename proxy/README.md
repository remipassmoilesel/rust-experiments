# Proxy

Yet another simple HTTP proxy.


## Installation

On Ubuntu 18.04:

    $ sudo apt install gcc-multilib
    $ cargo build


## Try it

    $ curl -X PUT localhost:7878/public
    $ curl -X GET localhost:7878/public/_search
    
    $ curl -H 'X-Authorization: abcdef' -X PUT localhost:7878/with_authorization/_search
    $ curl -H 'X-Authorization: abcdef' -X PUT localhost:7878/with_authorization/_search


## TODO

- Replace all String by &str
- Remove all unwrap calls

