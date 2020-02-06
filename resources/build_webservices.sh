#!/bin/sh

set -e

curl -sSL https://raw.githubusercontent.com/Samuel-Oliveira/Java_NFe/master/src/main/resources/WebServicesNfe.ini |
    sed '/;/d; s/\[\(.*\)]/\L\1:/; s/\(.*\)=/ \L\1: /g; s/usar:\(.*\)/usar:\L\1/g' >webservices.yaml
