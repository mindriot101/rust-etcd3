# Etcd V3 in Rust

Uses bleeding edge version of tokio to support async/await.

## Features

### KV
* [x] get ranges
* [x] put value at key
* [x] delete range
* [ ] start transaction
* [ ] compact event history

### Watch
* [x] watch events

### Lease
* [ ] grant a lease
* [ ] revoke a lease
* [ ] keep a lease alive
* [ ] get a lease ttl
* [ ] list existing leases

### Cluster
* [ ] add a member to the cluster
* [ ] remove a member from the cluster
* [ ] update member configuration
* [x] list members in the cluster
* [ ] promote a member to raft voting member

### Maintenance
* [ ] activates, deactivaes and queries alarms
* [ ] get status of a cluster member
* [ ] defragment a cluster member's database
* [ ] hash a backend keyspace
* [ ] compute the hash of all MVCC keys
* [ ] send a snapshot to another client
* [ ] move leadership to another member

### Auth
* [ ] enable authentication
* [ ] disable authentication
* [ ] authenticate
* [ ] add user
* [ ] get user
* [ ] list user
* [ ] delete user
* [ ] change user password
* [ ] grant role to user
* [ ] revoke role from user
* [ ] add role
* [ ] get role
* [ ] list role
* [ ] delete role
* [ ] grant permission to role
* [ ] revoke permission from role


## Alternatives

* [etcd](https://crates.io/crates/etcd) - does not support V2
