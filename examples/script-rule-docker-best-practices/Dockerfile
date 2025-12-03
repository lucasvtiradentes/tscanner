FROM node:latest

RUN apt-get update
RUN apt-get install curl

ADD https://example.com/file.tar.gz /tmp/

COPY ../config /app/config

CMD ["node", "app.js"]
