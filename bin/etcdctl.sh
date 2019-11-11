#!/bin/sh

docker exec -it -e ETCDCTL_API=3 etcd etcdctl $*
