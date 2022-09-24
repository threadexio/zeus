#!/bin/bash
set -e

git describe --tags --always --dirty --broken
