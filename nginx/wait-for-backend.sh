#!/bin/sh

set -e

until curl --silent http://backend:8080/hey; do
    echo >&2 "Backend is unavailable - sleeping"
    sleep 1
done

echo >&2 "Backend is up - executing command"
exec nginx -g 'daemon off;'