#!/bin/bash
cp Dockerfile.no-rebuild Dockerfile
flyctl deploy --remote-only