#!/bin/bash
set -e

if [[ -d /tmp/.ssh ]]; then
  mkdir -p /root
  cp -R /tmp/.ssh /root/.ssh
  chmod 700 /root/.ssh
  chmod 600 /root/.ssh/*
  chmod 644 /root/.ssh/*.pub
  chmod 644 /root/.ssh/known_hosts
fi

exec "$@"
