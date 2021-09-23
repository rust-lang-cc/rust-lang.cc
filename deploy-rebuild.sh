#!/bin/bash
cp Dockerfile.rebuild Dockerfile
flyctl deploy --remote-only

