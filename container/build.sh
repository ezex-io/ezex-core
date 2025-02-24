#!/bin/sh

## Builder docker
docker build -f ./container/builder.Dockerfile -t builder .

## Service dockers
docker build -f ./container/deposit.Dockerfile -t deposit-img .
