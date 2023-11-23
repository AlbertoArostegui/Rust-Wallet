FROM postgres:16.0-alpine3.18

ENV POSTGRES_USER postgres
ENV POSTGRES_PASSWORD postgres
ENV POSTGRES_DB users

ADD up.sql /docker-entrypoint-initdb.d/