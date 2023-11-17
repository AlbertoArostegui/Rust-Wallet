FROM node:21.1.0 as builder

COPY . .

RUN npm install

RUN npm run build

FROM httpd

COPY --from=builder ./dist /usr/local/apache2/htdocs