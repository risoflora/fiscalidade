#!/bin/sh

set -e

curl -sSL https://raw.githubusercontent.com/Samuel-Oliveira/Java_NFe/master/src/main/resources/WebServicesNfe.ini |
    sed '/;/d; s/\(.*\)=\(.*\)/"\1" = "\2"/g; N;/^\n$/D;P;D;' >webservices.toml
