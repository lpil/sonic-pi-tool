#!/usr/bin/env bash
set -euo pipefail

# Replication of crash detailed here:
# https://github.com/lpil/sonic-pi-tool/issues/12

cargo run eval "
  live_loop :boom do
    4.times do
      sample :loop_industrial

      sleep sample_duration(:loop_industrial)
    end
  end
"

sleep 1

cargo run eval "
  live_loop :boom do
    4.times do
      sample :loop_industrial, onset: 'uh oh'

      sleep sample_duration(:loop_industrial)
    end
  end
"
