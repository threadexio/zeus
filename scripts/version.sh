#!/bin/sh
set -e

git describe --tags --always --dirty --broken
