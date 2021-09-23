#!/bin/bash
cp Dockerfile.no-rebuild Dockerfile
git add . && git commit -m "update" && git push