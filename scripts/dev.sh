#!/usr/bin/env bash
if ! [ -x "$(command -v bunyan)" ]; then
  echo >&2 "Warning: bunyan is not installed."
  echo >&2 "Use:"
  echo >&2 " cargo install bunyan"
  echo >&2 "to install it."
  echo >&2 ""
  cargo watch -x check -x test -x run
fi

clear && cargo watch -x check -x test -x run | bunyan