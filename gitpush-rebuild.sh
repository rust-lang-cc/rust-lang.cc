#!/bin/bash
cp Dockerfile.rebuild Dockerfile
git add . && git commit -m "update" && git push